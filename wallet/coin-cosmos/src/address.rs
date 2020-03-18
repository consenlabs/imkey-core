use common::apdu::{CosmosApdu, ApduCheck};
use common::constants;
use common::path;
use common::utility;
use hex;
use mq::message;
use std::str::FromStr;
//use hex::FromHex;
use bech32::bech32::Bech32;
use bech32::AddressError;
use bitcoin::bech32::convert_bits;
use bitcoin_hashes::hex::{FromHex, ToHex};
use bitcoin_hashes::{hash160, Hash};
use ring::digest;
use secp256k1::{Message, PublicKey as PublicKey2, Secp256k1, SecretKey, Signature};
use crate::Result;
use device::device_binding::KEY_MANAGER;

#[derive(Debug)]
pub struct CosmosAddress {}

impl CosmosAddress {
    pub fn get_pub_key(path: &str) -> Result<String> {
        path::check_path_validity(path);

        let select_apdu = CosmosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu);
        ApduCheck::checke_response(&select_response)?;

        //get public
        let msg_pubkey = CosmosApdu::get_pubkey(&path, true);
        let res_msg_pubkey = message::send_apdu(msg_pubkey);
        ApduCheck::checke_response(&res_msg_pubkey)?;

        let sign_source_val = &res_msg_pubkey[..194];
        let sign_result = &res_msg_pubkey[194..res_msg_pubkey.len() - 4];
        let pub_key = &sign_source_val[..130];

        //use se public key verify sign
        // let se_pub_key = "04E03248A0012603C6B20786C2A86EB6B9DC1767BC56674EBE471ED5FDF287A063985885E0523E100319E0643810F0EAF66A0D4102AEAE49FD7BC7AC232247A3DC";
        let key_manager_obj = KEY_MANAGER.lock().unwrap();

        let sign_verify_result = utility::secp256k1_sign_verify(
            &key_manager_obj.se_pub_key,
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(sign_source_val).unwrap().as_slice(),
        )?;
        if !sign_verify_result {
//            return Err(error::Error::AddressError);
            return Err(format_err!("imkey_signature_verify_fail"));
        }

        let uncomprs_pubkey: String = res_msg_pubkey
            .chars()
            .take(res_msg_pubkey.len() - 4)
            .collect();
        let comprs_pubkey = utility::uncompress_pubkey_2_compress(&uncomprs_pubkey);

        Ok(comprs_pubkey)
    }

    pub fn get_address(path: &str) -> Result<String> {
        let comprs_pubkey = CosmosAddress::get_pub_key(path).unwrap();
        //hash160
        let pub_key_bytes = hex::decode(comprs_pubkey).unwrap();
        let pub_key_hash = hash160::Hash::hash(&pub_key_bytes).to_hex();
        let hh = Vec::from_hex(&pub_key_hash).unwrap();

        //bech32
        let hash5 = convert_bits(&hh, 8, 5, true);
        let b32 = Bech32 {
            hrp: "cosmos".to_string(),
            data: hash5.unwrap(),
        }; //todo use bitcoin_hash istead
        let address = match b32.to_string() {
            Ok(s) => s,
//            Err(e) => return Err(error::Error::AddressError),
            Err(e) => return Err(format_err!("AddressError")),
        };
        Ok(address)
    }

    pub fn display_address(path: &str) -> Result<String> {
        let address = CosmosAddress::get_address(path).unwrap();
        let reg_apdu = CosmosApdu::register_pubkey(address.as_bytes());
        let res_reg = message::send_apdu(reg_apdu);
        ApduCheck::checke_response(&res_reg)?;
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::CosmosAddress;
    use bech32::bech32::Bech32;
    use common::constants;
    use device::device_binding::DeviceManage;

    #[test]
    fn test_get_pub_key() {
        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let check_result = DeviceManage::bind_check(&path).unwrap_or_default();
        println!("check_result:{}",&check_result);

        let comprs_pubkey = CosmosAddress::get_pub_key(constants::COSMOS_PATH).unwrap();
        assert_eq!(&comprs_pubkey,"0232C1EF21D73C19531B0AA4E863CF397C2B982B2F958F60CDB62969824C096D65");
    }

    #[test]
    fn test_get_address() {
        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let check_result = DeviceManage::bind_check(&path).unwrap_or_default();
        println!("check_result:{}",&check_result);

        let address = CosmosAddress::get_address(constants::COSMOS_PATH).unwrap();
        assert_eq!(&address,"cosmos1ajz9y0x3wekez7tz2td2j6l2dftn28v26dd992");
    }

    #[test]
    fn test_display_address() {
        let address = CosmosAddress::display_address(constants::COSMOS_PATH).unwrap();
        assert_eq!(&address,"cosmos1ajz9y0x3wekez7tz2td2j6l2dftn28v26dd992");
    }

    #[test]
    fn testBech32() {
        //        let encoded = bech32::encode("bech32", vec![0x00, 0x01, 0x02].to_base32()).unwrap();
        //        assert_eq!(encoded, "bech321qqqsyrhqy2a".to_string());

        let b32 = Bech32 {
            hrp: "bech32".to_string(),
            data: vec![0x00, 0x01, 0x02],
        };
        let address = match b32.to_string() {
            Ok(s) => s,
            Err(e) => return,
        };
        assert_eq!(address, "bech321qpz4nc4pe".to_string());
    }
}
