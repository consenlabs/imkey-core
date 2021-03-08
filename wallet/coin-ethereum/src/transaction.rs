use crate::address::EthAddress;
use crate::ethapi::{EthMessageInput, EthMessageOutput, EthTxOutput};
use crate::types::{Action, Signature};
use crate::Result as EthResult;
use common::apdu::{ApduCheck, CoinCommonApdu, EthApdu};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::{hex_to_bytes, is_valid_hex, secp256k1_sign};
use common::{constants, utility, SignParam};
use device::device_binding::KEY_MANAGER;
use ethereum_types::{H256, U256};
use keccak_hash::keccak;
use lazy_static::lazy_static;
use rlp::{self, DecoderError, Encodable, Rlp, RlpStream};
use secp256k1::recovery::{RecoverableSignature, RecoveryId};
use secp256k1::{self, Message as SecpMessage, Signature as SecpSignature};
use transport::message::{send_apdu, send_apdu_timeout};

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
    ) -> EthResult<EthTxOutput> {
        // ) {
        //check path
        check_path_validity(path)?;

        //organize data
        let mut data_pack: Vec<u8> = Vec::new();
        let encode_tx = self.rlp_encode_tx(chain_id);

        //rlp encoded tx in TLV format
        data_pack.extend(
            [
                1,
                ((encode_tx.len() & 0xFF00) >> 8) as u8,
                (encode_tx.len() & 0x00FF) as u8,
            ]
            .iter(),
        );
        data_pack.extend(encode_tx.iter());
        //payment info in TLV format
        data_pack.extend([7, payment.as_bytes().len() as u8].iter());
        data_pack.extend(payment.as_bytes().iter());
        //receiver info in TLV format
        data_pack.extend([8, receiver.as_bytes().len() as u8].iter());
        data_pack.extend(receiver.as_bytes().iter());
        //fee info in TLV format
        data_pack.extend([9, fee.as_bytes().len() as u8].iter());
        data_pack.extend(fee.as_bytes().iter());

        let key_manager_obj = KEY_MANAGER.lock();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, &data_pack).unwrap();

        let mut apdu_pack: Vec<u8> = Vec::new();
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.extend(data_pack.as_slice());

        //select applet
        let select_apdu = EthApdu::select_applet();
        let select_result = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_result)?;

        //prepare apdu
        let msg_prepare = EthApdu::prepare_sign(apdu_pack);
        for msg in msg_prepare {
            let res = send_apdu_timeout(msg, constants::TIMEOUT_LONG)?;
            ApduCheck::check_response(&res)?;
        }

        //get public
        let msg_pubkey = EthApdu::get_xpub(path, false);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        ApduCheck::check_response(&res_msg_pubkey)?;

        let pubkey_raw = hex_to_bytes(&res_msg_pubkey[..130]).unwrap();

        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone()).unwrap();
        let address_checksummed = EthAddress::address_checksummed(&address_main);
        //compare address
        if address_checksummed != *sender {
            return Err(CoinError::ImkeyAddressMismatchWithPath.into());
        }
        //sign
        let msg_sign = EthApdu::sign_digest(path);
        let res_msg_sign = send_apdu(msg_sign)?;
        ApduCheck::check_response(&res_msg_sign)?;

        let sign_compact = &res_msg_sign[2..130];
        let sign_compact_vec = hex_to_bytes(sign_compact).unwrap(); //todo error

        let mut signature_obj = SecpSignature::from_compact(sign_compact_vec.as_slice()).unwrap();
        signature_obj.normalize_s();
        let normalizes_sig_vec = signature_obj.serialize_compact();

        let msg_hash = self.hash(chain_id);

        let rec_id =
            utility::retrieve_recid(&msg_hash[..], &normalizes_sig_vec, &pubkey_raw).unwrap();

        let mut data_arr = [0; 65];
        data_arr[0..64].copy_from_slice(&normalizes_sig_vec[0..64]);
        data_arr[64] = rec_id.to_i32() as u8;
        let sig = Signature(data_arr);

        let signed = self.with_signature(sig, chain_id);

        let mut tx_hash = hex::encode(signed.1.hash);
        if !tx_hash.starts_with("0x") {
            tx_hash.insert_str(0, "0x");
        }

        let tx_sign_result = EthTxOutput {
            signature: hex::encode(signed.0),
            tx_hash,
        };

        Ok(tx_sign_result)
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

    pub fn sign_message(
        input: EthMessageInput,
        sign_param: &SignParam,
    ) -> EthResult<EthMessageOutput> {
        check_path_validity(&sign_param.path)?;

        let message_to_sign;
        if is_valid_hex(&input.message) {
            let value = &input.message[2..];
            message_to_sign = hex::decode(value).unwrap();
        } else {
            message_to_sign = input.message.into_bytes();
        }

        let mut data = Vec::new();
        if input.is_personal_sign {
            let header = format!("Ethereum Signed Message:\n{}", &message_to_sign.len());
            data.extend(header.as_bytes());
        }
        data.extend(message_to_sign);

        let mut data_to_sign: Vec<u8> = Vec::new();
        data_to_sign.push(0x01);
        data_to_sign.push(((data.len() & 0xFF00) >> 8) as u8);
        data_to_sign.push((data.len() & 0x00FF) as u8);
        data_to_sign.extend(data.as_slice());

        let key_manager_obj = KEY_MANAGER.lock();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, &data_to_sign)?;

        let mut apdu_pack: Vec<u8> = vec![];
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.extend(data_to_sign.as_slice());

        let select_apdu = EthApdu::select_applet();
        let select_result = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_result)?;

        let msg_pubkey = EthApdu::get_xpub(&sign_param.path, false);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        let pubkey_raw = hex_to_bytes(&res_msg_pubkey[..130]).unwrap();
        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone()).unwrap();
        let address_checksummed = EthAddress::address_checksummed(&address_main);

        if &address_checksummed != &sign_param.sender {
            return Err(CoinError::ImkeyAddressMismatchWithPath.into());
        }

        let prepare_apdus = EthApdu::prepare_personal_sign(apdu_pack);
        for apdu in prepare_apdus {
            let res = send_apdu_timeout(apdu, constants::TIMEOUT_LONG)?;
            ApduCheck::check_response(&res)?;
        }

        let sign_apdu = EthApdu::personal_sign(&sign_param.path);
        let sign_response = send_apdu(sign_apdu)?;
        ApduCheck::check_response(&sign_response)?;

        let sign_compact = hex::decode(&sign_response[2..130]).unwrap();
        let mut signature_obj = SecpSignature::from_compact(sign_compact.as_slice()).unwrap();
        signature_obj.normalize_s();
        let normalizes_sig_vec = signature_obj.serialize_compact();

        let data_hash = tiny_keccak::keccak256(&data);
        let rec_id = utility::retrieve_recid(&data_hash, &normalizes_sig_vec, &pubkey_raw).unwrap();
        let rec_id = rec_id.to_i32();
        let v = rec_id + 27;

        let mut signature = hex::encode(&normalizes_sig_vec.as_ref());
        signature.push_str(&format!("{:02x}", &v));

        Ok(EthMessageOutput { signature })
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
        println!("hash:{}", &hex::encode(&hash));
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

