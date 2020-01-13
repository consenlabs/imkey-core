use bitcoin::{Address, PublicKey, Network, TxOut, Transaction, TxIn, OutPoint, Script, SigHashType};
use std::collections::HashMap;
use crate::error::BtcError;
use common::apdu::BtcApdu;
use common::constants::{MAX_UTXO_NUMBER, EACH_ROUND_NUMBER};
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
use common::utility::hex_to_bytes;

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
    pub property_id: i32,
}

impl BtcTransaction {
    pub fn sign_transaction(&self, network : Network, path : &String) -> Result<TxSignResult, BtcError>{
        //1.path校验

        //2.uxto数量检查
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber);
        }

        //检查找零金额是否小于最小值
        if self.amount - self.fee < 546 {
            return Err(BtcError::ImkeyAmountLessThanMinimum);
        }

        //3.get main public key(xpub)
        send_apdu(BtcApdu::select_applet());
        let xpub_data = send_apdu(BtcApdu::get_xpub(path.as_str(), false));



        //解析XPUB数据
        let sign_source_val = &xpub_data[..194];
        let sign_result = &xpub_data[194..];

        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //通过SE公钥验证签名
        let secp = Secp256k1::new();
        let se_pub_key = "04FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44";
        let se_pub_key_obj = PublicKey2::from_str(se_pub_key).unwrap();

        //对签名原值进行SHA256
        let message_hash = digest::digest(
            &digest::SHA256,
            Vec::from_hex(sign_source_val).unwrap().as_slice(),
        );
        let message_obj = Message::from_slice(message_hash.as_ref()).unwrap();
        //生成签名结果对象
        let mut sig = Signature::from_der(Vec::from_hex(sign_result).unwrap().as_slice()).unwrap();
        sig.normalize_s();
        let verify_result = secp.verify(&message_obj, &sig, &se_pub_key_obj).is_ok();
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
                let test_chain_number =
                    ChildNumber::from_normal_idx(index_number.parse().unwrap()).unwrap();
                pk = pk.ckd_pub(&bitcoin_secp, test_chain_number).unwrap();
            }
            //验证地址
            let temp_address = Address::p2pkh(
                &PublicKey::from_str(pk.public_key.to_string().as_str()).unwrap(),
                network,
            ).to_string();
            let temp_utxo_address = utxo.address.to_string();
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

        //6.add change output
        let mut txouts: Vec<TxOut> = Vec::new();
        let change_amount = total_amount - 546 - self.fee;
        let receiver_address = &self.unspents.get(0).unwrap().address;

        //5.add send to output
        let txout_send_output = TxOut {
            value: change_amount as u64,
            script_pubkey: receiver_address.script_pubkey(),
        };
        txouts.push(txout_send_output);//找零信息

        let txout_change_output = TxOut {
            value: 546 as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_change_output);//交易信息

        //budile omni data
        let mut property_id_bytes = num_bigint::BigInt::from(self.property_id).to_signed_bytes_le();
        if(property_id_bytes.len() < 4){
            let temp_number = 4 - property_id_bytes.len();
            for i in (0..temp_number) {
                property_id_bytes.push(0x00);
            }
        }
        property_id_bytes.reverse();
        let mut omni_data = hex::decode("6f6d6e6900000000").unwrap();
        omni_data.extend(property_id_bytes.iter());
        let mut amount_bytes = num_bigint::BigInt::from(self.amount).to_signed_bytes_le();
        if(amount_bytes.len() < 8){
            let temp_number = 8 - amount_bytes.len();
            for i in (0..temp_number) {
                amount_bytes.push(0x00);
            }
        }
        amount_bytes.reverse();
        omni_data.extend(amount_bytes.iter());
        println!("@@@@@@{}", hex::encode_upper(omni_data.clone()));
        let omni_output = TxOut {
            value: 0u64,
            script_pubkey: Builder::new().push_opcode(opcodes::all::OP_RETURN).push_slice(&omni_data[..]).into_script(),
        };

        println!("omni data-->{:?}", serialize(&omni_output).to_hex());
        txouts.push(omni_output);

        //8.output data serialize
        let mut tx_to_sign = Transaction {
            version: 1u32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data =  serialize(&tx_to_sign);

//        //先手动填入omni output TODO==================
//        output_serialize_data.extend(hex::decode("00000000000000002C6A146F6D6E69000000000000001F000000025706D4806A146F6D6E69000000000000001F000000025706D48000000000010000000000000000000FA06F").unwrap().iter());


        println!("AAAAA->{:?}", hex::encode_upper(output_serialize_data.as_slice()));
        //删除多余的input序列化数据
        output_serialize_data.remove(5);
        output_serialize_data.remove(5);
        //增加签名类型
        output_serialize_data.extend_from_slice(hex::decode("01000000").unwrap().as_slice());
        //设置input数量
        output_serialize_data.remove(4);
        output_serialize_data.insert(4, self.unspents.len() as u8);
        //添加旷工费用 0000000000002710 TODO
        output_serialize_data.extend_from_slice(hex::decode("0000000000000FA0").unwrap().as_slice());
        //添加地址版本 TODO
        output_serialize_data.extend_from_slice(hex::decode("6F").unwrap().as_slice());
        println!("signedHez-->{:?}", hex::encode_upper(output_serialize_data.clone()));
        //
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        //对序列化数据进行SHA256计算
        let message_hash = digest::digest(&digest::SHA256, output_serialize_data.as_slice());
        let message_hash = digest::digest(&digest::SHA256, message_hash.as_ref());

        //使用本地私钥对数据进行签名 TODO
        let private_key = "B226EA7A230A75DA23EDA981566988A96D12578FB695958BF06BD579230D6710";
        let temp_secret_key = SecretKey::from_slice(hex::decode(private_key).unwrap().as_slice());
        let message_data = Message::from_slice(message_hash.as_ref()).unwrap();
        let secp = Secp256k1::new();
        let sign_result = secp.sign(&message_data, &temp_secret_key.unwrap()).serialize_der().to_vec();
        let mut  sign_data = Vec::new();
        sign_data.push(0x00 as u8);
        sign_data.push(sign_result.len() as u8);
        sign_data.extend_from_slice(sign_result.as_slice());

        sign_data.extend_from_slice(output_serialize_data.as_slice());
        println!("package data-->{:?}", hex::encode_upper(sign_data.clone()));
        let omni_prepare_apdu_str = BtcApdu::omni_prepare_data(0x00, sign_data);
        //output序列化 TODO
//        for temp_str in btc_prepare_apdu_vec {
////            let xpub_data = hid_api::send(&hid_device, &temp_str);
//            send_apdu(temp_str);
//        }
        send_apdu(omni_prepare_apdu_str);

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
//                let btc_perpare_apdu_return = hid_api::send(&hid_device, &btc_perpare_apdu);
                send_apdu(btc_perpare_apdu);
            }
            for y in i * EACH_ROUND_NUMBER..(i+1)*EACH_ROUND_NUMBER {
                if y >= utxo_pub_key_vec.len(){
                    break;
                }
                let btc_sign_apdu = BtcApdu::btc_sign(y as u8, 0x01, format!("{}{}{}", path, "/", self.unspents.get(y).unwrap().derive_path).as_str());
                //发送签名指令到设备并获取签名结果
//                let btc_sign_apdu_return = hid_api::send(&hid_device, &btc_sign_apdu);
                let btc_sign_apdu_return = send_apdu(btc_sign_apdu);
                let sign_result_str = btc_sign_apdu_return[2..btc_sign_apdu_return.len() - 2].to_string();

                let sign_result_vec = Vec::from_hex(&sign_result_str).unwrap();
                let mut temp_signnture_obj = Signature::from_compact(sign_result_vec.as_slice()).unwrap();
                temp_signnture_obj.normalize_s();
                let mut sign_result_vec = temp_signnture_obj.serialize_der().to_vec();

                //添加HASH类型
                sign_result_vec.push(0x01);
                let temp_lock_script = Builder::new().push_slice(&sign_result_vec)
                    .push_slice(Vec::from_hex(utxo_pub_key_vec.get(y).unwrap()).unwrap().as_slice())
                    .into_script();

                lock_script_ver.push(temp_lock_script);
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

    pub fn sign_segwit_transaction(
        &self,
        network: Network,
        path: &String,
    ) -> Result<TxSignResult, BtcError> {
        //1.path校验

        //2.uxto数量检查
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber);
        }

        //3.get main public key(xpub)
        send_apdu(BtcApdu::select_applet());
        let xpub_data = send_apdu(BtcApdu::get_xpub(path.as_str(), false));

        //解析XPUB数据
        let sign_source_val = &xpub_data[..194];
        let sign_result = &xpub_data[194..];
        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //通过SE公钥验证签名
        let secp = Secp256k1::new();
        let se_pub_key = "04FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44";
        let se_pub_key_obj = PublicKey2::from_str(se_pub_key).unwrap();

        //对签名原值进行SHA256
        let message_hash = digest::digest(
            &digest::SHA256,
            Vec::from_hex(sign_source_val).unwrap().as_slice(),
        );
        let message_obj = Message::from_slice(message_hash.as_ref()).unwrap();
        //生成签名结果对象
        let mut sig = Signature::from_der(Vec::from_hex(sign_result).unwrap().as_slice()).unwrap();
        sig.normalize_s();
        let verify_result = secp.verify(&message_obj, &sig, &se_pub_key_obj).is_ok();
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
                let test_chain_number =
                    ChildNumber::from_normal_idx(index_number.parse().unwrap()).unwrap();
                pk = pk.ckd_pub(&bitcoin_secp, test_chain_number).unwrap();
            }
            //验证地址
            let temp_address = Address::p2shwpkh(
                &PublicKey::from_str(pk.public_key.to_string().as_str()).unwrap(),
                network,
            )
            .to_string();
            let temp_utxo_address = utxo.address.to_string();

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

        //5.add change output
        let mut txouts: Vec<TxOut> = Vec::new();
        let change_amount = total_amount - 546 - self.fee;
        let receiver_address = &self.unspents.get(0).unwrap().address;

        //6.add send to output
        let txout_send_output = TxOut {
            value: change_amount as u64,
            script_pubkey: receiver_address.script_pubkey(),
        };
        txouts.push(txout_send_output);//找零信息

        let txout_change_output = TxOut {
            value: 546 as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_change_output);//交易信息

        //budile omni data
        let mut property_id_bytes = num_bigint::BigInt::from(self.property_id).to_signed_bytes_le();
        if(property_id_bytes.len() < 4){
            let temp_number = 4 - property_id_bytes.len();
            for i in (0..temp_number) {
                property_id_bytes.push(0x00);
            }
        }
        property_id_bytes.reverse();
        let mut omni_data = hex::decode("6f6d6e6900000000").unwrap();
        omni_data.extend(property_id_bytes.iter());
        let mut amount_bytes = num_bigint::BigInt::from(self.amount).to_signed_bytes_le();
        if(amount_bytes.len() < 8){
            let temp_number = 8 - amount_bytes.len();
            for i in (0..temp_number) {
                amount_bytes.push(0x00);
            }
        }
        amount_bytes.reverse();
        omni_data.extend(amount_bytes.iter());
        println!("@@@@@@{}", hex::encode_upper(omni_data.clone()));
        let omni_output = TxOut {
            value: 0u64,
            script_pubkey: Builder::new().push_opcode(opcodes::all::OP_RETURN).push_slice(&omni_data[..]).into_script(),
        };

        println!("omni data-->{:?}", serialize(&omni_output).to_hex());
        txouts.push(omni_output);

        //8.output data serialize
        let mut tx_to_sign = Transaction {
            version: 2u32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data = serialize(&tx_to_sign);
        println!(
            "AAAAA->{:?}",
            hex::encode_upper(output_serialize_data.as_slice())
        );
        //删除多余的input序列化数据
        output_serialize_data.remove(5);
        output_serialize_data.remove(5);
        //增加签名类型
        output_serialize_data.extend_from_slice(hex::decode("01000000").unwrap().as_slice());
        //设置input数量
        output_serialize_data.remove(4);
        println!("@@@@@@->{:?}", self.unspents.len());
        output_serialize_data.insert(4, self.unspents.len() as u8);
        //添加旷工费用 0000000000002710 TODO
        output_serialize_data
            .extend_from_slice(hex::decode("0000000000000FA0").unwrap().as_slice());
        //添加地址版本 TODO
        output_serialize_data.extend_from_slice(hex::decode("6F").unwrap().as_slice());

        //
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);
        println!("%%%%%%%{:?}", hex::encode_upper(output_serialize_data.clone()));
        //对序列化数据进行SHA256计算
        let message_hash = digest::digest(&digest::SHA256, output_serialize_data.as_slice());
        let message_hash = digest::digest(&digest::SHA256, message_hash.as_ref());

        //使用本地私钥对数据进行签名 TODO
        let private_key = "B226EA7A230A75DA23EDA981566988A96D12578FB695958BF06BD579230D6710";
        let temp_secret_key = SecretKey::from_slice(hex::decode(private_key).unwrap().as_slice());
        let message_data = Message::from_slice(message_hash.as_ref()).unwrap();
        let secp = Secp256k1::new();
        let sign_result = secp
            .sign(&message_data, &temp_secret_key.unwrap())
            .serialize_der()
            .to_vec();
        let mut sign_data = Vec::new();
        sign_data.push(0x00 as u8);
        sign_data.push(sign_result.len() as u8);
        sign_data.extend_from_slice(sign_result.as_slice());

        sign_data.extend_from_slice(output_serialize_data.as_slice());
        println!("segwit serialize data->{:?}", hex::encode_upper(sign_data.clone()));
        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x00, &sign_data);
        //output序列化 TODO
        for temp_str in btc_prepare_apdu_vec {
            //            let xpub_data = hid_api::send(&hid_device, &temp_str);
            let xpub_data = send_apdu(temp_str);
        }

        let mut txinputs: Vec<TxIn> = Vec::new();
        let mut txhash_vout_vec = Vec::new();
        let mut sequence_vec : Vec<u8> = Vec::new();
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

            //======================================================================================================
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
            println!("&&&&&&{:?}", hex::encode_upper(data.clone()));
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
            println!("apdu-->{:?}", apdu);
            send_apdu(apdu);
        }

        //send sign apdu
        let mut lock_script_ver : Vec<Script> = Vec::new();
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (index, segwit_sign_apdu) in sign_apdu_vec.iter().enumerate() {
            let sign_apdu_return_data = send_apdu(segwit_sign_apdu.clone());
            let sign_result_str = sign_apdu_return_data[2..sign_apdu_return_data.len() - 2].to_string();
            println!("sign_apdu_return_data@@@@@@@@@-->{:?}", sign_apdu_return_data.clone());
            let sign_result_vec = Vec::from_hex(&sign_result_str).unwrap();
            let mut temp_signnture_obj =
                Signature::from_compact(sign_result_vec.as_slice()).unwrap();
            temp_signnture_obj.normalize_s();
            let mut sign_result_vec = temp_signnture_obj.serialize_der().to_vec();
            //设置hash类型
            sign_result_vec.push(0x01);
            println!("sign_result_vec@@@@@@@@@@-->{:?}", hex::encode_upper(sign_result_vec.clone()));
            witnesses.push((sign_result_vec, hex::decode(utxo_pub_key_vec.get(index).unwrap()).unwrap()));

        }

        let input_with_sigs: Result<Vec<TxIn>, _> = tx_to_sign
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
//                let pub_key = &self.prvkeys[0].public_key(&s);
//                let hash = hash160::Hash::hash(&pub_key.to_bytes()).into_inner();
                let hash = hash160::Hash::hash(hex_to_bytes(utxo_pub_key_vec.get(i).unwrap()).unwrap().as_slice()).into_inner();
                let hex = format!("160014{}", hex::encode(&hash));

                Ok(TxIn {
                    script_sig: Script::from(hex::decode(hex).unwrap()),
                    witness: vec![witnesses[i].0.clone(), witnesses[i].1.clone()],
                    ..*txin
                })
            })
            .collect();

