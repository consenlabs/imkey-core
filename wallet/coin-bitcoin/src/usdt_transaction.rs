use bitcoin::{Network, TxOut, Transaction, TxIn, OutPoint, Script, SigHashType, BitcoinHash};
use crate::error::BtcError;
use common::apdu::{BtcApdu, ApduCheck};
use common::constants::{MAX_UTXO_NUMBER, EACH_ROUND_NUMBER, MIN_NONDUST_OUTPUT};
use secp256k1::{Signature};
use bitcoin::hashes::hex::FromHex;
use bitcoin::blockdata::{opcodes, script::Builder};
use bitcoin::consensus::{serialize};
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::hex::ToHex;
use mq::message::send_apdu;
use bitcoin_hashes::hash160;
use bitcoin_hashes::Hash;
use crate::transaction::{BtcTransaction, Utxo};
use common::utility::{hex_to_bytes, bigint_to_byte_vec, secp256k1_sign};
use crate::common::{address_verify, get_xpub_data, secp256k1_sign_verify, get_address_version, TxSignResult};
use bitcoin::util::psbt::serialize::Serialize;
use device::device_binding::{KEY_MANAGER};
use common::path::check_path_validity;
use crate::Result;

impl BtcTransaction {
    pub fn sign_omni_transaction(&self, network : Network, path : &str, property_id : i32) -> Result<TxSignResult>{
        //path check
        check_path_validity(path)?;
        let mut path_str = path.to_string();
        if !path.ends_with("/") {
            path_str = format!("{}{}", path_str, "/");
        }
        //check uxto number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber.into());
        }

        //check change amount
        if self.amount - self.fee < MIN_NONDUST_OUTPUT {
            return Err(BtcError::ImkeyAmountLessThanMinimum.into());
        }

        //get xpub and sign data
        let xpub_data = get_xpub_data(path_str.as_str(), true)?;
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();
        //get xpub data
        let sign_source_val = &xpub_data[..194];
        let sign_result = &xpub_data[194..];
        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //use se public key verify sign
        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let sign_verify_result = secp256k1_sign_verify(&key_manager_obj.se_pub_key.as_slice(),
                                                       hex::decode(sign_result).unwrap().as_slice(),
                                                       hex::decode(sign_source_val).unwrap().as_slice());
        if sign_verify_result.is_err() || !sign_verify_result.ok().unwrap() {
            return Err(BtcError::ImkeySignatureVerifyFail.into());
        }

        //utxo address verify
        let utxo_pub_key_vec = address_verify(&self.unspents,
                                                   pub_key,
                                                   hex::decode(chain_code).unwrap().as_slice(),
                                                   network,
                                                   "btc")?;

        //calc utxo total amount
        let total_amount = self.get_total_amount();

        //add change output
        let mut txouts: Vec<TxOut> = Vec::new();
        let change_amount = total_amount - MIN_NONDUST_OUTPUT - self.fee;
        let receiver_address = &self.unspents.get(0).unwrap().address;
        let txout_send_output = TxOut {
            value: change_amount as u64,
            script_pubkey: receiver_address.script_pubkey(),
        };
        txouts.push(txout_send_output);

        //add send to output
        let txout_change_output = TxOut {
            value: MIN_NONDUST_OUTPUT as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_change_output);

        //add omni output
        txouts.push(self.build_omni_output(property_id, self.amount));

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


