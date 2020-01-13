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
    pub fn sign_transaction(&self, network : Network, path : &String) -> Result<TxSignResult, BtcError>{
        //1.path校验

        //2.uxto数量检查
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(BtcError::ImkeyExceededMaxUtxoNumber);
        }
        //3.get main public key(xpub)
//        let select_btc_apdu: String = BtcApdu::select_applet();
//        let hid_device = hid_api::connect();
//        let apdu_response = hid_api::send(&hid_device, &select_btc_apdu);
//        let get_xpub_apdu = BtcApdu::get_xpub(path.as_str(), false);
//        let xpub_data = hid_api::send(&hid_device, &get_xpub_apdu);
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
        if total_amount < self.amount {
            return Err(BtcError::ImkeyInsufficientFunds);
        }

        //5.add send to output
        let mut txouts: Vec<TxOut> = Vec::new();
        let txout_send_output = TxOut {
            value: self.amount as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_send_output);//交易信息
        //6.add change output
        let change_amount = total_amount - self.amount - self.fee;
        if change_amount > 2730 {
            //获取找零地址 TODO
//            println!("path --> {}", format!("{}{}{}", path, "/1/", self.change_idx).as_str());
//            let get_xpub_apdu = BtcApdu::get_xpub(format!("{}{}{}", path, "/1/", self.change_idx).as_str(), true);
//            let xpub_data = hid_api::send(&hid_device, &get_xpub_apdu);

            let pub_key = &sign_source_val[..130];
            let mut temp_public_Key = PublicKey::from_str(pub_key).unwrap();
            temp_public_Key.compressed = true;
            let change_addr = Address::p2pkh(&temp_public_Key, network);
            let txout_change_output = TxOut {//xxxxxxxxxxxxxxxxxxxxxxxxxxx找零金额貌似不对
                value: self.amount as u64,
                script_pubkey: change_addr.script_pubkey(),
            };
            txouts.push(txout_change_output);//找零信息

        }
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
        let mut tx_to_sign = Transaction {
            version: 1u32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data =  serialize(&tx_to_sign);
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
        output_serialize_data.extend_from_slice(hex::decode("0000000000002710").unwrap().as_slice());
        //添加地址版本 TODO
        output_serialize_data.extend_from_slice(hex::decode("6F").unwrap().as_slice());

        //
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        //对序列化数据进行SHA256计算
        let message_hash = digest::digest(&digest::SHA256, output_serialize_data.as_slice());
        let message_hash = digest::digest(&digest::SHA256, message_hash.as_ref());

        //使用本地私钥对数据进行签名 TODO
        let private_key = "05366894B4D914A203BBE512766960A7E861E81A19E3E68666F1F6D04AA1F87F";
        let temp_secret_key = SecretKey::from_slice(hex::decode(private_key).unwrap().as_slice());
        let message_data = Message::from_slice(message_hash.as_ref()).unwrap();
        let secp = Secp256k1::new();
        let sign_result = secp.sign(&message_data, &temp_secret_key.unwrap()).serialize_der().to_vec();
        let mut  sign_data = Vec::new();
        sign_data.push(0x00 as u8);
        sign_data.push(sign_result.len() as u8);
        sign_data.extend_from_slice(sign_result.as_slice());

        sign_data.extend_from_slice(output_serialize_data.as_slice());

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x41, 0x00, &sign_data);
        //output序列化 TODO
        for temp_str in btc_prepare_apdu_vec {
//            let xpub_data = hid_api::send(&hid_device, &temp_str);
            send_apdu(temp_str);
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
        if total_amount < self.amount {
            return Err(BtcError::ImkeyInsufficientFunds);
        }

        //5.add send to output
        let mut txouts: Vec<TxOut> = Vec::new();
        let txout_send_output = TxOut {
            value: self.amount as u64,
            script_pubkey: self.to.script_pubkey(),
        };
        txouts.push(txout_send_output); //交易信息
                                        //6.add change output
        let change_amount = total_amount - self.amount - self.fee;
        if change_amount > 2730 {
            //获取找零地址
            //            println!("path --> {}", format!("{}{}{}", path, "/1/", self.change_idx).as_str());
            //            let get_xpub_apdu = BtcApdu::get_xpub(format!("{}{}{}", path, "/1/", self.change_idx).as_str(), true);
            //            let xpub_data = hid_api::send(&hid_device, &get_xpub_apdu);
            let pub_key = &sign_source_val[..130];
            let mut temp_public_Key = PublicKey::from_str(pub_key).unwrap();
            temp_public_Key.compressed = true;
            let change_addr = Address::p2pkh(&temp_public_Key, network);
            let txout_change_output = TxOut {
                value: self.amount as u64,
                script_pubkey: change_addr.script_pubkey(),
            };
            txouts.push(txout_change_output); //找零信息
        }
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
            .extend_from_slice(hex::decode("0000000000002710").unwrap().as_slice());
        //添加地址版本 TODO
        output_serialize_data.extend_from_slice(hex::decode("C4").unwrap().as_slice());

        //
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);
        println!("%%%%%%%{:?}", hex::encode_upper(output_serialize_data.clone()));
        //对序列化数据进行SHA256计算
        let message_hash = digest::digest(&digest::SHA256, output_serialize_data.as_slice());
        let message_hash = digest::digest(&digest::SHA256, message_hash.as_ref());

        //使用本地私钥对数据进行签名 TODO
        let private_key = "05366894B4D914A203BBE512766960A7E861E81A19E3E68666F1F6D04AA1F87F";
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
        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x00, &sign_data);
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
        for (index, wegwit_sign_apdu) in sign_apdu_vec.iter().enumerate() {
            let sign_apdu_return_data = send_apdu(wegwit_sign_apdu.clone());
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

    #[test]
    fn test_sign_segwit_transaction() {
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
            change_idx: 0,
            amount: 88000,
            unspents: utxos,
            fee: 10000,
            payment: "0.0001 BT".to_string(),
            to_dis: Address::from_str("3CVD68V71no5jn2UZpLLq6hASpXu1jrByt").unwrap(),
            from: Address::from_str("3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy").unwrap(),
            fee_dis: "0.00007945 BTC".to_string(),
            extra_data: extra_data,
        };
        transaction_req_data.sign_segwit_transaction(Network::Testnet, &"m/49'/1'/0'/".to_string());
    }
}