//        let signed_tx = Transaction {
//            version: tx_to_sign.version,
//            lock_time: tx_to_sign.lock_time,
//            input: input_with_sigs?,
//            output: tx_to_sign.output.clone(),
//        };
        tx_to_sign.input = input_with_sigs?;
        let tx_bytes = serialize(&tx_to_sign);
        println!("seralize--->{:?}", hex::encode_upper(tx_bytes.clone()));
        println!("tx_bytes--->{:?}", tx_bytes.to_hex());
        println!("txid--->{:?}", tx_to_sign.txid().to_hex());
        println!("ntxid--->{:?}", tx_to_sign.ntxid().to_hex());

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.ntxid().to_hex(), //@@XM TODO: check this witness txid
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::usdt_transaction::{BtcTransaction, Transaction, Utxo};
    use bitcoin::{Address, Network};
    use hex::FromHex;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_sign_transaction() {
        //        let mut extra_data = HashMap::new();
        //        extra_data.insert("opReturn".to_string(), "0200000080a10bc28928f4c17a287318125115c3f098ed20a8237d1e8e4125bc25d1be99752adad0a7b9ceca853768aebb6965eca126a62965f698a0c1bc43d83db632ad7f717276057e6012afa99385".to_string());
//        let extra_data = Vec::from_hex("0200000080a10bc28928f4c17a287318125115c3f098ed20a8237d1e8e4125bc25d1be99752adad0a7b9ceca853768aebb6965eca126a62965f698a0c1bc43d83db632ad7f717276057e6012afa99385").unwrap();
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
//            change_idx: 53,
            amount: 10050000000,
            unspents: utxos,
            fee: 4000,
            payment: "100 USDT".to_string(),
            to_dis: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            from: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            fee_dis: "0.0004 BTC".to_string(),
            property_id: 31,
        };
        transaction_req_data.sign_transaction(Network::Testnet, &"m/44'/1'/0'".to_string());
    }

    #[test]
    fn test_sign_segwit_transaction() {
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
//            change_idx: 0,
            amount: 10000000000,
            unspents: utxos,
            fee: 4000,
            payment: "100 USDT".to_string(),
            to_dis: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            from: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            fee_dis: "0.0004 BTC".to_string(),
            property_id: 31
        };
        transaction_req_data.sign_segwit_transaction(Network::Testnet, &"m/49'/1'/0'/".to_string());
    }
}
