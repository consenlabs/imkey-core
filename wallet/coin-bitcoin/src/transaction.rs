use bitcoin::{Address, PublicKey, Network, TxOut, Transaction, TxIn, OutPoint, Script, SigHashType};
use std::collections::HashMap;
use crate::error::BtcError;
use common::apdu::BtcApdu;
use common::constants::{MAX_UTXO_NUMBER, EACH_ROUND_NUMBER, DUST_THRESHOLD};
use bitcoin::util::bip32::{ExtendedPubKey, ChainCode, ChildNumber};
use bitcoin::hashes::core::str::FromStr;
use secp256k1::{Secp256k1, Message, Signature, PublicKey as PublicKey2, SecretKey};
use bitcoin::hashes::hex::FromHex;
use bitcoin::secp256k1::Secp256k1 as BitcoinSecp256k1;
use bitcoin::blockdata::{opcodes, script::Builder};
use bitcoin::consensus::{serialize, Encodable};
use bitcoin_hashes::sha256d::Hash as Hash256;
use crate::tx_signer::TxSignResult;
use bitcoin_hashes::hex::ToHex;
use ring::digest;
use mq::message::send_apdu;
use bitcoin_hashes::hash160;
use bitcoin_hashes::Hash;
use common::utility::{hex_to_bytes, bigint_to_byte_vec, secp256k1_sign};
use crate::common::{address_verify, get_xpub_data, secp256k1_sign_verify, get_address_version};
use bitcoin::util::psbt::serialize::Serialize;
use device::key_manager::{KeyManager, SE_PUB_KEY, LOCL_PRI_KEY};
use common::path::check_path_validity;

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
//    pub change_idx: i32,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub fee: i64,
    pub payment: String,
    pub to_dis: Address,
    pub from: Address,
    pub fee_dis: String,
//    pub extra_data: Vec<u8>,
}

impl BtcTransaction {
    pub fn sign_transaction(&self, network : Network, path : &String, change_idx: i32, extra_data : &Vec<u8>) -> Result<TxSignResult, BtcError>{
        //path check
        let check_result = check_path_validity(path);
        if check_result.is_err() {
            return Err(BtcError::ImkeyPathIllegal);
        }
        //check uxto number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber);
        }

        //get xpub and sign data
        let xpub_data_result = get_xpub_data(path, true);
        if xpub_data_result.is_err() {
            return Err(xpub_data_result.err().unwrap());
        }
        let xpub_data = xpub_data_result.ok().unwrap();
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();

        //get xpub data
        let sign_source_val = &xpub_data[..194];
//        let sign_result = &xpub_data[194..xpub_data.len()-4];
        let sign_result = &xpub_data[194..];

        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //use se public key verify sign
//        let se_pub_key = "04FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44".to_string();
        let sign_verify_result = secp256k1_sign_verify(hex::decode(SE_PUB_KEY.lock().unwrap().as_str()).unwrap().as_slice(),
                            hex::decode(sign_result).unwrap().as_slice(),
                            hex::decode(sign_source_val).unwrap().as_slice());
        if sign_verify_result.is_err() || !sign_verify_result.ok().unwrap() {
            return Err(BtcError::ImkeySignatureVerifyFail);
        }
        //utxo address verify
        let address_verify_result = address_verify(&self.unspents,
                                                   pub_key,
                                                   hex::decode(chain_code).unwrap().as_slice(),
                                                   network,
                                                    "btc");
        if address_verify_result.is_err() {
           return  Err(address_verify_result.err().unwrap())
        }
        let mut utxo_pub_key_vec: Vec<String> = address_verify_result.ok().unwrap();

        //calc utxo total amount
        let mut total_amount = self.get_total_amount();
        if total_amount < self.amount {
            return Err(BtcError::ImkeyInsufficientFunds);
        }

        //add send to output
        let mut txouts: Vec<TxOut> = Vec::new();
        txouts.push(self.build_send_to_output());