        //添加地址版本
        let address_version = get_address_version(network, self.to.to_string().as_str())?;
        output_serialize_data.push(address_version);

        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        //use local private key sign data
        let mut output_pareper_data = secp256k1_sign(&key_manager_obj.pri_key, &output_serialize_data)?;
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        //send output prepare command
        let omni_prepare_apdu_str = BtcApdu::omni_prepare_data(0x00, output_pareper_data);
        let apdu_response = send_apdu(omni_prepare_apdu_str);
        ApduCheck::checke_response(&apdu_response)?;
        let mut lock_script_ver : Vec<Script> = vec![];
        let count = (self.unspents.len() - 1 )/ EACH_ROUND_NUMBER + 1;
        for i in (0..count) {
            for (x, temp_utxo) in self.unspents.iter().enumerate() {
                let mut input_data_vec = vec![];
                input_data_vec.push(x as u8);

                let mut temp_serialize_txin = TxIn{
                    previous_output: OutPoint {
                        txid: Hash256::from_hex(temp_utxo.txhash.as_str())?,
                        vout: temp_utxo.vout as u32,
                    },
                    script_sig: Script::default(),
                    sequence: 0xFFFFFFFF as u32,
                    witness: vec![]
                };
                if (x >= i * EACH_ROUND_NUMBER) && (x < (i + 1) * EACH_ROUND_NUMBER) {
                    temp_serialize_txin.script_sig = Script::from(Vec::from_hex(temp_utxo.script_pubkey.as_str())?);
                }
                input_data_vec.extend_from_slice(serialize(&temp_serialize_txin).as_slice());
                let btc_perpare_apdu = BtcApdu::btc_perpare_input(0x80, &input_data_vec);
                //发送签名数据到设备并获取返回数据
                let apdu_response = send_apdu(btc_perpare_apdu);
                ApduCheck::checke_response(&apdu_response)?;
            }
            for y in i * EACH_ROUND_NUMBER..(i+1)*EACH_ROUND_NUMBER {
                if y >= utxo_pub_key_vec.len(){
                    break;
                }
                let btc_sign_apdu = BtcApdu::btc_sign(y as u8,
                                                      SigHashType::All.as_u32() as u8,
                                                      format!("{}{}", path_str, self.unspents.get(y).unwrap().derive_path).as_str());
                //发送签名指令到设备并获取签名结果
                let btc_sign_apdu_return = send_apdu(btc_sign_apdu);
                ApduCheck::checke_response(&apdu_response)?;
                let sign_result_str = btc_sign_apdu_return[2..btc_sign_apdu_return.len() - 6].to_string();

                lock_script_ver.push(self.build_lock_script(sign_result_str.as_str(),
                                                            utxo_pub_key_vec.get(y).unwrap())?);
            }
        }
        let mut txinputs: Vec<TxIn> = vec![];
        for (index, unspent) in self.unspents.iter().enumerate() {
            let txin = TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.txhash)?,
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
            wtx_id: tx_to_sign.txid().to_hex(),
        })
    }

    pub fn sign_omni_segwit_transaction(&self, network: Network, path: &str, property_id : i32) -> Result<TxSignResult> {
        //path check
        check_path_validity(path)?;
        let mut path_str = path.to_string();
        if !path.ends_with("/") {
            path_str = format!("{}{}", path_str, "/");
        }
        //check uxto number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber.into());
        }
        let change_amount = self.get_total_amount() - self.fee - MIN_NONDUST_OUTPUT;
        //check change amount
        if change_amount < MIN_NONDUST_OUTPUT {
            return Err(BtcError::ImkeyAmountLessThanMinimum.into());
        }

        //get xpub and sign data
        let xpub_data = get_xpub_data(path_str.as_str(), true)?;
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();

        //get xpub data
        let sign_source_val = &xpub_data[..194];
        let sign_result = &xpub_data[194..];
        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //use se public key verify sign
        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let sign_verify_result = secp256k1_sign_verify(&key_manager_obj.se_pub_key.as_slice(),
                                                       hex::decode(sign_result).unwrap().as_slice(),
                                                       hex::decode(sign_source_val).unwrap().as_slice());
        if sign_verify_result.is_err() || !sign_verify_result.ok().unwrap() {
            return Err(BtcError::ImkeySignatureVerifyFail.into());
        }

        //utxo address verify
        let utxo_pub_key_vec = address_verify(&self.unspents,
                                                   pub_key,
                                                   hex::decode(chain_code).unwrap().as_slice(),
                                                   network,
                                                   "segwit")?;

        //5.add change output
        let mut txouts: Vec<TxOut> = vec![];
        let receiver_address = &self.unspents.get(0).unwrap().address;
        let txout_send_output = TxOut {
            value: change_amount as u64,
            script_pubkey: receiver_address.script_pubkey(),
        };
        txouts.push(txout_send_output);

        //6.add send to output
        let txout_change_output = TxOut {
            value: MIN_NONDUST_OUTPUT as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_change_output);

        //add omni output
        txouts.push(self.build_omni_output(property_id, self.amount));

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
        //添加地址版本
        let address_version = get_address_version(network, self.to.to_string().as_str())?;
        output_serialize_data.push(address_version);
        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);
        //use local private key sign data
        let mut output_pareper_data = secp256k1_sign(&key_manager_obj.pri_key, &output_serialize_data)?;
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x00, &output_pareper_data);
        for temp_str in btc_prepare_apdu_vec {
            let apdu_response = send_apdu(temp_str);
            ApduCheck::checke_response(&apdu_response)?;
        }

        let mut txinputs: Vec<TxIn> = vec![];
        let mut txhash_vout_vec = vec![];
        let mut sequence_vec : Vec<u8> = vec![];
        let mut sign_apdu_vec : Vec<String> = vec![];
        for (index, unspent) in self.unspents.iter().enumerate() {
            let txin = TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.txhash)?,
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF as u32,
                witness: vec![],
            };

            txhash_vout_vec.extend(serialize(&txin.previous_output).iter());
            sequence_vec.extend(serialize(&txin.sequence).iter());

            let mut data : Vec<u8> = vec![];
            //txhash and vout
            let txhash_data = serialize(&txin.previous_output);
            data.extend(txhash_data.iter());

            //lock script
            let pub_key_bytes = hex::decode(utxo_pub_key_vec.get(index).unwrap())?;
            let pub_key_hash = hash160::Hash::hash(&pub_key_bytes).into_inner();
            let script_hex = format!("76a914{}88ac", hex::encode(pub_key_hash));
            let script = Script::from(hex::decode(script_hex)?);
            let script_data = serialize(&script);
            data.extend(script_data.iter());

            //amount
            let mut utxo_amount = num_bigint::BigInt::from(unspent.amount).to_signed_bytes_le();
            if(utxo_amount.len() < 8){
                let temp_number = 8 - utxo_amount.len();
                for _i in (0..temp_number) {
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
            let sign_path = format!("{}{}", path_str, unspent.derive_path);
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

        let mut txhash_vout_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x40, &txhash_vout_vec);
        let mut sequence_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x80, &sequence_vec);
        txhash_vout_prepare_apdu_vec.append(&mut sequence_prepare_apdu_vec);
        for apdu in txhash_vout_prepare_apdu_vec {
            let apdu_response = send_apdu(apdu);
            ApduCheck::checke_response(&apdu_response)?;
        }

        //send sign apdu
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (index, segwit_sign_apdu) in sign_apdu_vec.iter().enumerate() {
            //send sign apdu
            let sign_apdu_return_data = send_apdu(segwit_sign_apdu.clone());
            ApduCheck::checke_response(&sign_apdu_return_data)?;
            //build signature obj
            let sign_result_vec = Vec::from_hex(&sign_apdu_return_data[2..sign_apdu_return_data.len() - 6]).unwrap();
            let mut temp_signnture_obj =
                Signature::from_compact(sign_result_vec.as_slice())?;
            temp_signnture_obj.normalize_s();
            //generator der sign data
            let mut sign_result_vec = temp_signnture_obj.serialize_der().to_vec();
            //add hash type
            sign_result_vec.push(SigHashType::All.as_u32() as u8);
            witnesses.push((sign_result_vec, hex::decode(utxo_pub_key_vec.get(index).unwrap())?));
        }

        let input_with_sigs: Result<Vec<TxIn>> = tx_to_sign
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
        println!("ntxid--->{:?}", tx_to_sign.bitcoin_hash().to_hex());

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.bitcoin_hash().to_hex(),
        })
    }

    pub fn build_omni_output(&self, property_id : i32, amount : i64, ) -> TxOut {
        let mut property_id_bytes = num_bigint::BigInt::from(property_id).to_signed_bytes_le();
        if(property_id_bytes.len() < 4){
            let temp_number = 4 - property_id_bytes.len();
            for _i in (0..temp_number) {
                property_id_bytes.push(0x00);
            }
        }
        property_id_bytes.reverse();
        let mut omni_data = hex::decode("6f6d6e6900000000").unwrap();
        omni_data.extend(property_id_bytes.iter());
        let mut amount_bytes = num_bigint::BigInt::from(amount).to_signed_bytes_le();
        if(amount_bytes.len() < 8){
            let temp_number = 8 - amount_bytes.len();
            for _i in (0..temp_number) {
                amount_bytes.push(0x00);
            }
        }
        amount_bytes.reverse();
        omni_data.extend(amount_bytes.iter());
        TxOut {
            value: 0u64,
            script_pubkey: Builder::new().push_opcode(opcodes::all::OP_RETURN).push_slice(&omni_data[..]).into_script(),
        }
    }

}

