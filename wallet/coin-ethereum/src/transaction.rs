use common::error::Error;
use common::utility::hex_to_bytes;
use dotenv::dotenv;
use ethereum_types::{Address, H256, U256};
use keccak_hash::keccak;
use lazy_static::lazy_static;
use rlp::{self, DecoderError, Encodable, Rlp, RlpStream};
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::{self, Message as SecpMessage, Secp256k1};
use std::env;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
    static ref ETHPRVKEY: Vec<u8> = eth_get_prvkey();
    static ref SIGNOPTION: String = sign_option();
    static ref ETHKEYID: u16 = eth_sign_keyid();
}

pub fn eth_get_prvkey() -> Vec<u8> {
    dotenv().ok();
    let ethereum_private_key =
        hex_to_bytes(&env::var("ETHEREUM_PRIVATE_KEY").expect("ETHEREUM_PRIVATE_KEY must be set"))
            .expect("ETHEREUM_PRIVATE_KEY must be valid hexadecimal string");
    ethereum_private_key
}

pub fn eth_sign_keyid() -> u16 {
    dotenv().ok();
    let eth_keyid = env::var("ETH_SIGN_KEYID")
        .unwrap_or("3".to_owned())
        .parse::<u16>()
        .unwrap();
    eth_keyid
}

pub fn sign_option() -> String {
    dotenv().ok();
    let sign_option = env::var("SIGN_OPTION").unwrap_or("SOFTWARE".to_string());
    sign_option
}

/// Transaction action type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Create creates new contract.
    Create,
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(Address),
}

impl Default for Action {
    fn default() -> Action {
        Action::Create
    }
}

impl rlp::Decodable for Action {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            if rlp.is_data() {
                Ok(Action::Create)
            } else {
                Err(DecoderError::RlpExpectedToBeData)
            }
        } else {
            Ok(Action::Call(rlp.as_val()?))
        }
    }
}

impl rlp::Encodable for Action {
    fn rlp_append(&self, s: &mut RlpStream) {
        match *self {
            Action::Create => s.append_internal(&""),
            Action::Call(ref addr) => s.append_internal(addr),
        };
    }
}

pub struct Signature([u8; 65]);

impl Signature {
    /// Get a slice into the 'r' portion of the data.
    pub fn r(&self) -> &[u8] {
        &self.0[0..32]
    }

    /// Get a slice into the 's' portion of the data.
    pub fn s(&self) -> &[u8] {
        &self.0[32..64]
    }

    /// Get the recovery byte.
    pub fn v(&self) -> u8 {
        self.0[64]
    }
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
    pub fn eth_sign(
        &self,
        chain_id: Option<u64>,
    ) -> Result<(Vec<u8>, UnverifiedTransaction), Error> {
        let prvkey = SecretKey::from_slice(&ETHPRVKEY).map_err(|_err| Error::PrvKeyError)?;
        let sig = self.sign_hash(&prvkey, &self.hash(chain_id))?;
        Ok(self.with_signature(sig, chain_id))
    }

    /// The message hash of the transaction.
    pub fn hash(&self, chain_id: Option<u64>) -> H256 {
        let mut stream = RlpStream::new();
        self.rlp_append_unsigned_transaction(&mut stream, chain_id);
        keccak(stream.as_raw())
    }

    pub fn sign_hash(&self, prvkey: &SecretKey, message: &H256) -> Result<Signature, Error> {
        let context = &SECP256K1;
        let s = context.sign_recoverable(
            &SecpMessage::from_slice(&message[..]).map_err(|_err| Error::MessageError)?,
            &prvkey,
        );
        let (rec_id, data) = s.serialize_compact();
        let mut data_arr = [0; 65];

        // no need to check if s is low, it always is
        data_arr[0..64].copy_from_slice(&data[0..64]);
        data_arr[64] = rec_id.to_i32() as u8;
        Ok(Signature(data_arr))
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

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::{Address, H160, U256};
    use hex;
    use rustc_hex::{FromHex, ToHex};
    use serde;
    use std::str::FromStr;

    #[test]
    fn test_eth_sign() {
        /*
        let tx = Transaction {
          nonce: U256::from(9),
          gas_price: U256::from(20000000000 as usize),
          gas_limit: U256::from(21000),
          //to: Some(Address::from_str("3535353535353535353535353535353535353535").unwrap()),
          //to: Some("3535353535353535353535353535353535353535".to_string()),
          to: Action::Call(Address::from_str("3535353535353535353535353535353535353535").unwrap()),
          value: U256::from(1000000000000000000 as usize),
          data: Vec::new(),
        };

        let pk = SecretKey::from_str("4646464646464646464646464646464646464646464646464646464646464646").unwrap();
        let signetx = tx.eth_sign(Some(1));
        let mut args = [0u8; 32];
        signetx.unwrap().1.r.to_big_endian(&mut args[0..32]);
        let testhex = hex::encode(args);
        let testhex2 = args.to_vec();

        assert_eq!(2 + 2, 4);
        */

        env::set_var(
            "ETHEREUM_PRIVATE_KEY",
            "4646464646464646464646464646464646464646464646464646464646464646",
        );
        let tx = Transaction {
            nonce: U256::from(9),
            gas_price: U256::from(20000000000 as usize),
            gas_limit: U256::from(21000),
            to: Action::Call(
                Address::from_str("3535353535353535353535353535353535353535").unwrap(),
            ),
            value: U256::from(1000000000000000000 as usize),
            data: Vec::new(),
        };

        let signedtx = tx.eth_sign(Some(1)).unwrap();
        let mut args = [0u8; 32];
        signedtx.clone().1.r.to_big_endian(&mut args[0..32]);
        let r_hex = hex::encode(args);
        signedtx.clone().1.s.to_big_endian(&mut args[0..32]);
        let s_hex = hex::encode(args);
        assert_eq!(
            r_hex,
            "18ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276"
        );
        assert_eq!(
            s_hex,
            "67cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83"
        );
        let nonesense = 0;
    }
}
