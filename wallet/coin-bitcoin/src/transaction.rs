use crate::error::BtcError;
use bitcoin::blockdata::{opcodes, script::Builder};
use bitcoin::consensus::serialize;
use bitcoin::hashes::core::str::FromStr;
use bitcoin::hashes::hex::FromHex;
use bitcoin::secp256k1::Secp256k1 as BitcoinSecp256k1;
use bitcoin::util::bip32::{ChainCode, ChildNumber, ExtendedPubKey};
use bitcoin::{Address, Network, PublicKey, Transaction, TxOut};
use common::apdu::BtcApdu;
use common::constants::MAX_UTXO_NUMBER;
use device::hid_api;
use ring::digest;
use secp256k1::{Message, PublicKey as PublicKey2, Secp256k1, Signature};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Utxo {
    pub txhash: String,
    pub vout: i32,
    pub amount: i64,
    pub address: Address,
    pub script_pubkey: String,
    pub derive_path: String,
    pub sequence: i64,
}

pub struct BtcTransaction {
    pub to: Address,
    pub change_idx: i32,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub fee: i64,
    pub payment: String,
    pub to_dis: Address,
    pub from: Address,
    pub fee_dis: String,
    pub extra_data: Vec<u8>,
}

impl BtcTransaction {
    pub fn sign_transaction(&self, network: Network, path: &String) -> Result<(), BtcError> {
        //1.path check

        //2.uxto number check
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber);
        }
        //3.get main public key(xpub)
        let select_btc_apdu: String = BtcApdu::select_applet();
        let hid_device = hid_api::connect();
        let apdu_response = hid_api::send(&hid_device, &select_btc_apdu);
        let get_xpub_apdu = BtcApdu::get_xpub(path.as_str(), false);
        let xpub_data = hid_api::send(&hid_device, &get_xpub_apdu);
        //解析XPUB数据
        let sign_data = &xpub_data[..194];
        let sign_result = &xpub_data[194..];
        println!("签名原值-->{:?}", sign_data);
        println!("签名结果-->{:?}", sign_result);
        let pub_key = &sign_data[..130];
        let chain_code = &sign_data[130..];
        println!("公钥-->{:?}", pub_key);
        println!("链码-->{:?}", chain_code);
        //通过SE公钥验证签名
        let secp = Secp256k1::new();
        let se_pub_key = "04FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44";
        let se_pub_key_obj = PublicKey2::from_str(se_pub_key).unwrap();

        //对签名原值进行SHA256
        let message_hash = digest::digest(
            &digest::SHA256,
            Vec::from_hex(sign_data).unwrap().as_slice(),
        );
        println!("hash-->{}", hex::encode_upper(message_hash.as_ref()));
        let message_obj = Message::from_slice(message_hash.as_ref()).unwrap();
        //生成签名结果对象
        let mut sig = Signature::from_der(Vec::from_hex(sign_result).unwrap().as_slice()).unwrap();
        sig.normalize_s();
        let verify_result = secp.verify(&message_obj, &sig, &se_pub_key_obj).is_ok();
        println!("签名验证结果->{}", verify_result);
        if !verify_result {
            return Err(BtcError::ImkeySignatureVerifyFail);
        }

        let mut utxo_pub_key_vec: Vec<String> = Vec::new();
        for utxo in &self.unspents {
            //4.get utxo public key
            let mut temp_public_Key = PublicKey::from_str(pub_key).unwrap();
            temp_public_Key.compressed = true;
            let temp_chain_code_vec = Vec::from_hex(chain_code).unwrap();
            let temp_chain_code = ChainCode::from(temp_chain_code_vec.as_slice());
            let mut pk = ExtendedPubKey {
                network: network,
                depth: 0,
                parent_fingerprint: Default::default(),
                child_number: ChildNumber::from_normal_idx(0).unwrap(),
                public_key: temp_public_Key,
                chain_code: temp_chain_code,
            };

            let bitcoin_secp = BitcoinSecp256k1::new();
            let index_number_vec: Vec<&str> = utxo.derive_path.as_str().split('/').collect();
            for index_number in index_number_vec {
                println!("{}", index_number);
                let test_chain_number =
                    ChildNumber::from_normal_idx(index_number.parse().unwrap()).unwrap();
                pk = pk.ckd_pub(&bitcoin_secp, test_chain_number).unwrap();
                println!("{:?}", pk.to_string());
                println!("{:?}", pk.public_key.to_string().to_uppercase());
            }
            //验证地址
            let temp_address = Address::p2pkh(
                &PublicKey::from_str(pk.public_key.to_string().as_str()).unwrap(),
                network,
            )
            .to_string();
            let temp_utxo_address = utxo.address.to_string();
            println!("{}", temp_address);
            println!("{}", temp_utxo_address);
            if !temp_address.eq(&temp_utxo_address) {
                return Err(BtcError::ImkeyAddressMismatchWithPath);
            }
            utxo_pub_key_vec.push(pk.public_key.to_string());
        }

        //计算UTXO总金额
        let mut total_amount: i64 = 0;
        for unspent in &self.unspents {
            total_amount += unspent.amount;
        }
        if total_amount < self.amount {
            return Err(BtcError::ImkeyInsufficientFunds);
        }

        //5.add send to output
        let mut txouts: Vec<TxOut> = Vec::new();
        let txout_send_output = TxOut {
            value: self.amount as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_send_output);
        //6.add change output
        let change_amount = total_amount - self.amount - self.fee;
        //获取找零地址
        let get_xpub_apdu = BtcApdu::get_xpub(
            format!("{}{}{}", path, "/1/", self.change_idx).as_str(),
            true,
        );
        let xpub_data = hid_api::send(&hid_device, &get_xpub_apdu);
        let pub_key = &sign_data[..130];
        let mut temp_public_Key = PublicKey::from_str(pub_key).unwrap();
        temp_public_Key.compressed = true;
        let change_addr = Address::p2pkh(&temp_public_Key, network);

        let txout_change_output = TxOut {
            value: self.amount as u64,
            script_pubkey: change_addr.script_pubkey(),
        };
        txouts.push(txout_change_output);
        //7.add the op_return
        if (!self.extra_data.is_empty()) {
            if self.extra_data.len() > 80 {
                return Err(BtcError::ImkeySdkIllegalArgument);
            }
            let opreturn_script = Builder::new()
                .push_opcode(opcodes::all::OP_RETURN)
                .push_slice(&self.extra_data[..])
                .into_script();
            let txout_opreturn = TxOut {
                value: 0u64,
                script_pubkey: opreturn_script,
            };
            txouts.push(txout_opreturn);
        }
        //8.output data serialize
        let tx_to_sign = Transaction {
            version: 1u32,
            lock_time: 0u32,
            input: Vec::new(),
            output: txouts,
        };
        let output_serialize_data = serialize(&tx_to_sign);
        println!("{:?}", hex::encode_upper(output_serialize_data));

        //9.

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{BtcTransaction, Transaction, Utxo};
    use bitcoin::{Address, Network};
    use hex::FromHex;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_sign_transaction() {
        //        let mut extra_data = HashMap::new();
        //        extra_data.insert("opReturn".to_string(), "0200000080a10bc28928f4c17a287318125115c3f098ed20a8237d1e8e4125bc25d1be99752adad0a7b9ceca853768aebb6965eca126a62965f698a0c1bc43d83db632ad7f717276057e6012afa99385".to_string());
        let extra_data = Vec::from_hex("0200000080a10bc28928f4c17a287318125115c3f098ed20a8237d1e8e4125bc25d1be99752adad0a7b9ceca853768aebb6965eca126a62965f698a0c1bc43d83db632ad7f717276057e6012afa99385").unwrap();
        let utxo = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("mh7jj2ELSQUvRQELbn9qyA4q5nADhmJmUC").unwrap(),
            script_pubkey: "76a914118c3123196e030a8a607c22bafc1577af61497d88ac".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 4294967295,
        };
        let utxo2 = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 1,
            amount: 200000000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let utxo3 = Utxo {
            txhash: "14c67e92611dc33df31887bbc468fbbb6df4b77f551071d888a195d1df402ca9".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let utxo4 = Utxo {
            txhash: "117fb6b85ded92e87ee3b599fb0468f13aa0c24b4a442a0d334fb184883e9ab9".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);
        utxos.push(utxo3);
        utxos.push(utxo4);
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            change_idx: 53,
            amount: 799988000,
            unspents: utxos,
            fee: 10000,
            payment: "0.0001 BT".to_string(),
            to_dis: Address::from_str("3CVD68V71no5jn2UZpLLq6hASpXu1jrByt").unwrap(),
            from: Address::from_str("3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy").unwrap(),
            fee_dis: "0.00007945 BTC".to_string(),
            extra_data: extra_data,
        };
        transaction_req_data.sign_transaction(Network::Testnet, &"m/44'/1'/0'".to_string());
    }
}
