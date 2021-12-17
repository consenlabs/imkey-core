use crate::address::CosmosAddress;
use crate::cosmosapi::CosmosTxOutput;
use crate::Result;
use bitcoin_hashes::hex::ToHex;
use common::apdu::{ApduCheck, CoinCommonApdu, CosmosApdu};
use common::constants;
use common::utility::{hex_to_bytes, secp256k1_sign, sha256_hash};
use device::device_binding::KEY_MANAGER;
use secp256k1::{self, Signature as SecpSignature};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use transport::message::{send_apdu, send_apdu_timeout};

#[derive(Debug)]
pub struct CosmosTransaction {
    pub sign_data: String,
    pub path: String,
    pub payment_dis: String,
    pub to_dis: String,
    pub fee_dis: String,
}

impl CosmosTransaction {
    pub fn sign(self) -> Result<CosmosTxOutput> {
        let sign_hash = sha256_hash(hex_to_bytes(&self.sign_data)?.as_slice());
        let mut sign_pack = "0120".to_string();
        sign_pack.push_str(&sign_hash.to_hex());
        if self.payment_dis == "" {
            sign_pack.push_str("070008000900");
        } else {
            sign_pack.push_str("07");
            sign_pack.push_str(&format!("{:02x}", self.payment_dis.as_bytes().len()));
            sign_pack.push_str(&hex::encode(&self.payment_dis));
            sign_pack.push_str("08");
            sign_pack.push_str(&format!("{:02x}", self.to_dis.as_bytes().len()));
            sign_pack.push_str(&hex::encode(&self.to_dis));
            sign_pack.push_str("09");
            sign_pack.push_str(&format!("{:02x}", self.fee_dis.as_bytes().len()));
            sign_pack.push_str(&hex::encode(&self.fee_dis));
        }

        let sign_pack_vec = hex::decode(sign_pack).expect("Decoding failed");

        let key_manager_obj = KEY_MANAGER.lock();
        let mut prepare_data = secp256k1_sign(&key_manager_obj.pri_key, &sign_pack_vec.as_slice())?;
        std::mem::drop(key_manager_obj);
        prepare_data.insert(0, prepare_data.len() as u8);
        prepare_data.insert(0, 0x00);
        prepare_data.extend(sign_pack_vec.iter());

        let select_apdu = CosmosApdu::select_applet();
        let select_response = send_apdu(select_apdu)?;
        ApduCheck::check_response(&select_response)?;

        let prepare_apdus = CosmosApdu::prepare_sign(prepare_data);

        for apdu in prepare_apdus {
            let response = send_apdu_timeout(apdu, constants::TIMEOUT_LONG)?;
            ApduCheck::check_response(&response)?;
        }

        let sign_apdu = CosmosApdu::sign_digest(constants::COSMOS_PATH);

        let sign_result = send_apdu(sign_apdu)?;
        ApduCheck::check_response(&sign_result)?;

        let sign_compact = hex::decode(&sign_result[2..130]).unwrap();
        let mut signature_obj = SecpSignature::from_compact(sign_compact.as_slice()).unwrap();
        signature_obj.normalize_s();
        let signature = signature_obj.serialize_compact().to_hex();

        let pub_key = CosmosAddress::get_pub_key(&self.path)?;

        let output = CosmosTxOutput { signature, pub_key };
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    // use crate::transaction::{Coin, CosmosTransaction, SignData, StdFee};
    use crate::transaction::CosmosTransaction;
    use common::constants;
    use common::utility::{hex_to_bytes, secp256k1_sign};
    use device::device_binding::bind_test;
    use serde_json::json;

    #[test]
    fn test_ecsign() {
        let sign_pack = hex_to_bytes(
            "0120D560F6EAB74C1D26DD5FAB27B9F700F4C371AC76A82E9A2E534269322D129E2F070008000900",
        )
        .unwrap();
        let private_key =
            hex_to_bytes("F85B222058BBEFFF888AAF7AD1D08B0C9C5FF719027F7DB69859B72A17B28749")
                .unwrap();
        let prepare_data = secp256k1_sign(&private_key, &sign_pack.as_slice()).unwrap();
        let prepare_data_hex = hex::encode(&prepare_data);
        assert_eq!(prepare_data_hex,
        "3045022100a773a750391978586598843f89921d33083f670049906dc68ad312867df2826d0220312d22dcc102d8ba2a86972c7c73f082c53b29ef0a04ac630def935ed996d9c2"
        );
    }

    #[test]
    fn test_base64() {
        let hex = "477135B0DF08980F927D1569A780B4C4D24DA503BBCF98B87F606C29D47110FB654A8BAC272C80860018D77039563644209011717F4A69691F6B27C44C48002E".to_string();
        let bytes = hex::decode(&hex).unwrap();
        let base64 = base64::encode(&bytes);
        assert_eq!(base64,
        "R3E1sN8ImA+SfRVpp4C0xNJNpQO7z5i4f2BsKdRxEPtlSousJyyAhgAY13A5VjZEIJARcX9KaWkfayfETEgALg=="
        );
    }

    #[test]
    fn test_sign_delegate() {
        bind_test();
        let sign_data= "7b226163636f756e745f6e756d626572223a2231323334353637383930222c22636861696e5f6964223a2274656e6465726d696e745f74657374222c22666565223a7b22616d6f756e74223a5b7b22616d6f756e74223a2230222c2264656e6f6d223a22227d5d2c22676173223a223231393036227d2c226d656d6f223a22222c226d736773223a5b7b2274797065223a22636f736d6f732d73646b2f4d736744656c6567617465222c2276616c7565223a7b22616d6f756e74223a5b7b22616d6f756e74223a223130222c2264656e6f6d223a2261746f6d227d5d2c2264656c656761746f725f61646472657373223a22636f736d6f73317930613873633561797635326632666d35743768723267383871676c6a7a6b346a637a373866222c2276616c696461746f725f61646472657373223a22636f736d6f7376616c6f706572317a6b757072383368727a6b6e33757035656c6b747a63713374756674386e78736d7764716770227d7d5d2c2273657175656e6365223a2231323334353637383930227d".to_string();
        let input = CosmosTransaction {
            sign_data,
            path: constants::COSMOS_PATH.to_string(),
            payment_dis: "".to_string(),
            to_dis: "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt".to_string(),
            fee_dis: "0.00075 atom".to_string(),
        };
        let cosmos_tx_output = input.sign().unwrap();
        assert_eq!("878fff70e60b4e20d86ddc3ed4d559b9fc29a4801c110d71b1631870275c1adb2ee314a7612b71c898086b9ef8e39b079d78163fce15be743981ea146402b195", cosmos_tx_output.signature);
        assert_eq!(
            "0232C1EF21D73C19531B0AA4E863CF397C2B982B2F958F60CDB62969824C096D65",
            cosmos_tx_output.pub_key
        )
    }

    #[test]
    fn test_sign_payment_dis() {
        bind_test();
        let sign_data = "7b226163636f756e745f6e756d626572223a2231323334353637383930222c22636861696e5f6964223a2274656e6465726d696e745f74657374222c22666565223a7b22616d6f756e74223a5b7b22616d6f756e74223a2230222c2264656e6f6d223a22227d5d2c22676173223a223231393036227d2c226d656d6f223a22222c226d736773223a5b7b2274797065223a22636f736d6f732d73646b2f4d736744656c6567617465222c2276616c7565223a7b22616d6f756e74223a5b7b22616d6f756e74223a223130222c2264656e6f6d223a2261746f6d227d5d2c2264656c656761746f725f61646472657373223a22636f736d6f73317930613873633561797635326632666d35743768723267383871676c6a7a6b346a637a373866222c2276616c696461746f725f61646472657373223a22636f736d6f7376616c6f706572317a6b757072383368727a6b6e33757035656c6b747a63713374756674386e78736d7764716770227d7d5d2c2273657175656e6365223a2231323334353637383930227d".to_string();
        let input = CosmosTransaction {
            sign_data,
            path: constants::COSMOS_PATH.to_string(),
            payment_dis: "0.001 ATOM".to_string(),
            to_dis: "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt".to_string(),
            fee_dis: "0.00075 atom".to_string(),
        };
        let cosmos_tx_output = input.sign().unwrap();
        assert_eq!("878fff70e60b4e20d86ddc3ed4d559b9fc29a4801c110d71b1631870275c1adb2ee314a7612b71c898086b9ef8e39b079d78163fce15be743981ea146402b195", cosmos_tx_output.signature);
        assert_eq!(
            "0232C1EF21D73C19531B0AA4E863CF397C2B982B2F958F60CDB62969824C096D65",
            cosmos_tx_output.pub_key
        )
    }

    #[test]
    fn test_sort_vec() {
        let mut vec = Vec::new();
        vec.push("richard");
        vec.push("charles");
        vec.push("peter");
        vec.push("from");
        vec.push("to");
        vec.push("delegate");
        vec.push("valide");

        vec.sort();
        assert_eq!(
            format!("{:?}", vec),
            "[\"charles\", \"delegate\", \"from\", \"peter\", \"richard\", \"to\", \"valide\"]"
        );
    }
}
