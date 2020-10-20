use crate::address::FilecoinAddress;
use crate::filecoinapi::{FilecoinTxReq, FilecoinTxRes, Signature, UnsignedMessage};
use crate::utils::message_digest;
use crate::Result;

use common::apdu::{ApduCheck, CoinCommonApdu, FilecoinApdu};
use common::error::CoinError;
use common::utility::{hex_to_bytes, is_valid_hex, retrieve_recid, secp256k1_sign, sha256_hash};
use common::{constants, path, utility};
use device::device_binding::KEY_MANAGER;

use forest_address::Address;
use forest_message::UnsignedMessage as ForestUnsignedMessage;
use forest_vm::Serialized;
use num_bigint_chainsafe::BigInt;
use secp256k1::recovery::{RecoverableSignature, RecoveryId};
use secp256k1::{self, Message as SecpMessage, Signature as SecpSignature};
use serde_cbor::to_vec;
use std::str::FromStr;
use transport::message::{send_apdu, send_apdu_timeout};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {}

impl Transaction {
    fn convert_message(message: &UnsignedMessage) -> Result<ForestUnsignedMessage> {
        let to = Address::from_str(&message.to).map_err(|_| CoinError::InvalidAddress)?;
        let from = Address::from_str(&message.from).map_err(|_| CoinError::InvalidAddress)?;
        let value = BigInt::from_str(&message.value).map_err(|_| CoinError::InvalidNumber)?;
        let gas_limit = message.gas_limit;
        let gas_fee_cap =
            BigInt::from_str(&message.gas_fee_cap).map_err(|_| CoinError::InvalidNumber)?;
        let gas_premium =
            BigInt::from_str(&message.gas_premium).map_err(|_| CoinError::InvalidNumber)?;

        let message_params_bytes =
            base64::decode(&message.params).map_err(|_| CoinError::InvalidParam)?;
        let params = Serialized::new(message_params_bytes);

        let tmp = ForestUnsignedMessage::builder()
            .to(to)
            .from(from)
            .sequence(message.nonce)
            .value(value)
            .method_num(message.method)
            .params(params)
            .gas_limit(gas_limit)
            .gas_premium(gas_premium)
            .gas_fee_cap(gas_fee_cap)
            .build()
            .map_err(|_| CoinError::InvalidFormat)?;

        Ok(tmp)
    }

    pub fn sign_tx(tx_input: FilecoinTxReq) -> Result<FilecoinTxRes> {
        path::check_path_validity(&tx_input.path).unwrap();

        let tx = tx_input.message.unwrap();
        let unsigned_message = Self::convert_message(&tx)?;
        let cbor_buffer = to_vec(&unsigned_message)?;
        let cid = message_digest(&cbor_buffer);

        let signature_type = 2;

        //check address
        let address =
            FilecoinAddress::get_address(tx_input.path.as_str(), tx_input.network.as_str())?;

        //compare address
        if address != tx_input.from_dis {
            return Err(CoinError::ImkeyAddressMismatchWithPath.into());
        }

        //organize data
        let mut data_pack: Vec<u8> = Vec::new();

        data_pack.extend([1, cid.len() as u8].iter());
        data_pack.extend(cid.iter());

        //path
        data_pack.extend([2, tx_input.path.as_bytes().len() as u8].iter());
        data_pack.extend(tx_input.path.as_bytes().iter());
        //payment info in TLV format
        data_pack.extend([7, tx_input.payment_dis.as_bytes().len() as u8].iter());
        data_pack.extend(tx_input.payment_dis.as_bytes().iter());
        //receiver info in TLV format
        data_pack.extend([8, tx_input.to_dis.as_bytes().len() as u8].iter());
        data_pack.extend(tx_input.to_dis.as_bytes().iter());
        //fee info in TLV format
        data_pack.extend([9, tx_input.fee_dis.as_bytes().len() as u8].iter());
        data_pack.extend(tx_input.fee_dis.as_bytes().iter());

        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, &data_pack).unwrap();

        let mut apdu_pack: Vec<u8> = Vec::new();
        apdu_pack.push(0x00);
        apdu_pack.push(bind_signature.len() as u8);
        apdu_pack.extend(bind_signature.as_slice());
        apdu_pack.extend(data_pack.as_slice());

        //prepare apdu
        let msg_prepare = FilecoinApdu::prepare_sign(apdu_pack);
        for msg in msg_prepare {
            let res = send_apdu_timeout(msg, constants::TIMEOUT_LONG)?;
            ApduCheck::checke_response(&res)?;
        }

        //sign
        let msg_sign = FilecoinApdu::sign_digest(tx_input.path.as_str());
        let res_msg_sign = send_apdu(msg_sign)?;
        ApduCheck::checke_response(&res_msg_sign)?;

        let sign_compact = &res_msg_sign[2..130];
        let sign_compact_vec = hex_to_bytes(sign_compact).unwrap();

        let mut signnture_obj = SecpSignature::from_compact(sign_compact_vec.as_slice()).unwrap();
        signnture_obj.normalize_s();
        let normalizes_sig_vec = signnture_obj.serialize_compact();

        //get public
        let msg_pubkey = FilecoinApdu::get_xpub(&tx_input.path, false);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        ApduCheck::checke_response(&res_msg_pubkey)?;

        let pubkey_raw = hex_to_bytes(&res_msg_pubkey[..130]).unwrap();

        let rec_id = utility::retrieve_recid(&cid, &normalizes_sig_vec, &pubkey_raw).unwrap();

        let mut data_arr = [0; 65];
        data_arr[0..64].copy_from_slice(&normalizes_sig_vec[0..64]);
        data_arr[64] = rec_id.to_i32() as u8;

        let signature_type = 1;

        Ok(FilecoinTxRes {
            cid: base64::encode(&cid),
            message: Some(tx.clone()),
            signature: Some(Signature {
                r#type: signature_type,
                data: base64::encode(&data_arr.to_vec()),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use device::device_binding::bind_test;
    use hex;

    #[test]
    fn test_sign_trans() {
        bind_test();
        let message = UnsignedMessage {
            to: "f1d2xrzcslx7xlbbylc5c3d5lvandqw4iwl6epxba".to_string(),
            from: "f1o2ph66tg7o7obyrqa7eiwiinrltauzxitkuk4ay".to_string(),
            nonce: 1,
            value: "100000".to_string(),
            gas_limit: 1,
            gas_fee_cap: "1".to_string(),
            gas_premium: "1".to_string(),
            method: 0,
            params: "".to_string(),
        };

        let tx_input = FilecoinTxReq {
            message: Some(message),
            path: "m/44'/461'/0/0/0".to_string(),
            network: "MAINNET".to_string(),
            payment_dis: "1 FILECION".to_string(),
            to_dis: "f1d2xrzcslx7xlbbylc5c3d5lvandqw4iwl6epxba".to_string(),
            from_dis: "f1o2ph66tg7o7obyrqa7eiwiinrltauzxitkuk4ay".to_string(),
            fee_dis: "0.1 FILECION".to_string(),
        };

        let tx_result = Transaction::sign_tx(tx_input).unwrap();
        let signature = tx_result.signature.unwrap();

        assert_eq!(signature.r#type, 1);
        assert_eq!(signature.data, "k/ODPDElcw/xCQ0WWO3r7H3GoKpJVX7j6x1lyNFZ4YNvoWx8/RVqn0/+GNUvFCj1EOEXKFNf2h5LsBmiHDllkgE=");
    }
}
