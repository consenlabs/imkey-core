use crate::eth2api::{Eth2MsgSignInput, Eth2MsgSignOutput};
use crate::Result;
use common::apdu::{Apdu, ApduCheck, BlsApdu};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::{is_valid_hex, secp256k1_sign};
use common::{constants, utility, SignParam};
use device::device_binding::KEY_MANAGER;
use transport::message::{send_apdu, send_apdu_timeout};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Eth2Sign {}

impl Eth2Sign {
    pub fn msg_sign(
        msg_sign_data: Eth2MsgSignInput,
        sign_param: &SignParam,
    ) -> Result<Eth2MsgSignOutput> {
        check_path_validity(sign_param.path.as_str()).expect("check path error");
        let select_apdu = Apdu::select_applet(constants::ETH2_AID);
        let select_result = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_result)?;

        let message_to_sign;
        if is_valid_hex(&msg_sign_data.message) {
            let value = if msg_sign_data.message.to_lowercase().starts_with("0x") {
                &msg_sign_data.message[2..]
            } else {
                &msg_sign_data.message
            };
            message_to_sign = hex::decode(value).unwrap();
        } else {
            message_to_sign = msg_sign_data.message.into_bytes();
        }

        if message_to_sign.len() > 255 {
            return Err(CoinError::SignDataTooLong.into());
        }

        //organize data
        let mut data_pack: Vec<u8> = Vec::new();

        data_pack.extend([1, message_to_sign.len() as u8].iter());
        data_pack.extend(message_to_sign.iter());

        //path
        data_pack.extend([2, sign_param.path.as_bytes().len() as u8].iter());
        data_pack.extend(sign_param.path.as_bytes().iter());

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

        Ok(Eth2MsgSignOutput {
            signature: hex::encode(sig),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::eth2api::Eth2MsgSignInput;
    use crate::transaction::Eth2Sign;
    use common::{constants, SignParam};
    use device::device_binding::bind_test;

    #[test]
    fn msg_sign_test() {
        //mnemonicx = "gauge hole clog property soccer idea cycle stadium utility slice hold chief"
        bind_test();
        let sign_param = SignParam {
            chain_type: "ETHEREUM2".to_string(),
            path: constants::ETH2_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "".to_string(),
            fee: "".to_string(),
        };
        let eth2MsgSignInput = Eth2MsgSignInput {
            message: "0x64726E3DA8".to_string(),
        };
        let signature = Eth2Sign::msg_sign(eth2MsgSignInput, &sign_param);
        assert_eq!(signature.unwrap().signature, "a1367b7e99d5bf139a9cd6cd857bbf12c379397b1c9347afd794da0459efda3850298c33c9f292e5ebb05143e77afe4d092c51170866cb852abfd9bb7139e6e72e34b44318d6ab30390d48d6095c2df7489877de6091cc68329a0537a8da4bf2");
        let sign_param = SignParam {
            chain_type: "ETHEREUM2".to_string(),
            path: constants::ETH2_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "".to_string(),
            fee: "".to_string(),
        };
        let eth2MsgSignInput = Eth2MsgSignInput {
            message: "111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(),
        };
        let signature = Eth2Sign::msg_sign(eth2MsgSignInput, &sign_param);
        assert_eq!(signature.unwrap().signature, "b24f8da68fdfcc492fd57c68e8818513a2613c18f293042af9ac4d9a371f5f33d57848552a07292c313411db1306d1e409eba509a33c3162703b146516ef498d1bf32e6e4157922b345d688b5a75b4aab26d3c0f74e10040e481b9066b40abfc");
    }
}
