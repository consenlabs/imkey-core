use crate::address::EthAddress;
use crate::types::{Action, Signature};
use bitcoin::hashes::{sha256d, Hash};
use common::apdu::EthApdu;
use common::error::Error;
use common::path::check_path_validity;
use common::utility::{hex_to_bytes, sha256_hash, secp256k1_sign_hash, secp256k1_sign};
use ethereum_types::{Address, H256, U256};
use keccak_hash::keccak;
use lazy_static::lazy_static;
use mq::message::send_apdu;
use rlp::{self, DecoderError, Encodable, Rlp, RlpStream};
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::recovery::{RecoverableSignature, RecoveryId};
use secp256k1::{self, Message as SecpMessage, Secp256k1};
use common::ethapi::{EthPersonalSignInput, EthPersonalSignOutput};
use common::utility;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub to: Action,
    pub value: U256,
    pub data: Vec<u8>,
}

impl Transaction {
    /// Signs the transaction as coming from `sender`.
    pub fn sign(
        &self,
        chain_id: Option<u64>,
        path: &str,
        payment: &str,
        receiver: &str,
        sender: &str,
        fee: &str,
    ) -> Result<(Vec<u8>, UnverifiedTransaction), Error> {
        //check path
        check_path_validity(path);
        //select applet
        let select_apdu = EthApdu::select_applet();
        let select_result = send_apdu(select_apdu);

        //organize data
        let mut apdu_pack = Vec::new();
        let encode_tx = self.rlp_encode_tx(chain_id);
        //rlp encoded tx in TLV format
        apdu_pack.extend(
            [
                1,
                ((encode_tx.len() & 0xFF00) >> 8) as u8,
                (encode_tx.len() & 0x00FF) as u8,
            ]
            .iter(),
        );
        apdu_pack.extend(encode_tx.iter());
        //payment info in TLV format
        apdu_pack.extend([7, payment.as_bytes().len() as u8].iter());
        apdu_pack.extend(payment.as_bytes().iter());
        //receiver info in TLV format
        apdu_pack.extend([8, receiver.as_bytes().len() as u8].iter());
        apdu_pack.extend(receiver.as_bytes().iter());
        //fee info in TLV format
        apdu_pack.extend([9, fee.as_bytes().len() as u8].iter());
        apdu_pack.extend(fee.as_bytes().iter());

        let test_pack = hex::encode(apdu_pack.clone());
        println!("test is {}", test_pack);

        //hash data for verification sign
        let hash_data = sha256d::Hash::from_slice(&apdu_pack);

        //TODO: sign using private key, here need to bypass the checking in applet
        let mut signature = vec![0; 65];
        signature.insert(0, signature.len() as u8);
        signature.insert(0, 0);
        apdu_pack.splice(0..0, signature.iter().cloned()); //@@XM TODO: check this insertion

        //prepare apdu
        let msg_prepare = EthApdu::prepare_sign(apdu_pack);
        for msg in msg_prepare {
            let res = send_apdu(hex::encode(msg));
        }

        //get public
        let msg_pubkey = EthApdu::get_pubkey(path, false);
        let res_msg_pubkey = send_apdu(hex::encode(msg_pubkey));

        let pubkey_raw =
            hex_to_bytes(&res_msg_pubkey[2..130]).map_err(|_err| Error::PubKeyError)?;

        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;
        let address_checksummed = EthAddress::address_checksummed(&address_main);
        //compare address
        if address_checksummed != *sender {
            return Err(Error::AddressError);
        }
        //sign
        let msg_sign = EthApdu::sign_digest(path);
        let res_msg_sign = send_apdu(msg_sign);

        //handle sign result
        //let sign_res = String::from("mock for signature"); //@@XM TODO: replace with real result
        //let r = &sign_res[2..66];
        //let s = &sign_res[66..130];
        let sign_compact = &res_msg_sign[2..130];
        let sign_compact_vec = hex_to_bytes(sign_compact).map_err(|_err| Error::SignError)?;

        let msg_hash = self.hash(chain_id);
        let msg_to_sign =
            &SecpMessage::from_slice(&msg_hash[..]).map_err(|_err| Error::MessageError)?;

        let rec_id = retrieve_recid(msg_to_sign, &sign_compact_vec, &pubkey_raw)?;

        let mut data_arr = [0; 65];
        data_arr[0..64].copy_from_slice(&sign_compact_vec[0..64]);
        data_arr[64] = rec_id.to_i32() as u8;
        let sig = Signature(data_arr);

        Ok(self.with_signature(sig, chain_id))
    }

