use common::cosmosapi::{CosmosTxInput, CosmosTxOutput};
use mq::message;
use serde::{Serialize, Serializer,Deserialize};
use std::collections::HashMap;
use common::utility::{sha256_hash, hex_to_bytes, secp256k1_sign};
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::Hash;
use common::apdu::CosmosApdu;
use mq::message::send_apdu;
use common::constants;
use num_bigint::BigInt;
use num_traits::Num;
use std::ops::Sub;
use crate::address::CosmosAddress;

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
    pub fn sign(mut self) -> CosmosTxOutput {
        let json = serde_json::to_vec(&self.sign_data).unwrap();
        let json = String::from_utf8(json.to_owned()).unwrap();
        println!("{}", &json);//todo sort json
        let jsonHash = sha256_hash(json.as_bytes()).to_hex();
        println!("hash:{}", &jsonHash);

        let mut sign_pack = "0120".to_string();
        sign_pack.push_str(&jsonHash);
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
//        let sign_pack_hash = sha256_hash(&sha256_hash(&sign_pack_vec.as_slice())).to_hex();
//        println!("sign_pack_hash:{}", &sign_pack_hash);

        //use local private key sign data
//        let private_key = hex_to_bytes("15A3C9A55EAE204B1CC8F2DBA25AE9A4F35793D7226E9CDE8731D58D43D6C72C").unwrap();
        let private_key = hex_to_bytes("7CD950180EDFF1C4A21270AD293A274580D20C84DE06666467F6386FB7DDA352").unwrap();//ios
        let mut prepare_data = secp256k1_sign(&private_key, &sign_pack_vec.as_slice());
        let mut prepare_data_hex = hex::encode(&prepare_data);
        println!("prepare_data_hex:{}", &prepare_data_hex);
        prepare_data.insert(0, prepare_data.len() as u8);
        prepare_data.insert(0, 0x00);
        prepare_data.extend(sign_pack_vec.iter());
        prepare_data_hex = hex::encode(&prepare_data);
        println!("prepare_data_hex:{}", &prepare_data_hex);

        let select_apdu = CosmosApdu::select_applet();
        send_apdu(select_apdu);

        let prepare_apdus = CosmosApdu::prepare_sign(prepare_data);

        for apdu in prepare_apdus {
            println!("prepare_apdu:{}", &apdu);
            send_apdu(apdu);
        }

        let sign_apdu = CosmosApdu::sign_digest(constants::COSMOS_PATH);
        println!("sign_apdu:{}", &sign_apdu);

        let sign_result = send_apdu(sign_apdu);
        println!("sign_result:{}", &sign_result);

        let r_hex:String = sign_result.chars().skip(2).take(64).collect();
        let s_hex:String = sign_result.chars().skip(66).take(64).collect();
        println!("r_hex:{}", &r_hex);
        println!("s_hex:{}", &s_hex);

        let mut s_big = BigInt::from_str_radix(&s_hex,16).unwrap();
        let half_curve_order = BigInt::from_str_radix("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0",16).unwrap();
        let curve_n = BigInt::from_str_radix("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0",16).unwrap();
        if s_big.gt(&half_curve_order) {
            s_big = curve_n.sub(s_big);
        }
        let mut sLow = s_big.to_hex();
        while sLow.len() <64 {
            sLow.insert_str(0,"0");
        }

        let signature = r_hex + &sLow.to_uppercase();
        println!("signature:{}", &signature);
        let sign_bytes = hex::decode(signature).unwrap();

        let sign_base64 = base64::encode(&sign_bytes);
        println!("sign_base64:{}", &sign_base64);

        let pub_key = CosmosAddress::get_pub_key(&self.path).unwrap();
        let pub_key = hex::decode(pub_key).unwrap();
        let pub_key = base64::encode(&pub_key);
        println!("pub_key:{}", &pub_key);

//        account_number: self.sign_data.unwrap().account_number.to_string(),
//        pubkey: Pubkey {
//            ttype: "tendermint/PubKeySecp256k1".to_string(),
//            value: "".to_string()
//        },
//        pubKey: "".to_string(),
//        sequence: "".to_string()
        let std_signature = StdSignature{
            account_number: self.sign_data.account_number.to_string(),
            pub_key: Pubkey {
                ttype: "tendermint/PubKeySecp256k1".to_string(),
                value: pub_key.to_string()
            },
            sequence: self.sign_data.sequence.to_string(),
            signature:sign_base64,
        };

//        let self_clone = self.clone();

//        let feeCopy = self.sign_data.fee;
//        let fee = self.sign_data.fee
        let std_tx = StdTx{
            fee: self.sign_data.fee,
            signatures: vec![std_signature],
            msg: self.sign_data.msgs,
            memo: self.sign_data.memo,
        };

        let json = serde_json::to_vec(&std_tx).unwrap();
        let json = String::from_utf8(json.to_owned()).unwrap();
        println!("{}", &json);//todo sort json


        let ouput = CosmosTxOutput {
            signature: json.to_string(),
            tx_hash: "".to_string(),
        };
        ouput
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{CosmosTransaction, SignData, StdFee, Coin, Msg, MsgValue};
    use common::constants;
    use common::cosmosapi::CosmosTxInput;
    use common::utility::{secp256k1_sign, hex_to_bytes};

    #[test]
    fn test_hex_bytes() {}

    #[test]
    fn test_ecsign() {
        let sign_pack = hex_to_bytes("0120D560F6EAB74C1D26DD5FAB27B9F700F4C371AC76A82E9A2E534269322D129E2F070008000900").unwrap();
        let private_key = hex_to_bytes("F85B222058BBEFFF888AAF7AD1D08B0C9C5FF719027F7DB69859B72A17B28749").unwrap();
        let mut prepare_data = secp256k1_sign(&private_key, &sign_pack.as_slice());
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
        let cosmosTxOutput = input.sign();
//        println!("cosmosTxOutput:{}", cosmosTxOutput);
    }


}
