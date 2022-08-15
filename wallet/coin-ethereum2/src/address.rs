use crate::Result;

use common::apdu::{Apdu, ApduCheck, BlsApdu};
use common::constants::ETH2_AID;
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility;
use common::utility::secp256k1_sign_verify;
use device::device_binding::KEY_MANAGER;
use hex;
use transport::message;
use transport::message::send_apdu;

#[derive(Debug)]
pub struct Eth2Address {}

impl Eth2Address {
    pub fn get_pub_key(path: &str) -> Result<String> {
        check_path_validity(path)?;

        let select_apdu = Apdu::select_applet(ETH2_AID);
        let select_response = message::send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_response)?;

        let key_manager_obj = KEY_MANAGER.lock();
        let bind_signature = utility::secp256k1_sign(&key_manager_obj.pri_key, &path.as_bytes())?;

        let mut apdu_pack: Vec<u8> = vec![];
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.push(0x01);
        apdu_pack.push(path.as_bytes().len() as u8);
        apdu_pack.extend(path.as_bytes());

        //get public
        let msg_pubkey = BlsApdu::get_xpub(&apdu_pack);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        ApduCheck::check_response(&res_msg_pubkey)?;

        let pubkey = &res_msg_pubkey[..96];
        let sign_result = &res_msg_pubkey[96..res_msg_pubkey.len() - 4];

        //se signature verify
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
}

#[cfg(test)]
mod test {
    use crate::address::Eth2Address;
    use common::constants;
    use device::device_binding::bind_test;

    #[test]
    fn test_get_pub_key() {
        bind_test();

        let uncomprs_pubkey = Eth2Address::get_pub_key("m/0").unwrap();
        assert_eq!(
            &uncomprs_pubkey,
            "044B9C3C0E1CEFD90897798E7CE471FEFF0D1BE4C6BA24061D7D9F68CFDB19A0EC0192392A94B121743ADB91C7029C6F3C80FD18B6E34E8B8F9EA87E559C68FDC4"
        );
    }
}
