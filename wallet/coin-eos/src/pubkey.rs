use bitcoin::util::base58;
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::{hash160, Hash};
use common::apdu::EosApdu;
use common::error::Error;
use common::path;
use mq::message;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, Zero};
use std::str::FromStr;

#[derive(Debug)]
pub struct EosPubkey {}

impl EosPubkey {
    pub fn get_pubkey(path: &str) -> Result<String, Error> {
        path::check_path_validity(path);

        let select_apdu = EosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu);
        //todo: check select response

        //get public
        let msg_pubkey = EosApdu::get_pubkey(&path, true);
        let res_msg_pubkey = message::send_apdu(msg_pubkey);
        let uncomprs_pubkey: String = res_msg_pubkey
            .chars()
            .take(res_msg_pubkey.len() - 4)
            .collect();
        let comprs_pubkey = EosPubkey::cal_comprs_pubkey(&uncomprs_pubkey);
        //        let pub_key_hash = hash160::Hash::hash(&pub_key_bytes).into_inner();
        let pubkey_hash = hash160::Hash::from_hex(&comprs_pubkey)
            .unwrap()
            .into_inner();
        //        let pubkey_hash = hex::encode(&pubkey_hash);
        //        let check_sum:String = pubkey_hash.chars().take(4).collect();
        //        let pk_with_checksum = comprs_pubkey + &check_sum;
        let pk_base58 = base58::check_encode_slice(&pubkey_hash);
        Ok(pk_base58)
    }

    pub fn cal_comprs_pubkey(uncomprs_pubkey: &str) -> String {
        let x = &uncomprs_pubkey[2..=66];
        let y = &uncomprs_pubkey[66..130];
        let y_bint = BigInt::from_str(&y).unwrap();
        let two_bint = BigInt::from_i64(2).unwrap();

        let (_d, m) = y_bint.div_mod_floor(&two_bint);
        if m.is_zero() {
            return "02".to_owned() + x;
        } else {
            return "03".to_owned() + x;
        }
    }

    pub fn display_pubkey(path: &str) -> Result<String, Error> {
        let pubkey = EosPubkey::get_pubkey(path).unwrap();
        let reg_apdu = EosApdu::register_pubkey(pubkey.as_bytes());
        let res_reg = message::send_apdu(reg_apdu);
        //todo: check response
        Ok(res_reg)
    }
}