        //add change output
        let change_amount = self.get_change_amount();
        if change_amount > DUST_THRESHOLD {
            txouts.push(self.build_change_output(pub_key, network));
        }
        //add the op_return
        if (!extra_data.is_empty()) {
            if extra_data.len() > 80 {
                return Err(BtcError::ImkeySdkIllegalArgument);
            }
            txouts.push(self.build_op_return_output(&extra_data))
        }

        //output data serialize
        let mut tx_to_sign = Transaction {
            version: 1u32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data =  serialize(&tx_to_sign);

        //删除多余的input序列化数据
        output_serialize_data.remove(5);
        output_serialize_data.remove(5);
        //add sign type
        output_serialize_data.extend(SigHashType::All.serialize().iter());

        //set input number
        output_serialize_data.remove(4);
        output_serialize_data.insert(4, self.unspents.len() as u8);

        //add fee amount
        output_serialize_data.extend(bigint_to_byte_vec(self.fee));

        //add address version
        let address_version = get_address_version(network, self.to.to_string().as_str());
        if address_version.is_err() {
            return Err(address_version.err().unwrap());
        }
        output_serialize_data.push(address_version.ok().unwrap());

        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        //use local private key sign data
//        let private_key = hex_to_bytes("B226EA7A230A75DA23EDA981566988A96D12578FB695958BF06BD579230D6710").unwrap();
        let private_key = hex_to_bytes(LOCL_PRI_KEY.lock().unwrap().as_str()).unwrap();

        let mut output_pareper_data = secp256k1_sign(&private_key, &output_serialize_data);
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x41, 0x00, &output_pareper_data);
        for temp_str in btc_prepare_apdu_vec {
            let apdu_response = send_apdu(temp_str);
            if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
                panic!("btc output pareper apdu error");
            }
        }

        let mut lock_script_ver : Vec<Script> = Vec::new();
        let count = (self.unspents.len() - 1 )/ EACH_ROUND_NUMBER + 1;
        for i in (0..count) {
            for (x, temp_utxo) in self.unspents.iter().enumerate() {
                let mut input_data_vec = Vec::new();
                input_data_vec.push(x as u8);

                let mut temp_serialize_txin = TxIn{
                    previous_output: OutPoint {
                        txid: Hash256::from_hex(temp_utxo.txhash.as_str()).unwrap(),
                        vout: temp_utxo.vout as u32,
                    },
                    script_sig: Script::default(),
                    sequence: 0xFFFFFFFF as u32,
                    witness: vec![]
                };
                if (x >= i * EACH_ROUND_NUMBER) && (x < (i + 1) * EACH_ROUND_NUMBER) {
                    temp_serialize_txin.script_sig = Script::from(Vec::from_hex(temp_utxo.script_pubkey.as_str()).unwrap());
                }
                input_data_vec.extend_from_slice(serialize(&temp_serialize_txin).as_slice());
                let btc_perpare_apdu = BtcApdu::btc_perpare_input(0x80, &input_data_vec);
                //发送签名指令到设备并获取返回数据
                let apdu_response = send_apdu(btc_perpare_apdu);
                if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
                    panic!("btc pareper apdu error");
                }
            }
            for y in i * EACH_ROUND_NUMBER..(i+1) * EACH_ROUND_NUMBER {
                if y >= utxo_pub_key_vec.len(){
                    break;
                }
                let btc_sign_apdu = BtcApdu::btc_sign(y as u8,
                                                      SigHashType::All.as_u32() as u8,
                                                      format!("{}{}{}", path, "/", self.unspents.get(y).unwrap().derive_path).as_str());
                //发送签名指令到设备并获取签名结果
                let btc_sign_apdu_return = send_apdu(btc_sign_apdu);
                if !"9000".eq(&btc_sign_apdu_return[btc_sign_apdu_return.len() - 4 ..]) {
                    panic!("btc sign apdu error");
                }
                let btc_sign_apdu_return = &btc_sign_apdu_return[..btc_sign_apdu_return.len() - 4].to_string();
                let sign_result_str = btc_sign_apdu_return[2..btc_sign_apdu_return.len() - 2].to_string();

                lock_script_ver.push(self.build_lock_script(sign_result_str.as_str(),
                                                            utxo_pub_key_vec.get(y).unwrap()))
            }
        }
        let mut txinputs: Vec<TxIn> = Vec::new();
        for (index, unspent) in self.unspents.iter().enumerate() {
            let txin = TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.txhash).unwrap(),
                    vout: unspent.vout as u32,
                },
                script_sig: lock_script_ver.get(index).unwrap().clone(),
                sequence: 0xFFFFFFFF as u32,
                witness: vec![],
            };
            txinputs.push(txin);
        }
        tx_to_sign.input = txinputs;
        let tx_bytes = serialize(&tx_to_sign);
        println!("signature-->{:?}", tx_bytes.to_hex());
        println!("tx_hash-->{:?}", tx_to_sign.txid().to_hex());
        println!("ntxid-->{:?}", tx_to_sign.ntxid().to_hex());
        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.ntxid().to_hex(),
        })
    }

    pub fn sign_segwit_transaction(&self, network: Network, path: &String,change_idx: i32, extra_data : &Vec<u8>) -> Result<TxSignResult, BtcError> {
        //path check
        let check_result = check_path_validity(path);
        if check_result.is_err() {
            return Err(BtcError::ImkeyPathIllegal);
        }
        //check utxo number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber);
        }

        //get xpub and sign data
        let xpub_data_result = get_xpub_data(path, true);
        if xpub_data_result.is_err() {
            return Err(xpub_data_result.err().unwrap());
        }
        let xpub_data = xpub_data_result.ok().unwrap();
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();

        //get xpub data
        let sign_source_val = &xpub_data[..194];
        let sign_result = &xpub_data[194..];
        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //use se public key verify sign