#[cfg(test)]
mod tests {
    use super::*;
    use common::constants;
    use device::device_binding::bind_test;
    use ethereum_types::{Address, U256};
    use hex;
    use std::str::FromStr;

    #[test]
    fn test_sign_trans() {
        bind_test();

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

        let tx_result = tx
            .sign(Some(28), &path, &payment, &receiver, &sender, &fee)
            .unwrap();
        assert_eq!(
            tx_result.signature,
            "f867088504a817c8088302e248943535353535353535353535353535353535353535820200805ba03aa62abb45b77418caf139dda0179aea802c99967b3d690b87d586a87bc805afa02b5ce94f40dc865ca63403e0e5e723e1523884f001573677cd8cec11c7ca332f".to_string()
        );
        assert_eq!(
            tx_result.tx_hash,
            "0x09fa41c4d6b92482506c8c56f65b217cc3398821caec7695683110997426db01".to_string()
        );
    }

    #[test]
    fn test_data_is_null() {
        bind_test();

        let tx = Transaction {
            nonce: U256::from_dec_str("13").unwrap(),
            gas_price: U256::from_dec_str("150000").unwrap(),
            gas_limit: U256::from_dec_str("21000000000").unwrap(),
            to: Action::Call(
                Address::from_str("7c47ef93268a311f4cad0c750724299e9b72c268").unwrap(),
            ),
            value: U256::from_dec_str("10000000000000000").unwrap(),
            data: Vec::new(),
        };
        let path = "m/44'/60'/0'/0/0".to_string();
        let payment = "0.01 ETH".to_string();
        let receiver = "0x7c47ef93268a311f4cad0c750724299e9b72c268".to_string();
        let sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string();
        let fee = "0.0032 ether".to_string();

        let tx_result = tx
            .sign(Some(28), &path, &payment, &receiver, &sender, &fee)
            .unwrap();
        assert_eq!(
            tx_result.tx_hash,
            "0x9cb10bab794454c5c2606b5475a35f6429f5ff54c3e088d0c5d330f56155b0be".to_string()
        );
    }

