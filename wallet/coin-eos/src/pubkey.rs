use bitcoin::util::base58;
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::{ripemd160, Hash};
use common::apdu::EosApdu;
use common::{path, utility};
use mq::message;
use std::str::FromStr;
use crate::Result;
use device::device_binding::KEY_MANAGER;

#[derive(Debug)]
pub struct EosPubkey {}

impl EosPubkey {
    pub fn get_pubkey(path: &str) -> Result<String> {
        path::check_path_validity(path);

        let select_apdu = EosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu);
        //todo: check select response

        //get public key
        let msg_pubkey = EosApdu::get_pubkey(&path, true);
        let res_msg_pubkey = message::send_apdu(msg_pubkey);

        let sign_source_val = &res_msg_pubkey[..194];
        let sign_result = &res_msg_pubkey[194..res_msg_pubkey.len()-4];
        let pub_key = &sign_source_val[..130];

        let key_manager_obj = KEY_MANAGER.lock().unwrap();

        //use se public key verify sign
        // let se_pub_key = "04E03248A0012603C6B20786C2A86EB6B9DC1767BC56674EBE471ED5FDF287A063985885E0523E100319E0643810F0EAF66A0D4102AEAE49FD7BC7AC232247A3DC";
        let sign_verify_result = utility::secp256k1_sign_verify(&key_manager_obj.se_pub_key,
                                                                hex::decode(sign_result).unwrap().as_slice(),
                                                                hex::decode(sign_source_val).unwrap().as_slice())?;
        if !sign_verify_result {
            return Err(format_err!("imkey_signature_verify_fail"));
        }

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

    pub fn pubkey_from_response(response: &str) -> Result<String> {
        //compressed key
        let uncomprs_pubkey: String = response
            .chars()
            .take(response.len() - 4)
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
    pub fn display_pubkey(path: &str) -> Result<String> {
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
    use device::device_binding::DeviceManage;

    #[test]
    fn test_get_pubkey() {
        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let check_result = DeviceManage::bind_check(&path).unwrap_or_default();
        println!("check_result:{}",&check_result);

        let pubkey = EosPubkey::get_pubkey(constants::EOS_PATH);
        println!("pubkey:{}",pubkey.unwrap());
    }

    #[test]
    fn pubkey_from_response() {
        let response = "04AAF80E479AAC0813B17950C390A16438B307AEE9A814689D6706BE4FB4A4E30A4D2A7F75EF43344FA80580B5B1FBF9F233C378D99D5ADB5CAC9AE86F562803E13DC6BED90C9CE56BB58C24F200D64966E9553CCAAA731DD6B0B2C1C7708C55E53045022012B1393FAED0B88BD8FFC1333DC61F0D7FC862454339574A3A550D555F0ACCD2022100AF1C929FECB18F3226E0DB511731FA9D7016C23CB8E7AD30F5327B4CF681DD729000";
        let pubkey = EosPubkey::pubkey_from_response(response);
        println!("pubkey:{}",pubkey.unwrap());
    }

    #[test]
    fn test_display_pubkey() {
        let pubkey = EosPubkey::display_pubkey(constants::EOS_PATH);
        println!("pubkey:{}",pubkey.unwrap());
    }
}