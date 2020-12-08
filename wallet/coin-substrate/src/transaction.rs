use crate::substrateapi::{SubstrateRawTxIn, SubstrateTxOut};
use crate::{Result, SIGNATURE_TYPE_ED25519,PAYLOAD_HASH_THRESHOLD};
use common::{SignParam, constants, utility};
use sp_core::blake2_256;
use common::apdu::{Ed25519Apdu, ApduCheck, Apdu};
use transport::message::{send_apdu_timeout, send_apdu};
use common::utility::secp256k1_sign;
use common::error::CoinError;
use device::device_binding::KEY_MANAGER;
use common::constants::POLKADOT_AID;

#[derive(Debug)]
pub struct Transaction {}

impl Transaction {
    pub fn hash_unsigned_payload(payload: &[u8]) -> Result<Vec<u8>> {
        if payload.len() > PAYLOAD_HASH_THRESHOLD {
            Ok(blake2_256(&payload).to_vec())
        } else {
            Ok(payload.to_vec())
        }
    }

    pub fn sign_transaction(
        tx: &SubstrateRawTxIn,
        sign_param: &SignParam,
    ) -> Result<SubstrateTxOut> {
        let select_apdu = Apdu::select_applet(POLKADOT_AID);
        let select_result = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_result)?;

        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };
        let raw_data_bytes = hex::decode(&raw_data_bytes)?;
        let hash = Transaction::hash_unsigned_payload(&raw_data_bytes)?;

        //organize data
        let mut data_pack: Vec<u8> = Vec::new();

        data_pack.extend([1, hash.len() as u8].iter());
        data_pack.extend(hash.iter());

        //path
        data_pack.extend([2, sign_param.path.as_bytes().len() as u8].iter());
        data_pack.extend(sign_param.path.as_bytes().iter());
        //payment info in TLV format
        data_pack.extend([7, sign_param.payment.as_bytes().len() as u8].iter());
        data_pack.extend(sign_param.payment.as_bytes().iter());
        //receiver info in TLV format
        data_pack.extend([8, sign_param.receiver.as_bytes().len() as u8].iter());
        data_pack.extend(sign_param.receiver.as_bytes().iter());
        //fee info in TLV format
        data_pack.extend([9, sign_param.fee.as_bytes().len() as u8].iter());
        data_pack.extend(sign_param.fee.as_bytes().iter());

        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, &data_pack).unwrap();

        let mut apdu_pack: Vec<u8> = Vec::new();
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.extend(data_pack.as_slice());

        //sign
        let mut sign_response = "".to_string();
        let sign_apdus = Ed25519Apdu::sign(&apdu_pack);
        for apdu in sign_apdus {
            sign_response = send_apdu_timeout(apdu, constants::TIMEOUT_LONG)?;
            ApduCheck::check_response(&sign_response)?;
        }

        // verify
        let sign_source_val = &sign_response[..132];
        let sign_result = &sign_response[132..sign_response.len() - 4];
        let sign_verify_result = utility::secp256k1_sign_verify(
            &key_manager_obj.se_pub_key,
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(sign_source_val).unwrap().as_slice(),
        )?;

        if !sign_verify_result {
            return Err(CoinError::ImkeySignatureVerifyFail.into());
        }

        let sig = hex::decode(&sign_response[2..130])?;
        let sig_with_type = [vec![SIGNATURE_TYPE_ED25519], sig.to_vec()].concat();

        let tx_out = SubstrateTxOut {
            signature: format!("0x{}", hex::encode(&sig_with_type)),
        };
        Ok(tx_out)
    }
}

#[cfg(test)]
mod test {
    use common::SignParam;
    use common::constants::POLKADOT_PATH;
    use crate::transaction::Transaction;
    use crate::substrateapi::SubstrateRawTxIn;
    use device::device_binding::bind_test;

    #[test]
    fn test_sign_transaction() {
        bind_test();
        let sign_param = SignParam {
            chain_type: "POLKADOT".to_string(),
            path: POLKADOT_PATH.to_string(),
            network: "".to_string(),
            input: None,
            payment: "25 DOT".to_string(),
            receiver: "12pWV6LvG4iAfNpFNTvvkWy3H9H8wtCkjiXupAzo2BCmPViM".to_string(),
            sender: "147mvrDYhFpZzvFASKBDNVcxoyz8XCVNyyFKSZcpbQxN33TT".to_string(),
            fee: "15.4000 milli DOT".to_string(),
        };

        let input = SubstrateRawTxIn{
            raw_data: "0x0600ffd7568e5f0a7eda67a82691ff379ac4bba4f9c9b859fe779b5d46363b61ad2db9e56c0703d148e25901007b000000dcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025bde8f69eeb5e065e18c6950ff708d7e551f68dc9bf59a07c52367c0280f805ec7".to_string()
        };
        let ret = Transaction::sign_transaction(&input,&sign_param).expect("sign error");

        assert_eq!("sig",ret.signature);
    }
}