#[cfg(test)]
mod tests {
    use bitcoin::{Address, Network};
    use hex::FromHex;
    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::transaction::{BtcTransaction, Utxo};

    use device::key_manager::KeyManager;
    use device::device_binding::DeviceManage;

    #[test]
    fn test_sign_transaction() {
        //设备绑定
        device_binding_test();

        let utxo = Utxo {
            txhash: "0dd195c815c5086c5995f43a0c67d28344ae5fa130739a5e03ef40fea54f2031".to_string(),
            vout: 0,
            amount: 14824854,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            amount: 10050000000,
            unspents: utxos,
            fee: 4000,
            payment: "100 USDT".to_string(),
            to_dis: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            from: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            fee_dis: "0.0004 BTC".to_string(),
        };
        transaction_req_data.sign_omni_transaction(Network::Testnet, &"m/44'/1'/0'".to_string(), 31);
    }

    #[test]
    fn test_sign_segwit_transaction() {
        //设备绑定
        device_binding_test();

        let extra_data = Vec::from_hex("1234").unwrap();
        let utxo = Utxo {
            txhash: "9baf6fd0e560f9f199f4879c23cb73b9c4affb54a1cfdbacb85687efa89f4c78".to_string(),
            vout: 1,
            amount: 21863396,
            address: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            amount: 10000000000,
            unspents: utxos,
            fee: 4000,
            payment: "100 USDT".to_string(),
            to_dis: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            from: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            fee_dis: "0.0004 BTC".to_string(),
        };
        transaction_req_data.sign_omni_segwit_transaction(Network::Testnet, &"m/49'/1'/0'/".to_string(), 31);
    }