//        let se_pub_key = "04FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44";
        let sign_verify_result = secp256k1_sign_verify(hex::decode(SE_PUB_KEY.lock().unwrap().as_str()).unwrap().as_slice(),
                                                       hex::decode(sign_result).unwrap().as_slice(),
                                                       hex::decode(sign_source_val).unwrap().as_slice());
        if sign_verify_result.is_err() || !sign_verify_result.ok().unwrap() {
            return Err(BtcError::ImkeySignatureVerifyFail);
        }
        //utxo address verify
        let address_verify_result = address_verify(&self.unspents,
                                                   pub_key,
                                                   hex::decode(chain_code).unwrap().as_slice(),
                                                    network,
                                                    "segwit");
        if address_verify_result.is_err() {
            return  Err(address_verify_result.err().unwrap())
        }
        let mut utxo_pub_key_vec: Vec<String> = address_verify_result.ok().unwrap();

        //calc utxo total amount
        let total_amount = self.get_total_amount();
        if total_amount < self.amount {
            return Err(BtcError::ImkeyInsufficientFunds);
        }

        //add send to output
        let mut txouts: Vec<TxOut> = Vec::new();
        txouts.push(self.build_send_to_output());

        //add change output
        let change_amount = self.get_change_amount();
        if change_amount > DUST_THRESHOLD {
            txouts.push(self.build_change_output(pub_key, network));
        }
        //add the op_return
        if (!extra_data.is_empty()) {
            if extra_data.len() > 80 {
                return Err(BtcError::ImkeySdkIllegalArgument);
            }
            txouts.push(self.build_op_return_output(extra_data));
        }

        //8.output data serialize
        let mut tx_to_sign = Transaction {
            version: 2u32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data = serialize(&tx_to_sign);

        //删除多余的input序列化数据
        output_serialize_data.remove(5);
        output_serialize_data.remove(5);

        //add sign type
        output_serialize_data.extend(SigHashType::All.serialize().iter());

        //set input number
        output_serialize_data.remove(4);
        output_serialize_data.insert(4, self.unspents.len() as u8);

        //add fee amount
        output_serialize_data.extend(bigint_to_byte_vec(self.fee));

        //add address version
        let address_version = get_address_version(network, self.to.to_string().as_str());
        if address_version.is_err() {
            return Err(address_version.err().unwrap());
        }
        output_serialize_data.push(address_version.ok().unwrap());

        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        //use local private key sign data
//        let private_key = hex_to_bytes("B226EA7A230A75DA23EDA981566988A96D12578FB695958BF06BD579230D6710").unwrap();
        let private_key = hex_to_bytes(LOCL_PRI_KEY.lock().unwrap().as_str()).unwrap();
        let mut output_pareper_data = secp256k1_sign(&private_key, &output_serialize_data);
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x00, &output_pareper_data);
        //send output pareper command  TODO
        for temp_str in btc_prepare_apdu_vec {
            let apdu_response = send_apdu(temp_str);
            if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
                panic!("btc output pareper apdu error");
            }
        }

        let mut txinputs: Vec<TxIn> = vec![];
        let mut txhash_vout_vec = vec![];
        let mut sequence_vec : Vec<u8> = vec![];
        let mut sign_apdu_vec : Vec<String> = vec![];
        for (index, unspent) in self.unspents.iter().enumerate() {
            let mut txin = TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.txhash).unwrap(),
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF as u32,
                witness: vec![],
            };

            txhash_vout_vec.extend(serialize(&txin.previous_output).iter());
            sequence_vec.extend(serialize(&txin.sequence).iter());

            let mut data : Vec<u8> = Vec::new();
            //txhash and vout
            let mut txhash_data = serialize(&txin.previous_output);
            data.extend(txhash_data.iter());

            //lock script
            let pub_key_bytes = hex::decode(utxo_pub_key_vec.get(index).unwrap()).unwrap();
            let pub_key_hash = hash160::Hash::hash(&pub_key_bytes).into_inner();
            let script_hex = format!("76a914{}88ac", hex::encode(pub_key_hash));
            let script = Script::from(hex::decode(script_hex).unwrap());
            let script_data = serialize(&script);
            data.extend(script_data.iter());

            //amount
            let mut utxo_amount = num_bigint::BigInt::from(unspent.amount).to_signed_bytes_le();
            if(utxo_amount.len() < 8){
                let temp_number = 8 - utxo_amount.len();
                for i in (0..temp_number) {
                    utxo_amount.push(0x00);
                }
            }
            data.extend(utxo_amount.iter());

            //sequence
            data.extend(hex::decode("FFFFFFFF").unwrap());

            //set length
            data.insert(0, data.len() as u8);

            //address
            let mut address_data : Vec<u8> = vec![];
            let sign_path = format!("{}{}", path, unspent.derive_path);
            address_data.push(sign_path.as_bytes().len() as u8);
            address_data.extend_from_slice(sign_path.as_bytes());

            data.extend(address_data.iter());
            if(index == self.unspents.len() - 1){
                sign_apdu_vec.push(BtcApdu::btc_segwit_sign(true, 0x01, data));
            }else{
                sign_apdu_vec.push(BtcApdu::btc_segwit_sign(false, 0x01, data));
            }

            txinputs.push(txin.clone());
        }
        tx_to_sign.input = txinputs;

        let mut txhash_vout_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x40, &txhash_vout_vec);
        let mut sequence_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x80, &sequence_vec);
        txhash_vout_prepare_apdu_vec.append(&mut sequence_prepare_apdu_vec);
        for apdu in txhash_vout_prepare_apdu_vec {
            let apdu_response = send_apdu(apdu);
            if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
                panic!("usdt txhash vout pareper apdu error");
            }
        }

        //send sign apdu
        let mut lock_script_ver : Vec<Script> = Vec::new();
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (index, wegwit_sign_apdu) in sign_apdu_vec.iter().enumerate() {
            //send sign apdu （//响应报文为签名结果，格式为L|R|S|V|，66字节，其中L为1个字节，R、S分别为32字节，V为1个字节（27或28））
            let sign_apdu_return_data = send_apdu(wegwit_sign_apdu.clone());
            if !"9000".eq(&sign_apdu_return_data[sign_apdu_return_data.len() - 4 ..]) {
                panic!("btc segwit sign apdu error");
            }
            //build signature obj
            let sign_result_vec = Vec::from_hex(&sign_apdu_return_data[2..sign_apdu_return_data.len() - 6]).unwrap();
            let mut signnture_obj =
                Signature::from_compact(sign_result_vec.as_slice()).unwrap();
            signnture_obj.normalize_s();
            //generator der sign data
            let mut sign_result_vec = signnture_obj.serialize_der().to_vec();
            //add hash type
            sign_result_vec.push(SigHashType::All.as_u32() as u8);
            witnesses.push((sign_result_vec, hex::decode(utxo_pub_key_vec.get(index).unwrap()).unwrap()));
        }

        let input_with_sigs: Result<Vec<TxIn>, _> = tx_to_sign
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let hash = hash160::Hash::hash(hex_to_bytes(utxo_pub_key_vec.get(i).unwrap()).unwrap().as_slice()).into_inner();
                let hex = format!("160014{}", hex::encode(&hash));

                Ok(TxIn {
                    script_sig: Script::from(hex::decode(hex).unwrap()),
                    witness: vec![witnesses[i].0.clone(), witnesses[i].1.clone()],
                    ..*txin
                })
            }).collect();

        tx_to_sign.input = input_with_sigs?;
        let tx_bytes = serialize(&tx_to_sign);
        println!("seralize--->{:?}", hex::encode_upper(tx_bytes.clone()));
        println!("tx_bytes--->{:?}", tx_bytes.to_hex());
        println!("txid--->{:?}", tx_to_sign.txid().to_hex());
        println!("ntxid--->{:?}", tx_to_sign.ntxid().to_hex());

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.ntxid().to_hex(),
        })
    }

    pub fn get_total_amount(&self) -> i64 {
        let mut total_amount: i64 = 0;
        for unspent in &self.unspents {
            total_amount += unspent.amount;
        }
        total_amount
    }

    pub fn get_change_amount(&self) -> i64 {
        let total_amount = self.get_total_amount();
        let change_amout = total_amount - self.amount - self.fee;
        change_amout
    }

    pub fn build_send_to_output(&self) -> TxOut {
        TxOut {
            value: self.amount as u64,
            script_pubkey: self.to.script_pubkey(),
        }
    }

    pub fn build_change_output(&self, pub_key : &str, network : Network) ->TxOut {
        //get change address
        let mut public_key_obj = PublicKey::from_str(pub_key).unwrap();
        public_key_obj.compressed = true;
        let change_addr = Address::p2pkh(&public_key_obj, network);
        //build change output
        TxOut {
            value: self.get_change_amount() as u64,
            script_pubkey: change_addr.script_pubkey(),
        }
    }

    pub fn build_op_return_output(&self, extra_data : &Vec<u8>) -> TxOut {
        let opreturn_script = Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(&extra_data[..])
            .into_script();
        TxOut {
            value: 0u64,
            script_pubkey: opreturn_script,
        }
    }

    pub fn build_lock_script(&self, signed : &str, utxo_public_key : &str) -> Script{
        let signed_vec = Vec::from_hex(&signed).unwrap();
        let mut signnture_obj = Signature::from_compact(signed_vec.as_slice()).unwrap();
        signnture_obj.normalize_s();
        let mut signed_vec = signnture_obj.serialize_der().to_vec();

        //add hash type
        signed_vec.push(SigHashType::All.as_u32() as u8);
        Builder::new().push_slice(&signed_vec)
            .push_slice(Vec::from_hex(utxo_public_key).unwrap().as_slice())
            .into_script()
    }

    pub fn get_se_pub_key(se_cert : &str) -> String{
        return "".to_string();
    }

}

