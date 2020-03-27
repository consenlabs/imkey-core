use crate::cosmosapi::{CosmosTxRes};
use serde::{Serialize,Deserialize};
use common::utility::{sha256_hash, secp256k1_sign};
use secp256k1::{self, Signature as SecpSignature};
use bitcoin_hashes::hex::ToHex;
use common::apdu::{CosmosApdu, ApduCheck};
use mq::message::send_apdu;
use common::constants;
use crate::address::CosmosAddress;
use crate::Result;
use device::device_binding::KEY_MANAGER;

#[derive(Debug)]
pub struct CosmosTransaction {
    pub sign_data: SignData,
    pub path: String,
    pub payment_dis: String,
    pub to_dis: String,
    pub fee_dis: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignData{
    pub account_number: String,
    pub chain_id: String,
    pub fee: StdFee,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub msgs: Vec<Msg>,
    pub sequence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StdFee{
    pub amount: Vec<Coin>,
    pub gas: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coin{
    pub amount: String,
    pub denom: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg{
    #[serde(rename = "type")]
    pub ttype: String,
    pub value: MsgValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgValue{
    pub amount: Vec<Coin>,
    pub delegator_address: String,
    pub validator_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StdSignature{
    pub account_number: String,
    pub pub_key:Pubkey,
    pub sequence: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pubkey{
    #[serde(rename = "type")]
    pub ttype: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StdTx{
    pub fee: StdFee,
    pub signatures: Vec<StdSignature>,
    pub msg: Vec<Msg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl CosmosTransaction {
    pub fn sign(self) -> Result<CosmosTxRes> {
        let json = serde_json::to_vec(&self.sign_data).unwrap();
        let json = String::from_utf8(json.to_owned()).unwrap();
        println!("{}", &json);//todo sort json
        let json_hash = sha256_hash(json.as_bytes()).to_hex();
        println!("hash:{}", &json_hash);

        let mut sign_pack = "0120".to_string();
        sign_pack.push_str(&json_hash);
        if self.payment_dis == "" {//todo check null?
            sign_pack.push_str("070008000900");
        }else{
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
        println!("sign_pack:{}", &sign_pack);

        let sign_pack_vec = hex::decode(sign_pack).expect("Decoding failed");

        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let mut prepare_data = secp256k1_sign(&key_manager_obj.pri_key, &sign_pack_vec.as_slice())?;
        std::mem::drop(key_manager_obj);
        let mut prepare_data_hex = hex::encode(&prepare_data);
        println!("prepare_data_hex:{}", &prepare_data_hex);
        prepare_data.insert(0, prepare_data.len() as u8);
        prepare_data.insert(0, 0x00);
        prepare_data.extend(sign_pack_vec.iter());
        prepare_data_hex = hex::encode(&prepare_data);
        println!("prepare_data_hex:{}", &prepare_data_hex);

        let select_apdu = CosmosApdu::select_applet();
        let select_response = send_apdu(select_apdu);
        ApduCheck::checke_response(&select_response)?;

        let prepare_apdus = CosmosApdu::prepare_sign(prepare_data);

        for apdu in prepare_apdus {
            println!("prepare_apdu:{}", &apdu);
            let response = send_apdu(apdu);
            ApduCheck::checke_response(&response)?;
        }

        let sign_apdu = CosmosApdu::sign_digest(constants::COSMOS_PATH);
        println!("sign_apdu:{}", &sign_apdu);

        let sign_result = send_apdu(sign_apdu);
        println!("sign_result:{}", &sign_result);
        ApduCheck::checke_response(&sign_result)?;

        let sign_compact = hex::decode(&sign_result[2..130]).unwrap();
        let mut signnture_obj = SecpSignature::from_compact(sign_compact.as_slice()).unwrap();
        signnture_obj.normalize_s();
        let normalizes_sig_vec = signnture_obj.serialize_compact();

        let sign_base64 = base64::encode(&normalizes_sig_vec.as_ref());
        println!("sign_base64:{}", &sign_base64);


        let pub_key = CosmosAddress::get_pub_key(&self.path).unwrap();
        let pub_key = hex::decode(pub_key).unwrap();
        let pub_key = base64::encode(&pub_key);
        println!("pub_key:{}", &pub_key);


        let std_signature = StdSignature{
            account_number: self.sign_data.account_number.to_string(),
            pub_key: Pubkey {
                ttype: "tendermint/PubKeySecp256k1".to_string(),
                value: pub_key.to_string()
            },
            sequence: self.sign_data.sequence.to_string(),
            signature:sign_base64,
        };

        let std_tx = StdTx{
            fee: self.sign_data.fee,
            signatures: vec![std_signature],
            msg: self.sign_data.msgs,
            memo: self.sign_data.memo,
        };

        let json = serde_json::to_vec(&std_tx).unwrap();
        let json = String::from_utf8(json.to_owned()).unwrap();
        println!("{}", &json);//todo sort json

        let ouput = CosmosTxRes {
            tx_data: json.to_string(),
            tx_hash: "".to_string(),
        };
        Ok(ouput)
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{CosmosTransaction, SignData, StdFee, Coin, Msg, MsgValue};
    use common::constants;
    use common::cosmosapi::CosmosTxInput;
    use common::utility::{secp256k1_sign, hex_to_bytes};
    use device::device_binding::DeviceManage;

    #[test]
    fn test_hex_bytes() {}

    #[test]
    fn test_ecsign() {
        let sign_pack = hex_to_bytes("0120D560F6EAB74C1D26DD5FAB27B9F700F4C371AC76A82E9A2E534269322D129E2F070008000900").unwrap();
        let private_key = hex_to_bytes("F85B222058BBEFFF888AAF7AD1D08B0C9C5FF719027F7DB69859B72A17B28749").unwrap();
        let mut prepare_data = secp256k1_sign(&private_key, &sign_pack.as_slice()).unwrap();
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
    fn test_sign() {
        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let check_result = DeviceManage::bind_check(&path).unwrap();
        println!("check_result:{}",&check_result);

        let stdfee = StdFee{
            amount: vec![Coin{
                amount: "0".to_string(),
                denom: "".to_string()
            }],
            gas: "21906".to_string()
        };

        let msg = Msg{
            ttype: "cosmos-sdk/MsgDelegate".to_string(),
            value: MsgValue{
                amount: vec![Coin{
                    amount: "10".to_string(),
                    denom: "atom".to_string()
                }],
                delegator_address: "cosmos1y0a8sc5ayv52f2fm5t7hr2g88qgljzk4jcz78f".to_string(),
                validator_address: "cosmosvaloper1zkupr83hrzkn3up5elktzcq3tuft8nxsmwdqgp".to_string(),
            },
        };

        let sign_data = SignData{
            account_number: "1".to_string(),
            chain_id: "tendermint_test".to_string(),
            fee: stdfee,
            memo: None,
            msgs: vec![msg],
            sequence: "0".to_string()
        };

        let mut input = CosmosTransaction {
            sign_data: sign_data,
            path: constants::COSMOS_PATH.to_string(),
            payment_dis: "".to_string(),
            to_dis: "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt".to_string(),
            fee_dis: "0.00075 atom".to_string(),
        };
        let cosmosTxOutput = input.sign().unwrap();
        //R3E1sN8ImA+SfRVpp4C0xNJNpQO7z5i4f2BsKdRxEPtlSousJyyAhgAY13A5VjZEIJARcX9KaWkfayfETEgALg==
    }


}