    #[test]
    fn test_segwit_transaction_8utxo() {
        //设备绑定
        device_binding_test();

        let extra_data = Vec::from_hex("1234").unwrap();
        let utxo = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };
        let utxo2 = Utxo {
            txhash: "45ef8ac7f78b3d7d5ce71ae7934aea02f4ece1af458773f12af8ca4d79a9b531".to_string(),
            vout: 1,
            amount: 200000000,
            address: Address::from_str("3JmreiUEKn8P3SyLYmZ7C1YCd4r2nFy3Dp").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo3 = Utxo {
            txhash: "14c67e92611dc33df31887bbc468fbbb6df4b77f551071d888a195d1df402ca9".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("3JmreiUEKn8P3SyLYmZ7C1YCd4r2nFy3Dp").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo4 = Utxo {
            txhash: "117fb6b85ded92e87ee3b599fb0468f13aa0c24b4a442a0d334fb184883e9ab9".to_string(),
            vout: 1,
            amount: 200000000,
            address: Address::from_str("3JmreiUEKn8P3SyLYmZ7C1YCd4r2nFy3Dp").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo5 = Utxo {
            txhash: "013adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };
        let utxo6 = Utxo {
            txhash: "023adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };
        let utxo7 = Utxo {
            txhash: "033adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };
        let utxo8 = Utxo {
            txhash: "043adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 200000000,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);
        utxos.push(utxo3);
        utxos.push(utxo4);
        utxos.push(utxo5);
        utxos.push(utxo6);
        utxos.push(utxo7);
        utxos.push(utxo8);
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("3PGEDofNu6aJ3KfgK9PHGt3EW3oZK5qY1a").unwrap(),
            amount: 750000000,
            unspents: utxos,
            fee: 502130,
            payment: "0.0001 USDT".to_string(),
            to_dis: Address::from_str("3CVD68V71no5jn2UZpLLq6hASpXu1jrByt").unwrap(),
            from: Address::from_str("3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy").unwrap(),
            fee_dis: "0.00007945 BTC".to_string(),
        };
        transaction_req_data.sign_omni_segwit_transaction(Network::Bitcoin, &"m/49'/0'/0'/".to_string(), 31);
    }

    #[test]
    fn test1(){
        //设备绑定
        device_binding_test();

        let extra_data = Vec::from_hex("1234").unwrap();
        let utxo = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 10000112345678,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("3PGEDofNu6aJ3KfgK9PHGt3EW3oZK5qY1a").unwrap(),
            amount: 345678,
            unspents: utxos,
            fee: 502130,
            payment: "0.0001 USDT".to_string(),
            to_dis: Address::from_str("3CVD68V71no5jn2UZpLLq6hASpXu1jrByt").unwrap(),
            from: Address::from_str("3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy").unwrap(),
            fee_dis: "0.00007945 BTC".to_string(),
        };
        transaction_req_data.sign_omni_segwit_transaction(Network::Bitcoin, &"m/49'/0'/0'/".to_string(), 31);
    }

    #[test]
    fn device_binding_test(){
        //设备绑定
        let path = "/Users/caixiaoguang/workspace/myproject/imkey-core/".to_string();
        let bind_code = "E4APZZRT".to_string();
        // let mut device_manage = DeviceManage::new();
        let check_result = DeviceManage::bind_check(&path).unwrap_or_default();
        if !"bound_this".eq(check_result.as_str()) { //如果未和本设备绑定则进行绑定操作
            let bind_result = DeviceManage::bind_acquire(&bind_code).unwrap_or_default();
            if "5A".eq(bind_result.as_str()) {
                println!("{:?}", "binding success");
            }else {
                println!("{:?}", "binding error");
                return;
            }
        }
    }
}