    #[test]
    fn test_data_is_long() {
        bind_test();

        let data = "0x60056013565b6101918061001d6000396000f35b3360008190555056006001600060e060020a6000350480630a874df61461003a57806341c0e1b514610058578063a02b161e14610066578063dbbdf0831461007757005b610045600435610149565b80600160a060020a031660005260206000f35b610060610161565b60006000f35b6100716004356100d4565b60006000f35b61008560043560243561008b565b60006000f35b600054600160a060020a031632600160a060020a031614156100ac576100b1565b6100d0565b8060018360005260205260406000208190555081600060005260206000a15b5050565b600054600160a060020a031633600160a060020a031614158015610118575033600160a060020a0316600182600052602052604060002054600160a060020a031614155b61012157610126565b610146565b600060018260005260205260406000208190555080600060005260206000a15b50565b60006001826000526020526040600020549050919050565b600054600160a060020a031633600160a060020a0316146101815761018f565b600054600160a060020a0316ff5b56".to_string();
        let data_vec;
        if data.starts_with("0x") {
            // data = hex::encode(&data[2..]);
            data_vec = hex::decode(&data[2..]).unwrap();
        } else {
            data_vec = hex::decode(&data).unwrap();
        }

        let tx = Transaction {
            nonce: U256::from_dec_str("13").unwrap(),
            gas_price: U256::from_dec_str("150000").unwrap(),
            gas_limit: U256::from_dec_str("21000000000").unwrap(),
            to: Action::Call(
                Address::from_str("7c47ef93268a311f4cad0c750724299e9b72c268").unwrap(),
            ),
            value: U256::from_dec_str("10000000000000000").unwrap(),
            data: Vec::from(data_vec.as_slice()),
        };
        let path = "m/44'/60'/0'/0/0".to_string();
        let payment = "0.01 ETH".to_string();
        let receiver = "0x7c47ef93268a311f4cad0c750724299e9b72c268".to_string();
        let sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string();
        let fee = "0.0032 ether".to_string();

        let tx_result = tx
            .sign(Some(28), &path, &payment, &receiver, &sender, &fee)
            .unwrap();
        assert_eq!(
            tx_result.tx_hash,
            "0xff0c83a7c9208ea28712900cabc8cd5fe624b9c6bdc208517b6725c706422e08".to_string()
        );
    }

    #[test]
    fn test_zero_bytes() {
        bind_test();

        let data = "0x0000000000000000000000000000000000000000000000000000000000".to_string();
        let data_vec;
        if data.starts_with("0x") {
            // data = hex::encode(&data[2..]);
            data_vec = hex::decode(&data[2..]).unwrap();
        } else {
            data_vec = hex::decode(&data).unwrap();
        }

        let tx = Transaction {
            nonce: U256::from_dec_str("13").unwrap(),
            gas_price: U256::from_dec_str("150000").unwrap(),
            gas_limit: U256::from_dec_str("21000000000").unwrap(),
            to: Action::Call(
                Address::from_str("7c47ef93268a311f4cad0c750724299e9b72c268").unwrap(),
            ),
            value: U256::from_dec_str("10000000000000000").unwrap(),
            data: Vec::from(data_vec.as_slice()),
        };
        let path = "m/44'/60'/0'/0/0".to_string();
        let payment = "0.01 ETH".to_string();
        let receiver = "0x7c47ef93268a311f4cad0c750724299e9b72c268".to_string();
        let sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string();
        let fee = "0.0032 ether".to_string();

        let tx_result = tx
            .sign(Some(28), &path, &payment, &receiver, &sender, &fee)
            .unwrap();
        assert_eq!(
            tx_result.tx_hash,
            "0x5481b9f73cb42eb2be84c4a3995ec1ea2fafc93597f564fe46b40d82026c4224".to_string()
        );
    }

