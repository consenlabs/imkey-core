use bitcoin::util::base58;
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::{ripemd160, Hash};
use common::apdu::EosApdu;
use common::error::Error;
use common::path;
use mq::message;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, Zero, Num};
use std::str::FromStr;

#[derive(Debug)]
pub struct EosPubkey {}

impl EosPubkey {
    pub fn get_pubkey(path: &str) -> Result<String, Error> {
        path::check_path_validity(path);

        let select_apdu = EosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu);
        //todo: check select response

        //get public key
        let msg_pubkey = EosApdu::get_pubkey(&path, true);
        let res_msg_pubkey = message::send_apdu(msg_pubkey);

        //compressed key
        let uncomprs_pubkey: String = res_msg_pubkey
            .chars()
            .take(res_msg_pubkey.len() - 4)
            .collect();
        let comprs_pubkey = EosPubkey::cal_comprs_pubkey(&uncomprs_pubkey);

        //checksum base58
        let mut comprs_pubkey_slice = hex::decode(comprs_pubkey).expect("Decoding failed");
        let pubkey_hash = ripemd160::Hash::hash(&comprs_pubkey_slice);
        let check_sum = &pubkey_hash[0..4];
        comprs_pubkey_slice.extend(check_sum);
        let eos_pk = "EOS".to_owned() + base58::encode_slice(&comprs_pubkey_slice).as_ref();
        Ok(eos_pk)
    }

    pub fn cal_comprs_pubkey(uncomprs_pubkey: &str) -> String {
        let x = &uncomprs_pubkey[2..66];
        let y = &uncomprs_pubkey[66..130];
//        let y_bint = BigInt::from_str(&y).unwrap();
        let y_bint = BigInt::from_str_radix(&y,16).unwrap();
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
        Ok(pubkey)
    }
}

#[cfg(test)]
mod tests {
    use crate::pubkey::EosPubkey;
    use common::constants;

    #[test]
    fn test_get_pubkey() {
        let pubkey = EosPubkey::get_pubkey(constants::EOS_PATH);
        println!("pubkey:{}",pubkey.unwrap());
    }

    #[test]
    fn test_display_pubkey() {
        let pubkey = EosPubkey::display_pubkey(constants::EOS_PATH);
        println!("pubkey:{}",pubkey.unwrap());
    }
}