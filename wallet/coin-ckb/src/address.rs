use crate::hash::blake2b_160;
use crate::Result;
use bech32::ToBase32;
use common::path::check_path_validity;
use common::apdu::{Apdu, ApduCheck, Secp256k1Apdu};
use transport::message::send_apdu;
use common::utility::{secp256k1_sign, secp256k1_sign_verify, uncompress_pubkey_2_compress};
use common::error::CoinError;
use device::device_binding::KEY_MANAGER;
use common::constants::FILECOIN_AID;
use common::constants;

pub struct CkbAddress {}

impl CkbAddress {
    pub fn from_public_key(network: &str, pubkey: &[u8]) -> Result<String> {
        let prefix = match network {
            "TESTNET" => "ckt",
            _ => "ckb",
        };

        let pub_key_hash = blake2b_160(pubkey);

        let mut buf = vec![];
        buf.extend(vec![0x1, 0x00]); // append short version for locks with popular codehash and default code hash index
        buf.extend(pub_key_hash);

        Ok(bech32::encode(prefix, buf.to_base32())?)
    }

    pub fn get_address(network: &str, path: &str) -> Result<String> {
        check_path_validity(path).expect("check path error");

        let select_apdu = Apdu::select_applet(FILECOIN_AID);
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
        let msg_pubkey = Secp256k1Apdu::get_xpub(&apdu_pack);
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

        //compressed key
        let comprs_pubkey = uncompress_pubkey_2_compress(&res_msg_pubkey[..res_msg_pubkey.len()-4]);

        let address = CkbAddress::from_public_key(network,&comprs_pubkey.as_bytes())?;
        Ok(address)
    }

    pub fn display_address(network: &str, path: &str) -> Result<String> {
        let address = CkbAddress::get_address(network,path)?;
        let menu_name = "CKB".as_bytes();
        let reg_apdu = Secp256k1Apdu::register_address(menu_name, address.as_bytes());
        let res_reg = send_apdu(reg_apdu)?;
        ApduCheck::checke_response(&res_reg)?;
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::CkbAddress;
    use device::device_binding::bind_test;
    use common::constants;


    #[test]
    fn test_from_public_key() {
        let bytes = hex::decode("024a501efd328e062c8675f2365970728c859c592beeefd6be8ead3d901330bc01")
            .expect("hex decode error");
        let network = "TESTNET";
        assert_eq!(
            CkbAddress::from_public_key(network,&bytes).expect("invalid public key"),
            "ckt1qyqrdsefa43s6m882pcj53m4gdnj4k440axqswmu83"
        );

        let bytes = hex::decode("024a501efd328e062c8675f2365970728c859c592beeefd6be8ead3d901330bc01")
            .expect("hex decode error");
        let network = "MAINNET";
        assert_eq!(
            CkbAddress::from_public_key(network,&bytes).expect("invalid public key"),
            "ckb1qyqrdsefa43s6m882pcj53m4gdnj4k440axqdt9rtd"
        );
    }

    #[test]
    fn test_get_address() {
        bind_test();

        let network = "TESTNET";
        let address = CkbAddress::get_address(network,constants::CKB_PATH).expect("get address fail");
        assert_eq!(&address, "ckt1qyqtr684u76tu7r8efkd24hw8922xfvhnazskzdzy6");
    }

    #[test]
    fn test_display_address() {
        bind_test();

        let network = "TESTNET";
        let address = CkbAddress::display_address(network,constants::CKB_PATH).expect("get address fail");
        println!("address:{}", &address);
        assert_eq!(&address, "ckt1qyqtr684u76tu7r8efkd24hw8922xfvhnazskzdzy6");
    }
}
