use crate::common::constants;
use crate::Result;
use common::apdu::{ApduCheck, BtcApdu, CoinCommonApdu, Apdu, Ed25519Apdu};
use secp256k1::PublicKey;
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::ed25519::Public;
use sp_core::Public as TraitPublic;
use std::str::FromStr;
use transport::message::send_apdu;
use common::path::check_path_validity;
use common::utility::{secp256k1_sign, secp256k1_sign_verify, uncompress_pubkey_2_compress};
use common::error::CoinError;
use device::device_binding::KEY_MANAGER;

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
    pub fn get_address(path: &str, address_type: AddressType) -> Result<String> {
        // check_path_validity(path).expect("check path error");

        let select_apdu = Apdu::select_applet("695F627463");
        let select_response = send_apdu(select_apdu)?;
        ApduCheck::checke_response(&select_response)?;

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
        ApduCheck::checke_response(&res_msg_pubkey)?;

        let sign_source_val = &res_msg_pubkey[..194];
        let sign_result = &res_msg_pubkey[194..res_msg_pubkey.len() - 4];

        //verify
        let sign_verify_result = secp256k1_sign_verify(
            &key_manager_obj.se_pub_key,
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(sign_source_val).unwrap().as_slice(),
        )?;
        if !sign_verify_result {
            return Err(CoinError::ImkeySignatureVerifyFail.into());
        }
        // let comprs_pubkey = uncompress_pubkey_2_compress(&res_msg_pubkey[..res_msg_pubkey.len()-4]);
        // let comprs_pubkey_bytes = hex::decode(&comprs_pubkey).expect("decode ckb pubkey error");
        let address = SubstrateAddress::from_public_key(&res_msg_pubkey[..res_msg_pubkey.len()-4].as_bytes(),address_type)?;
        Ok(address)
    }

    pub fn display_address(path: &str, address_type: AddressType) -> Result<String> {
        let address = SubstrateAddress::get_address(path,address_type)?;
        let menu_name = match address_type {
            AddressType::Polkadot => "Polkadot",
            AddressType::Kusama => "Kusama",
            _ => panic!("address type support"),
        };
        let menu_name = "Polkadot".as_bytes();
        let reg_apdu = Ed25519Apdu::register_address(menu_name, address.as_bytes());
        let res_reg = send_apdu(reg_apdu)?;
        ApduCheck::checke_response(&res_reg)?;
        Ok(address)
    }
}

/**
get xpub
*/
pub fn get_pub_key(path: &str, verify_flag: bool) -> Result<String> {
    let select_response = send_apdu(BtcApdu::select_applet())?;
    ApduCheck::checke_response(&select_response)?;
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, verify_flag))?;
    ApduCheck::checke_response(&xpub_data)?;
    Ok(xpub_data[..130].to_string())
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum AddressType {
    Polkadot,
    Kusama,
}

#[cfg(test)]
mod test {
    use schnorrkel::{ExpansionMode, MiniSecretKey};
    use sp_core::crypto::Ss58AddressFormat;
    use sp_core::crypto::Ss58Codec;
    use sp_core::sr25519::{Pair, Public};
    use sp_core::Pair as TraitPair;
    use crate::address::{AddressType, SubstrateAddress};
    use device::device_binding::bind_test;

    #[test]
    fn key_test() {
        let mut seed = Pair::generate().1.to_vec();
        seed = hex::decode("1111111111111111111111111111111111111111111111111111111111111111")
            .unwrap();
        println!("{}", hex::encode_upper(seed.clone()));
        let mini_key: MiniSecretKey = MiniSecretKey::from_bytes(seed.as_slice())
            .expect("32 bytes can always build a key; qed");

        let kp = mini_key.expand_to_keypair(ExpansionMode::Ed25519);
        let gen_pair = Pair::from(kp);
        let polakdot_address = gen_pair
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::PolkadotAccount);
        println!("polakdot_address : {}", polakdot_address);
        let polakdot_address = gen_pair
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::KusamaAccount);
        println!("kusama_address : {}", polakdot_address);
        let polakdot_address = gen_pair
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::SubstrateAccount);
        println!("substrate_address : {}", polakdot_address);
        let temp_pub_key = gen_pair.public().0;
        let temp_private = gen_pair.to_raw_vec();
        println!("public key: {}", hex::encode_upper(temp_pub_key));
        println!("private key: {}", hex::encode_upper(temp_private));
    }

    #[test]
    fn test_address_from_public() {
        let pub_key = hex::decode("50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859")
            .expect("hex decode error");
        let address = SubstrateAddress::from_public_key(&pub_key, AddressType::Polkadot).expect("invalid public key");
        assert_eq!("12pWV6LvG4iAfNpFNTvvkWy3H9H8wtCkjiXupAzo2BCmPViM", address);

        let pub_key = hex::decode("50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859")
            .expect("hex decode error");
        let address = SubstrateAddress::from_public_key(&pub_key, AddressType::Kusama).expect("invalid public key");
        assert_eq!("EPq15Rj2eTcyVdBBXgyWKVta7Zj4FTo7beB3YHPwtPjxEkr", address);
    }

    #[test]
    fn test_get_address() {
        bind_test();
        let address = SubstrateAddress::get_address("m/44'/434'/0’/0’/0’",AddressType::Kusama).expect("get address error");
        assert_eq!("EPq15Rj2eTcyVdBBXgyWKVta7Zj4FTo7beB3YHPwtPjxEkr", address);
    }
}
