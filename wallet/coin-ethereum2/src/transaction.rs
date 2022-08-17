use crate::Result;
use common::apdu::{Apdu, ApduCheck, BlsApdu};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::secp256k1_sign;
use common::{constants, utility};
use device::device_binding::KEY_MANAGER;
use transport::message::{send_apdu, send_apdu_timeout};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Eth2Sign {}

impl Eth2Sign {
    pub fn msg_sign(data: Vec<u8>, path: &str) -> Result<String> {
        check_path_validity(path).expect("check path error");
        let select_apdu = Apdu::select_applet(constants::ETH2_AID);
        let select_result = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_result)?;

        let hash = data;

        //organize data
        let mut data_pack: Vec<u8> = Vec::new();

        data_pack.extend([1, hash.len() as u8].iter());
        data_pack.extend(hash.iter());

        //path
        data_pack.extend([2, path.as_bytes().len() as u8].iter());
        data_pack.extend(path.as_bytes().iter());

        let key_manager_obj = KEY_MANAGER.lock();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, &data_pack).unwrap();

        let mut apdu_pack: Vec<u8> = Vec::new();
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.extend(data_pack.as_slice());

        //sign
        let mut sign_response = "".to_string();
        let sign_apdus = BlsApdu::msg_sign(&apdu_pack);
        for apdu in sign_apdus {
            sign_response = send_apdu_timeout(apdu, constants::TIMEOUT_LONG)?;
            ApduCheck::check_response(&sign_response)?;
        }

        // verify
        let sign_len = usize::from_str_radix(&sign_response[..2], 16).unwrap() * 2 + 2;
        let sign_source_val = &sign_response[..sign_len];
        let sign_result = &sign_response[sign_len..sign_response.len() - 4];
        let sign_verify_result = utility::secp256k1_sign_verify(
            &key_manager_obj.se_pub_key,
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(sign_source_val).unwrap().as_slice(),
        )?;

        if !sign_verify_result {
            return Err(CoinError::ImkeySignatureVerifyFail.into());
        }

        let sig = hex::decode(&sign_response[2..sign_len])?;

        Ok(hex::encode(sig))
    }
}

#[cfg(test)]
mod test {
    use crate::transaction::Eth2Sign;
    use device::device_binding::bind_test;
    #[test]
    fn msg_sign_test() {
        bind_test();
        let hash = hex::decode("64726E3DA8").unwrap();
        let path = "m/0";
        let signature = Eth2Sign::msg_sign(hash, path);
        println!("signature-->{}", signature.unwrap())
    }
}