#[cfg(test)]
mod tests {
    use crate::transaction::{BtcTransaction, Transaction, Utxo};
    use bitcoin::{Address, Network};
    use hex::FromHex;
    use std::collections::HashMap;
    use std::str::FromStr;

    use device::key_manager::KeyManager;
    use device::device_binding::DeviceManage;

    #[test]
    fn test_sign_transaction() {
        //设备绑定
        device_binding_test();

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
            txhash: "45ef8ac7f78b3d7d5ce71ae7934aea02f4ece1af458773f12af8ca4d79a9b531".to_string(),
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
            vout: 1,
            amount: 200000000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let utxo5 = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("mh7jj2ELSQUvRQELbn9qyA4q5nADhmJmUC").unwrap(),
            script_pubkey: "76a914118c3123196e030a8a607c22bafc1577af61497d88ac".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 4294967295,
        };
        let utxo6 = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 1,
            amount: 200000000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let utxo7 = Utxo {
            txhash: "14c67e92611dc33df31887bbc468fbbb6df4b77f551071d888a195d1df402ca9".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let utxo8 = Utxo {
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
        //        utxos.push(utxo5);
        //        utxos.push(utxo6);
        //        utxos.push(utxo7);
        //        utxos.push(utxo8);
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
//            change_idx: 53,
            amount: 799988000,
            unspents: utxos,
            fee: 10000,
            payment: "0.0001 BT".to_string(),
            to_dis: Address::from_str("3CVD68V71no5jn2UZpLLq6hASpXu1jrByt").unwrap(),
            from: Address::from_str("3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy").unwrap(),
            fee_dis: "0.00007945 BTC".to_string(),
//            extra_data: extra_data,
        };
        transaction_req_data.sign_transaction(Network::Testnet, &"m/44'/1'/0'".to_string(), 53, &extra_data);
    }