    pub fn rlp_encode_tx(&self, chain_id: Option<u64>) -> Vec<u8> {
        let mut stream = RlpStream::new();
        self.rlp_append_unsigned_transaction(&mut stream, chain_id);
        stream.as_raw().to_vec()
    }

    /// The message hash of the transaction.
    pub fn hash(&self, chain_id: Option<u64>) -> H256 {
        let mut stream = RlpStream::new();
        self.rlp_append_unsigned_transaction(&mut stream, chain_id);
        keccak(stream.as_raw())
    }

    pub fn rlp_append_unsigned_transaction(&self, s: &mut RlpStream, chain_id: Option<u64>) {
        s.begin_list(if chain_id.is_none() { 6 } else { 9 });
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.to);
        s.append(&self.value);
        s.append(&self.data);
        if let Some(n) = chain_id {
            s.append(&n);
            s.append(&0u8);
            s.append(&0u8);
        }
    }

    pub fn with_signature(
        &self,
        sig: Signature,
        chain_id: Option<u64>,
    ) -> (Vec<u8>, UnverifiedTransaction) {
        let unverified = UnverifiedTransaction {
            unsigned: self.clone(),
            r: sig.r().into(),
            s: sig.s().into(),
            v: self.add_chain_replay_protection(sig.v() as u64, chain_id),
            hash: H256::zero(),
        };
        (unverified.rlp_bytes(), unverified.compute_hash())
    }

    pub fn add_chain_replay_protection(&self, v: u64, chain_id: Option<u64>) -> u64 {
        v + if let Some(n) = chain_id {
            35 + n * 2
        } else {
            27
        }
    }

    pub fn sign_persional_message(input:EthPersonalSignInput) -> EthPersonalSignOutput{
//        let select_apdu = EthApdu::select_applet();
//        let select_result = send_apdu(select_apdu);
//        let message_vec = hex::decode(input.message).expect();
//        hex::decode()
        let header = format!("Ethereum Signed Message:\n{}", &input.message.as_bytes().len());
        println!("header:{}", &header);

        let mut data = Vec::new();
        data.extend(header.as_bytes());
        data.extend(input.message.as_bytes());
        println!("data:{}", &hex::encode(&data));

        let mut data_to_sign: Vec<u8>  = Vec::new();
        data_to_sign.push(0x01);
        data_to_sign.push(((data.len() & 0xFF00) >> 8) as u8);
        data_to_sign.push((data.len() & 0x00FF) as u8);
        data_to_sign.extend(data.as_slice());
        println!("data_to_sign:{}", &hex::encode(&data_to_sign));

        let private_key = hex_to_bytes("15A3C9A55EAE204B1CC8F2DBA25AE9A4F35793D7226E9CDE8731D58D43D6C72C").unwrap();//ios
        let mut bind_signature = secp256k1_sign(&private_key, &data_to_sign);
        println!("bind_signature:{}", &hex::encode(&bind_signature));

        let mut apdu_pack: Vec<u8>  = Vec::new();
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.extend(data_to_sign.as_slice());
        println!("apdu_pack:{}", &hex::encode(&apdu_pack));

        let select_apdu = EthApdu::select_applet();
        let select_result = send_apdu(select_apdu);

        let msg_pubkey = EthApdu::get_pubkey(&input.path, false);
        let res_msg_pubkey = send_apdu(msg_pubkey);
        let pubkey_raw = hex_to_bytes(&res_msg_pubkey[2..130]).unwrap();
        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone()).unwrap();
        println!("address_main:{}", &hex::encode(&address_main));

        //todo check address
        if &address_main == &input.sender {
        }

        let prepare_apdus = EthApdu::prepare_personal_sign(apdu_pack);
        for apdu in prepare_apdus {
            println!("prepare apdu:{}", &apdu);
            send_apdu(apdu);//todo check response
        }

        let sign_apdu = EthApdu::personal_sign(&input.path);
        let sign_response = send_apdu(sign_apdu);

        let r = &sign_response[2..66];
        let s = &sign_response[66..130];

        //calc v
