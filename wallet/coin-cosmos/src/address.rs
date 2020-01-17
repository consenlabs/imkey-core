use common::error::Error;
use hex;
use common::apdu::CosmosApdu;
use mq::message;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, Zero, Num};
use std::str::FromStr;
use common::constants;
use common::path;
use hex::FromHex;
use secp256k1::{Secp256k1, Message, Signature, PublicKey as PublicKey2, SecretKey};
use ring::digest;
use bech32::ToBase32;

#[derive(Debug)]
pub struct CosmosAddress {}

impl CosmosAddress {
    pub fn get_address(path: &str) -> Result<String, Error> {
        path::check_path_validity(path);

        let select_apdu = CosmosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu);
        //todo: check select response

        //get public
        let msg_pubkey = CosmosApdu::get_pubkey(&path, true);
        let res_msg_pubkey = message::send_apdu(msg_pubkey);

        let data:String = res_msg_pubkey.chars().take(97*2).collect();
        let signature:String = res_msg_pubkey.chars().skip(97*2).take(res_msg_pubkey.len()-4-97*2).collect();


//        let uncomprs_pubkey: String = res_msg_pubkey
//            .chars()
//            .take(res_msg_pubkey.len() - 4)
//            .collect();
//        let comprs_pubkey = cal_comprs_pubkey(&uncomprs_pubkey);
//        let pubkey_hash = hash160::Hash::from_hex(&comprs_pubkey)
//            .unwrap()
//            .into_inner();
//        let pk_base58 = base58::check_encode_slice(&pubkey_hash);
//        Ok(pk_base58)

//        let secp = Secp256k1::new();
//        let se_pub_key = "04E03248A0012603C6B20786C2A86EB6B9DC1767BC56674EBE471ED5FDF287A063985885E0523E100319E0643810F0EAF66A0D4102AEAE49FD7BC7AC232247A3DC";
//        let se_pub_key_obj = PublicKey2::from_str(se_pub_key).unwrap();
//
//        let message_hash = digest::digest(
//            &digest::SHA256,
//            Vec::from_hex(data).unwrap().as_slice(),
//        );
//        let message_obj = Message::from_slice(message_hash.as_ref()).unwrap();
//        let sig_data = Vec::from_hex(signature).unwrap().as_slice();
//        //生成签名结果对象
//        let mut sig = Signature::from_der(sig_data).unwrap();
//        sig.normalize_s();
//        let verify_result = secp.verify(&message_obj, &sig, &se_pub_key_obj).is_ok();
//        if !verify_result {
//            return Err(Error::MessageError);
//        }

        let uncomprs_pubkey: String = res_msg_pubkey
            .chars()
            .take(res_msg_pubkey.len() - 4)
            .collect();
        let comprs_pubkey = CosmosAddress::cal_comprs_pubkey(&uncomprs_pubkey);

//        let mut buf = vec![];
//        buf.extend(vec![0x1, 0x00]); // append short version for locks with popular codehash and default code hash index
//        buf.extend(Vec::from_hex(comprs_pubkey).unwrap());
        let buf = Vec::from_hex(comprs_pubkey).unwrap();

        let prefix = "cosmos";
        Ok(bech32::encode(prefix, buf.to_base32()).unwrap())
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

    pub fn display_address(path: &str) -> Result<String, Error> {
        let pubkey = CosmosAddress::get_address(path).unwrap();
        let reg_apdu = CosmosApdu::register_pubkey(pubkey.as_bytes());
        let res_reg = message::send_apdu(reg_apdu);
        //todo: check response
        Ok(res_reg)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::CosmosAddress;
    use common::constants;

    #[test]
    fn test_get_address() {
        let address = CosmosAddress::get_address(constants::COSMOS_PATH);
        println!("address:{}",address.unwrap());
    }
}