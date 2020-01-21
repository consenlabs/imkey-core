use bitcoin::util::base58;
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::{ripemd160, Hash};
use common::apdu::EosApdu;
use common::error::Error;
use common::{path, utility};
use mq::message;
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

        let sign_source_val = &res_msg_pubkey[..194];
        let sign_result = &res_msg_pubkey[194..];
        let pub_key = &sign_source_val[..130];

        //use se public key verify sign
//        let se_pub_key = "04E03248A0012603C6B20786C2A86EB6B9DC1767BC56674EBE471ED5FDF287A063985885E0523E100319E0643810F0EAF66A0D4102AEAE49FD7BC7AC232247A3DC";
//        let sign_verify_result = utility::secp256k1_sign_verify(hex::decode(se_pub_key).unwrap().as_slice(),
//                                                                hex::decode(sign_result).unwrap().as_slice(),
//                                                                hex::decode(sign_source_val).unwrap().as_slice());
//        if !sign_verify_result {
//            return Err(Error::MessageError);
//        }

        //compressed key
        let uncomprs_pubkey: String = res_msg_pubkey
            .chars()
            .take(res_msg_pubkey.len() - 4)
            .collect();
        let comprs_pubkey = utility::uncompress_pubkey_2_compress(&uncomprs_pubkey);

        //checksum base58
        let mut comprs_pubkey_slice = hex::decode(comprs_pubkey).expect("Decoding failed");
        let pubkey_hash = ripemd160::Hash::hash(&comprs_pubkey_slice);
        let check_sum = &pubkey_hash[0..4];
        comprs_pubkey_slice.extend(check_sum);
        let eos_pk = "EOS".to_owned() + base58::encode_slice(&comprs_pubkey_slice).as_ref();

        Ok(eos_pk)
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