use crate::Result;
use common::apdu::{Apdu, ApduCheck, BtcApdu, CoinCommonApdu, Ed25519Apdu};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::{secp256k1_sign, secp256k1_sign_verify, uncompress_pubkey_2_compress};
use device::device_binding::KEY_MANAGER;
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::ed25519::Public;
use sp_core::Public as TraitPublic;
use std::str::FromStr;
use transport::message::send_apdu;

pub struct SubstrateAddress();
impl SubstrateAddress {
    pub fn from_public_key(public_key: &[u8], address_type: AddressType) -> Result<String> {
        let public_obj = Public::from_slice(public_key);
        let address = match address_type {
            AddressType::Polkadot => {
                public_obj.to_ss58check_with_version(Ss58AddressFormat::PolkadotAccount)
            }
            AddressType::Kusama => {
                public_obj.to_ss58check_with_version(Ss58AddressFormat::KusamaAccount)
            }
            _ => panic!("address type support"),
        };

        Ok(address)
    }

    pub fn get_public_key(path: &str) -> Result<String>{
        check_path_validity(path).expect("check path error");

        let select_apdu = Apdu::select_applet("695F656473725F646F74");
        let select_response = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_response)?;

        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, path.as_bytes())?;

        let mut apdu_pack: Vec<u8> = vec![];
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.push(0x01);
        apdu_pack.push(path.as_bytes().len() as u8);
        apdu_pack.extend(path.as_bytes());

        //get public
        let msg_pubkey = Ed25519Apdu::get_xpub(&apdu_pack);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        ApduCheck::check_response(&res_msg_pubkey)?;

        let pubkey = &res_msg_pubkey[..64];
        let sign_result = &res_msg_pubkey[64..res_msg_pubkey.len() - 4];

        println!("pubkey: {}", pubkey);

        //verify
        let sign_verify_result = secp256k1_sign_verify(
            &key_manager_obj.se_pub_key,
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(pubkey).unwrap().as_slice(),
        )?;
        if !sign_verify_result {
            return Err(CoinError::ImkeySignatureVerifyFail.into());
        }

        Ok(pubkey.to_string())
    }

    pub fn get_address(path: &str, address_type: AddressType) -> Result<String> {
        let public_key = SubstrateAddress::get_public_key(path)?;
        let address = SubstrateAddress::from_public_key(
            &hex::decode(&public_key)?,
            address_type,
        )?;
        Ok(address)
    }

    pub fn display_address(path: &str, address_type: AddressType) -> Result<String> {
        let address = SubstrateAddress::get_address(path, address_type)?;
        let menu_name = match address_type {
            AddressType::Polkadot => "Polkadot",
            AddressType::Kusama => "Kusama",
            _ => panic!("address type support"),
        };
        let menu_name = "Polkadot".as_bytes();
        let reg_apdu = Ed25519Apdu::register_address(menu_name, address.as_bytes());
        let res_reg = send_apdu(reg_apdu)?;
        ApduCheck::check_response(&res_reg)?;
        Ok(address)
    }
}

/**
get xpub
*/
pub fn get_pub_key(path: &str, verify_flag: bool) -> Result<String> {
    let select_response = send_apdu(BtcApdu::select_applet())?;
    ApduCheck::check_response(&select_response)?;
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, verify_flag))?;
    ApduCheck::check_response(&xpub_data)?;
    Ok(xpub_data[..130].to_string())
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum AddressType {
    Polkadot,
    Kusama,
}

#[cfg(test)]
mod test {
    use crate::address::{AddressType, SubstrateAddress};
    use common::constants::{KUSAMA_PATH, POLKADOT_PATH};
    use device::device_binding::bind_test;
    use sp_core::crypto::Ss58AddressFormat;
    use sp_core::crypto::Ss58Codec;
    use sp_core::sr25519::{Pair, Public};
    use sp_core::Pair as TraitPair;

    #[test]
    fn test_address_from_public() {
        let pub_key =
            hex::decode("EDB9955556C8E07287DF95AD77FAD826168F8A50488CCE0D738DF3769E24613A")
                .expect("hex decode error");
        let address = SubstrateAddress::from_public_key(&pub_key, AddressType::Polkadot)
            .expect("invalid public key");
        assert_eq!("12pWV6LvG4iAfNpFNTvvkWy3H9H8wtCkjiXupAzo2BCmPViM", address);

        let pub_key =
            hex::decode("50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859")
                .expect("hex decode error");
        let address = SubstrateAddress::from_public_key(&pub_key, AddressType::Kusama)
            .expect("invalid public key");
        assert_eq!("EPq15Rj2eTcyVdBBXgyWKVta7Zj4FTo7beB3YHPwtPjxEkr", address);
    }

    #[test]
    fn test_get_address() {
        bind_test();
        let address = SubstrateAddress::get_address(POLKADOT_PATH, AddressType::Polkadot)
            .expect("get address error");
        assert_eq!("16NhUkUTkYsYRjMD22Sop2DF8MAXUsjPcYtgHF3t1ccmohx1", address);
        let address = SubstrateAddress::get_address(KUSAMA_PATH, AddressType::Kusama)
            .expect("get address error");
        assert_eq!("DXQbtNdVTDL5CDFW4DoGL8v14A5zaGWukRdQsY1xT1vCJgH", address);
    }

    #[test]
    fn test_display_address() {
        bind_test();
        let address = SubstrateAddress::display_address(POLKADOT_PATH, AddressType::Polkadot)
            .expect("get address error");
        assert_eq!("147mvrDYhFpZzvFASKBDNVcxoyz8XCVNyyFKSZcpbQxN33TT", address);
        let address = SubstrateAddress::display_address(KUSAMA_PATH, AddressType::Kusama)
            .expect("get address error");
        assert_eq!("DXQbtNdVTDL5CDFW4DoGL8v14A5zaGWukRdQsY1xT1vCJgH", address);
    }
}