    #[test]
    fn test_sign_personal_message() {
        bind_test();

        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path: constants::ETH_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "".to_string(),
        };
        let input = EthMessageInput {
            message: "Hello imKey".to_string(),
            is_personal_sign: true,
        };
        let output = Transaction::sign_message(input, &sign_param).unwrap();
        assert_eq!(
            output.signature,
            "d928f76ad80d63003c189b095078d94ae068dc2f18a5cafd97b3a630d7bc47465bd6f1e74de2e88c05b271e1c5a8b93564d9d8842c207482b20634d68f2d54e51b".to_string()
        );

        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path: constants::ETH_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "".to_string(),
        };
        let input = EthMessageInput {
            message: "0x8d61d40bb0761526fe24d84199321d5e9f6542e56c52018c401b963d64ef21678c18563a3eba889229ab078a8a1baed22226913f".to_string(),
            is_personal_sign: true
        };

        let output = Transaction::sign_message(input, &sign_param).unwrap();
        assert_eq!(
            output.signature,
            "35a94616ce12ddb79f6d351c2644c0fa2f496bd152b17102a5672359f583373b6dd5d2a60f5d9909cf84e6af7dc40176179c819a7cbd9b199f4c2e868530293f1b".to_string()
        );
    }

    #[test]
    fn test_ec_sign() {
        bind_test();

        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path: constants::ETH_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "".to_string(),
        };
        let input = EthMessageInput {
            message: "Hello imKey".to_string(),
            is_personal_sign: false,
        };
        let output = Transaction::sign_message(input, &sign_param).unwrap();
        assert_eq!(
            output.signature,
            "57c976d1fa15c7e833fd340bcb3a96974060ed555369d443449ac4429c1933433afa5304d1cfcb6799403f2b97a1e83309b98fae8ad5fade62335664d90e819f1b".to_string()
        );

        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path: constants::ETH_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "".to_string(),
        };
        let input = EthMessageInput {
            message: "0x8d61d40bb0761526fe24d84199321d5e9f6542e56c52018c401b963d64ef21678c18563a3eba889229ab078a8a1baed22226913f".to_string(),
            is_personal_sign: false
        };
        let output = Transaction::sign_message(input, &sign_param).unwrap();
        assert_eq!(
            output.signature,
            "3d8ba5e7375900476d715b479938e48a2e46e59f8e2e12673adb5e3df78a622050053ae0183f5e555e5db34ff43293de255f384709bd3fe6e00b8239c7f1a3561c".to_string()
        );
    }

    #[test]
    fn test_retrieve_recid() {
        let hash = "123faa96160f0b89a758c4f8585500d0ab6559565e184a02882c8b3cda20263d";
        let sign = "397828f985a5d19546fe59425d44c745c72152eac845e54fd748b457ba306c682582567be75888645d623225af599cc0ae9f285f8d0d020e7c9a9246985b4dda";
        let pubkey = "04aaf80e479aac0813b17950c390a16438b307aee9a814689d6706be4fb4a4e30a4d2a7f75ef43344fa80580b5b1fbf9f233c378d99d5adb5cac9ae86f562803e1";

        let rec_id = utility::retrieve_recid(
            &hex::decode(hash).unwrap(),
            &&hex::decode(sign).unwrap(),
            &&hex::decode(pubkey).unwrap(),
        )
        .unwrap();
        let rec_id = rec_id.to_i32();
        assert_eq!(rec_id, 0);
    }

    #[test]
    fn test_address_mismatch() {
        bind_test();

        let tx = Transaction {
            nonce: U256::from_dec_str("13").unwrap(),
            gas_price: U256::from_dec_str("150000").unwrap(),
            gas_limit: U256::from_dec_str("21000000000").unwrap(),
            to: Action::Call(
                Address::from_str("7c47ef93268a311f4cad0c750724299e9b72c268").unwrap(),
            ),
            value: U256::from_dec_str("10000000000000000").unwrap(),
            data: Vec::new(),
        };
        let path = "m/44'/60'/0'/0/1".to_string();
        let payment = "0.01 ETH".to_string();
        let receiver = "0x7c47ef93268a311f4cad0c750724299e9b72c268".to_string();
        let sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string();
        let fee = "0.0032 ether".to_string();

        let tx_result = tx.sign(Some(28), &path, &payment, &receiver, &sender, &fee);
        assert_eq!(
            format!("{}", tx_result.err().unwrap()),
            "imkey_address_mismatch_with_path"
        );

        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path,
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "".to_string(),
        };
        let input = EthMessageInput {
            message: "Hello imKey".to_string(),
            is_personal_sign: true,
        };
        let output = Transaction::sign_message(input, &sign_param);
        assert_eq!(
            format!("{}", output.err().unwrap()),
            "imkey_address_mismatch_with_path"
        );
    }
}