//        let pub_key_raw = hex::decode(&pubkey_raw).unwrap();
        let sign_compact = hex::decode(&sign_response[2..130]).unwrap();
        let data_hash = tiny_keccak::keccak256(&data);
        let rec_id = utility::retrieve_recid(data_hash.as_ref(), &sign_compact, &pubkey_raw).unwrap();
        let rec_id = rec_id.to_i32();
        println!("rec_id:{}", &rec_id);
        let v = rec_id + 27 + 4;

        let mut signature = "".to_string();
        signature.push_str(&format!("{:02X}", &v));
        signature.push_str(r);
        signature.push_str(s);
        println!("signature:{}", &signature);

        let output = EthPersonalSignOutput{
            signature
        };
        output
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnverifiedTransaction {
    /// Plain Transaction.
    unsigned: Transaction,
    /// The V field of the signature; the LS bit described which half of the curve our point falls
    /// in. The MS bits describe which chain this transaction is for. If 27/28, its for all chains.
    v: u64,
    /// The R field of the signature; helps describe the point on the curve.
    r: U256,
    /// The S field of the signature; helps describe the point on the curve.
    s: U256,
    /// Hash of the transaction
    pub hash: H256,
}

impl rlp::Decodable for UnverifiedTransaction {
    fn decode(d: &Rlp) -> Result<Self, DecoderError> {
        if d.item_count()? != 9 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        let hash = keccak(d.as_raw());
        Ok(UnverifiedTransaction {
            unsigned: Transaction {
                nonce: d.val_at(0)?,
                gas_price: d.val_at(1)?,
                gas_limit: d.val_at(2)?,
                to: d.val_at(3)?,
                value: d.val_at(4)?,
                data: d.val_at(5)?,
            },
            v: d.val_at(6)?,
            r: d.val_at(7)?,
            s: d.val_at(8)?,
            hash: hash,
        })
    }
}

impl rlp::Encodable for UnverifiedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.rlp_append_sealed_transaction(s)
    }
}

impl UnverifiedTransaction {
    /// Used to compute hash of created transactions
    fn compute_hash(mut self) -> UnverifiedTransaction {
        let hash = keccak(&*self.rlp_bytes());
        self.hash = hash;
        self
    }

    /// Append object with a signature into RLP stream
    fn rlp_append_sealed_transaction(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.unsigned.nonce);
        s.append(&self.unsigned.gas_price);
        s.append(&self.unsigned.gas_limit);
        s.append(&self.unsigned.to);
        s.append(&self.unsigned.value);
        s.append(&self.unsigned.data);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
    }
}

pub fn retrieve_recid(
    msg: &SecpMessage,
    sign_compact: &Vec<u8>,
    pubkey: &Vec<u8>,
) -> Result<RecoveryId, Error> {
    let secp_context = &SECP256K1;

    let mut recid_final = -1i32;
    for i in 0..4 {
        let rec_id = RecoveryId::from_i32(i as i32).unwrap();
        let sig = RecoverableSignature::from_compact(&sign_compact, rec_id)
            .map_err(|_err| Error::SignError)?;
        if let Ok(rec_pubkey) = secp_context.recover(&msg, &sig) {
            let rec_pubkey_raw = rec_pubkey.serialize_uncompressed();
            if rec_pubkey_raw[1..65].to_vec() == *pubkey {
                recid_final = i;
                break;
            }
        } else {
            continue;
        }
    }

    let rec_id = RecoveryId::from_i32(recid_final).map_err(|_err| Error::SignError);
    rec_id
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::{Address, H160, U256};
    use hex;
    use rustc_hex::{FromHex, ToHex};
    use serde;
    use std::str::FromStr;
    use common::constants;

    #[test]
    fn test_apdu_pack() {
        let tx = Transaction {
            nonce: U256::from(8),
            gas_price: U256::from(20000000008 as usize),
            gas_limit: U256::from(189000),
            to: Action::Call(
                Address::from_str("3535353535353535353535353535353535353535").unwrap(),
            ),
            value: U256::from(512 as usize),
            data: Vec::new(),
        };

        let path = "m/44'/60'/0'/0/0".to_string();
        let payment = "0.01 ETH".to_string();
        let receiver = "0xE6F4142dfFA574D1d9f18770BF73814df07931F3".to_string();
        let sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string();
        let fee = "0.0032 ether".to_string();

        let signedtx = tx.sign(Some(28), &path, &payment, &receiver, &sender, &fee);
        let nonesense = 0;

        //expected apdu_pack before sign using binding privekey is "010028E708850
        //4A817C8088302E2489435353535353535353535353535353535353535358202
        //00801C80800708302E303120455448082A30784536463431343264664641353
        //7344431643966313837373042463733383134646630373933314633090C302E30303332206574686572"
    }

    #[test]
    fn test_sign_personal_message(){
        let input = EthPersonalSignInput{
            path: constants::ETH_PATH.to_string(),
            message: "Hello imKey".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string()
        };
        let output = Transaction::sign_persional_message(input);
    }
}