    #[test]
    fn test_sign_segwit_transaction() {

        //设备绑定
        device_binding_test();

        let extra_data = Vec::from_hex("1234").unwrap();
        let utxo = Utxo {
            txhash: "c2ceb5088cf39b677705526065667a3992c68cc18593a9af12607e057672717f".to_string(),
            vout: 0,
            amount: 50000,
            address: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo2 = Utxo {
            txhash: "9ad628d450952a575af59f7d416c9bc337d184024608f1d2e13383c44bd5cd74".to_string(),
            vout: 0,
            amount: 50000,
            address: Address::from_str("2N54wJxopnWTvBfqgAPVWqXVEdaqoH7Suvf").unwrap(),
            script_pubkey: "a91481af6d803fdc6dca1f3a1d03f5ffe8124cd1b44787".to_string(),
            derive_path: "0/1".to_string(),
            sequence: 0,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("2N9wBy6f1KTUF5h2UUeqRdKnBT6oSMh4Whp").unwrap(),
//            change_idx: 0,
            amount: 88000,
            unspents: utxos,
            fee: 10000,
            payment: "0.0001 BT".to_string(),
            to_dis: Address::from_str("3CVD68V71no5jn2UZpLLq6hASpXu1jrByt").unwrap(),
            from: Address::from_str("3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy").unwrap(),
            fee_dis: "0.00007945 BTC".to_string(),
//            extra_data: extra_data,
        };
        transaction_req_data.sign_segwit_transaction(Network::Testnet, &"m/49'/1'/0'/".to_string(), 0, &extra_data);
    }

    #[test]
    fn device_binding_test(){
        //设备绑定
        let path = "/Users/caixiaoguang/workspace/myproject/imkey-core/".to_string();
        let bind_code = "E4APZZRT".to_string();
        let mut device_manage = DeviceManage::new();
        let check_result = device_manage.bind_check(&path);
        if !"bound_this".eq(check_result.as_str()) { //如果未和本设备绑定则进行绑定操作
            let bind_result = device_manage.bind_acquire(&bind_code);
            if "5A".eq(bind_result.as_str()) {
                println!("{:?}", "binding success");
            }else {
                println!("{:?}", "binding error");
                return;
            }
        }
    }
}
