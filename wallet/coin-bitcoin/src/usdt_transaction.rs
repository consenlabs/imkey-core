use crate::address::BtcAddress;
use crate::common::{
    address_verify, get_address_version, get_path_and_pubkeys, get_xpub_data,
    secp256k1_sign_verify, PathPubKey, TransTypeFlg, TxSignResult,
};
use crate::transaction::BtcTransaction;
use crate::Result;
use bitcoin::blockdata::{opcodes, script::Builder};
use bitcoin::consensus::serialize;
use bitcoin::hashes::hex::FromHex;
use bitcoin::util::psbt::serialize::Serialize;
use bitcoin::{Address, Network, OutPoint, Script, SigHashType, Transaction, TxIn, TxOut};
use bitcoin_hashes::hash160;
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::Hash;
use common::apdu::{ApduCheck, BtcApdu};
use common::constants::{
    BTC_NATIVE_SEGWIT_MAINNET_PATH, BTC_NATIVE_SEGWIT_TESTNET_PATH, EACH_ROUND_NUMBER,
    MAX_UTXO_NUMBER, MIN_NONDUST_OUTPUT, TIMEOUT_LONG, UNCOMPRESSED_PUBKEY_STRING_LEN,
    XPUB_STRING_LEN,
};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::{bigint_to_byte_vec, hex_to_bytes, secp256k1_sign};
use device::device_binding::KEY_MANAGER;
use secp256k1::Signature;
use std::str::FromStr;
use transport::message::{send_apdu, send_apdu_timeout};

impl BtcTransaction {
    pub fn sign_omni_transaction(
        &self,
        network: Network,
        path: &str,
        property_id: i32,
    ) -> Result<TxSignResult> {
        //path check
        check_path_validity(path)?;
        let mut path_str = path.to_string();
        if !path.ends_with("/") {
            path_str = format!("{}{}", path_str, "/");
        }
        //check uxto number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(CoinError::ImkeyExceededMaxUtxoNumber.into());
        }

        //check change amount
        if self.amount - self.fee < MIN_NONDUST_OUTPUT {
            return Err(CoinError::ImkeyAmountLessThanMinimum.into());
        }

        //get xpub and sign data
        let xpub_data = get_xpub_data(path_str.as_str(), true)?;
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();
        //get xpub data
        let sign_source_val = &xpub_data[..XPUB_STRING_LEN];
        let sign_result = &xpub_data[XPUB_STRING_LEN..];
        let pub_key = &sign_source_val[..UNCOMPRESSED_PUBKEY_STRING_LEN];
        let chain_code = &sign_source_val[UNCOMPRESSED_PUBKEY_STRING_LEN..];

        //use se public key verify sign
        let key_manager_obj = KEY_MANAGER.lock();
        let sign_verify_result = secp256k1_sign_verify(
            &key_manager_obj.se_pub_key.as_slice(),
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(sign_source_val).unwrap().as_slice(),
        );
        if sign_verify_result.is_err() || !sign_verify_result.ok().unwrap() {
            return Err(CoinError::ImkeySignatureVerifyFail.into());
        }

        //utxo address verify
        let utxo_pub_key_vec = address_verify(
            &self.unspents,
            pub_key,
            hex::decode(chain_code).unwrap().as_slice(),
            network,
            TransTypeFlg::BTC,
        )?;

        //add change output
        let mut txouts: Vec<TxOut> = Vec::new();
        let change_amount = self.get_total_amount() - MIN_NONDUST_OUTPUT - self.fee;
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

        //version, locktime and output data serialize
        let mut tx_to_sign = Transaction {
            version: 1i32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data = serialize(&tx_to_sign);

        output_serialize_data.remove(5);
        output_serialize_data.remove(5);

        //add sign type
        output_serialize_data.extend(SigHashType::All.serialize().iter());

        //set input number
        output_serialize_data.remove(4);
        output_serialize_data.insert(4, self.unspents.len() as u8);

        //add fee amount
        output_serialize_data.extend(bigint_to_byte_vec(self.fee));

        //set address version
        let address_version = get_address_version(network, self.to.to_string().as_str())?;
        output_serialize_data.push(address_version);

        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        //use local private key sign data
        let mut output_pareper_data =
            secp256k1_sign(&key_manager_obj.pri_key, &output_serialize_data)?;
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        //send output prepare command
        let omni_prepare_apdu_str = BtcApdu::omni_prepare_data(0x00, output_pareper_data);
        ApduCheck::check_response(&send_apdu_timeout(omni_prepare_apdu_str, TIMEOUT_LONG)?)?;
        let mut lock_script_ver: Vec<Script> = vec![];
        let count = (self.unspents.len() - 1) / EACH_ROUND_NUMBER + 1;
        for i in 0..count {
            for (x, temp_utxo) in self.unspents.iter().enumerate() {
                let mut input_data_vec = vec![];
                input_data_vec.push(x as u8);

                let mut temp_serialize_txin = TxIn {
                    previous_output: OutPoint {
                        txid: bitcoin::hash_types::Txid::from_hex(temp_utxo.txhash.as_str())?,
                        vout: temp_utxo.vout as u32,
                    },
                    script_sig: Script::default(),
                    sequence: 0xFFFFFFFF as u32,
                    witness: vec![],
                };
                if (x >= i * EACH_ROUND_NUMBER) && (x < (i + 1) * EACH_ROUND_NUMBER) {
                    temp_serialize_txin.script_sig =
                        Script::from(Vec::from_hex(temp_utxo.script_pubkey.as_str())?);
                }
                input_data_vec.extend_from_slice(serialize(&temp_serialize_txin).as_slice());
                let btc_perpare_apdu = BtcApdu::btc_perpare_input(0x80, &input_data_vec);
                //send perpare apdu
                ApduCheck::check_response(&send_apdu(btc_perpare_apdu)?)?;
            }
            for y in i * EACH_ROUND_NUMBER..(i + 1) * EACH_ROUND_NUMBER {
                if y >= utxo_pub_key_vec.len() {
                    break;
                }
                let btc_sign_apdu = BtcApdu::btc_sign(
                    y as u8,
                    SigHashType::All.as_u32() as u8,
                    format!("{}{}", path_str, self.unspents.get(y).unwrap().derive_path).as_str(),
                );
                //send sign apdu
                let btc_sign_apdu_return = send_apdu(btc_sign_apdu)?;
                ApduCheck::check_response(&btc_sign_apdu_return)?;
                let sign_result_str =
                    btc_sign_apdu_return[2..btc_sign_apdu_return.len() - 6].to_string();

                lock_script_ver.push(self.build_lock_script(
                    sign_result_str.as_str(),
                    utxo_pub_key_vec.get(y).unwrap(),
                )?);
            }
        }
        let mut txinputs: Vec<TxIn> = vec![];
        for (index, unspent) in self.unspents.iter().enumerate() {
            let txin = TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::hash_types::Txid::from_hex(&unspent.txhash)?,
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

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.ntxid().to_hex(),
        })
    }

    pub fn sign_omni_segwit_transaction(
        &self,
        network: Network,
        path: &str,
        property_id: i32,
    ) -> Result<TxSignResult> {
        //path check
        check_path_validity(path)?;
        let mut path_str = path.to_string();
        if !path.ends_with("/") {
            path_str = format!("{}{}", path_str, "/");
        }
        //check uxto number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(CoinError::ImkeyExceededMaxUtxoNumber.into());
        }
        let change_amount = self.get_total_amount() - self.fee - MIN_NONDUST_OUTPUT;
        //check change amount
        if change_amount < MIN_NONDUST_OUTPUT {
            return Err(CoinError::ImkeyAmountLessThanMinimum.into());
        }

        //get xpub and sign data
        let xpub_data = get_xpub_data(path_str.as_str(), true)?;
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();

        //get xpub data
        let sign_source_val = &xpub_data[..XPUB_STRING_LEN];
        let sign_result = &xpub_data[XPUB_STRING_LEN..];
        let pub_key = &sign_source_val[..UNCOMPRESSED_PUBKEY_STRING_LEN];
        let chain_code = &sign_source_val[UNCOMPRESSED_PUBKEY_STRING_LEN..];

        //use se public key verify sign
        let key_manager_obj = KEY_MANAGER.lock();
        let sign_verify_result = secp256k1_sign_verify(
            &key_manager_obj.se_pub_key.as_slice(),
            hex::decode(sign_result).unwrap().as_slice(),
            hex::decode(sign_source_val).unwrap().as_slice(),
        );
        if sign_verify_result.is_err() || !sign_verify_result.ok().unwrap() {
            return Err(CoinError::ImkeySignatureVerifyFail.into());
        }

        //utxo address verify
        let utxo_pub_key_vec = address_verify(
            &self.unspents,
            pub_key,
            hex::decode(chain_code).unwrap().as_slice(),
            network,
            TransTypeFlg::SEGWIT,
        )?;

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
            version: 2i32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data = serialize(&tx_to_sign);

        output_serialize_data.remove(5);
        output_serialize_data.remove(5);
        //add sign type
        output_serialize_data.extend(SigHashType::All.serialize().iter());
        //set input number
        output_serialize_data.remove(4);
        output_serialize_data.insert(4, self.unspents.len() as u8);
        //add fee amount
        output_serialize_data.extend(bigint_to_byte_vec(self.fee));
        //set address version
        let address_version = get_address_version(network, self.to.to_string().as_str())?;
        output_serialize_data.push(address_version);
        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);
        //use local private key sign data
        let mut output_pareper_data =
            secp256k1_sign(&key_manager_obj.pri_key, &output_serialize_data)?;
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x00, &output_pareper_data);
        for temp_str in btc_prepare_apdu_vec {
            ApduCheck::check_response(&send_apdu_timeout(temp_str, TIMEOUT_LONG)?)?;
        }

        let mut txinputs: Vec<TxIn> = vec![];
        let mut txhash_vout_vec = vec![];
        let mut sequence_vec: Vec<u8> = vec![];
        let mut sign_apdu_vec: Vec<String> = vec![];
        for (index, unspent) in self.unspents.iter().enumerate() {
            let txin = TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::hash_types::Txid::from_hex(&unspent.txhash)?,
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF as u32,
                witness: vec![],
            };

            txhash_vout_vec.extend(serialize(&txin.previous_output).iter());
            sequence_vec.extend(serialize(&txin.sequence).iter());

            let mut data: Vec<u8> = vec![];
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
            while utxo_amount.len() < 8 {
                utxo_amount.push(0x00);
            }
            data.extend(utxo_amount.iter());

            //sequence
            data.extend(hex::decode("FFFFFFFF").unwrap());

            //set length
            data.insert(0, data.len() as u8);

            //address
            let mut address_data: Vec<u8> = vec![];
            let sign_path = format!("{}{}", path_str, unspent.derive_path);
            address_data.push(sign_path.as_bytes().len() as u8);
            address_data.extend_from_slice(sign_path.as_bytes());

            data.extend(address_data.iter());
            if index == self.unspents.len() - 1 {
                sign_apdu_vec.push(BtcApdu::btc_segwit_sign(true, 0x01, data));
            } else {
                sign_apdu_vec.push(BtcApdu::btc_segwit_sign(false, 0x01, data));
            }

            txinputs.push(txin.clone());
        }
        tx_to_sign.input = txinputs;

        let mut txhash_vout_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x40, &txhash_vout_vec);
        let mut sequence_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x80, &sequence_vec);
        txhash_vout_prepare_apdu_vec.append(&mut sequence_prepare_apdu_vec);
        for prepare_apdu in txhash_vout_prepare_apdu_vec {
            ApduCheck::check_response(&send_apdu(prepare_apdu)?)?;
        }

        //send sign apdu
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (index, segwit_sign_apdu) in sign_apdu_vec.iter().enumerate() {
            //send sign apdu
            let sign_apdu_return_data = send_apdu(segwit_sign_apdu.clone())?;
            ApduCheck::check_response(&sign_apdu_return_data)?;
            //build signature obj
            let sign_result_vec =
                Vec::from_hex(&sign_apdu_return_data[2..sign_apdu_return_data.len() - 6]).unwrap();
            let mut temp_signature_obj = Signature::from_compact(sign_result_vec.as_slice())?;
            temp_signature_obj.normalize_s();
            //generator der sign data
            let mut sign_result_vec = temp_signature_obj.serialize_der().to_vec();
            //add hash type
            sign_result_vec.push(SigHashType::All.as_u32() as u8);
            witnesses.push((
                sign_result_vec,
                hex::decode(utxo_pub_key_vec.get(index).unwrap())?,
            ));
        }

        let input_with_sigs: Result<Vec<TxIn>> = tx_to_sign
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let hash = hash160::Hash::hash(
                    hex_to_bytes(utxo_pub_key_vec.get(i).unwrap())
                        .unwrap()
                        .as_slice(),
                )
                .into_inner();
                let hex = format!("160014{}", hex::encode(&hash));
                Ok(TxIn {
                    script_sig: Script::from(hex::decode(hex).unwrap()),
                    witness: vec![witnesses[i].0.clone(), witnesses[i].1.clone()],
                    ..*txin
                })
            })
            .collect();

        tx_to_sign.input = input_with_sigs?;
        let tx_bytes = serialize(&tx_to_sign);

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.wtxid().to_hex(),
        })
    }

    pub fn sign_omni_mixed_transaction(
        &self,
        network: Network,
        property_id: i32,
    ) -> Result<TxSignResult> {
        let path_and_pubkeys = get_path_and_pubkeys(&self.unspents, network)?;

        let change_amount = self.get_total_amount() - self.fee - MIN_NONDUST_OUTPUT;
        //check change amount
        if change_amount < MIN_NONDUST_OUTPUT {
            return Err(CoinError::ImkeyAmountLessThanMinimum.into());
        }

        let parent_path = match network {
            Network::Testnet => BTC_NATIVE_SEGWIT_TESTNET_PATH,
            Network::Bitcoin => BTC_NATIVE_SEGWIT_MAINNET_PATH,
            _ => BTC_NATIVE_SEGWIT_MAINNET_PATH,
        };
        //add change output
        let mut txouts: Vec<TxOut> = vec![];
        let path_temp = format!("{}{}", parent_path, "/1/0");
        let address_str = BtcAddress::get_native_segwit_address(network, path_temp.as_str())?;
        let address_obj = Address::from_str(address_str.as_str())?;
        let txout_send_output = TxOut {
            value: change_amount as u64,
            script_pubkey: address_obj.script_pubkey(),
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
            version: 2i32,
            lock_time: 0u32,
            input: vec![],
            output: txouts,
        };
        let mut output_serialize_data = serialize(&tx_to_sign);

        output_serialize_data.remove(5);
        output_serialize_data.remove(5);
        //add sign type
        output_serialize_data.extend(SigHashType::All.serialize().iter());
        //set input number
        output_serialize_data.remove(4);
        output_serialize_data.insert(4, self.unspents.len() as u8);
        //add fee amount
        output_serialize_data.extend(bigint_to_byte_vec(self.fee));
        //set address version
        let address_version = get_address_version(network, self.to.to_string().as_str())?;
        output_serialize_data.push(address_version);
        //set 01 tag and length
        output_serialize_data.insert(0, output_serialize_data.len() as u8);
        output_serialize_data.insert(0, 0x01);

        let key_manager_obj = KEY_MANAGER.lock();
        //use local private key sign data
        let mut output_pareper_data =
            secp256k1_sign(&key_manager_obj.pri_key, &output_serialize_data)?;
        output_pareper_data.insert(0, output_pareper_data.len() as u8);
        output_pareper_data.insert(0, 0x00);
        output_pareper_data.extend(output_serialize_data.iter());

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x00, &output_pareper_data);
        for temp_str in btc_prepare_apdu_vec {
            ApduCheck::check_response(&send_apdu_timeout(temp_str, TIMEOUT_LONG)?)?;
        }

        let mut txinputs: Vec<TxIn> = vec![];
        let mut txhash_vout_vec = vec![];
        let mut sequence_vec: Vec<u8> = vec![];

        // hash vout data and Sequnce data
        for unspent in self.unspents.iter() {
            let txin = TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::hash_types::Txid::from_hex(&unspent.txhash)?,
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF as u32,
                witness: vec![],
            };

            txhash_vout_vec.extend(serialize(&txin.previous_output).iter());
            sequence_vec.extend(serialize(&txin.sequence).iter());
        }

        let mut txhash_vout_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x40, &txhash_vout_vec);
        let mut sequence_prepare_apdu_vec = BtcApdu::btc_prepare(0x34, 0x80, &sequence_vec);
        txhash_vout_prepare_apdu_vec.append(&mut sequence_prepare_apdu_vec);
        for prepare_apdu in txhash_vout_prepare_apdu_vec {
            ApduCheck::check_response(&send_apdu(prepare_apdu)?)?;
        }

        // sign
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        let mut lock_script_ver: Vec<Script> = vec![];

        for (index, unspent) in self.unspents.iter().enumerate() {
            let path_pubkey: &PathPubKey = path_and_pubkeys.get(index).unwrap();

            let txin = TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::hash_types::Txid::from_hex(&unspent.txhash)?,
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF as u32,
                witness: vec![],
            };

            let mut data: Vec<u8> = vec![];

            // type? legacy
            if unspent.script_pubkey.starts_with("76a914")
                || unspent.script_pubkey.starts_with("76A914")
            {
                for (x, temp_utxo) in self.unspents.iter().enumerate() {
                    let mut input_data_vec = vec![];
                    let mut temp_serialize_txin = TxIn {
                        previous_output: OutPoint {
                            txid: bitcoin::hash_types::Txid::from_hex(temp_utxo.txhash.as_str())?,
                            vout: temp_utxo.vout as u32,
                        },
                        script_sig: Script::default(),
                        sequence: 0xFFFFFFFF as u32,
                        witness: vec![],
                    };
                    if x == index {
                        temp_serialize_txin.script_sig =
                            Script::from(Vec::from_hex(temp_utxo.script_pubkey.as_str())?);
                    }
                    input_data_vec.extend_from_slice(serialize(&temp_serialize_txin).as_slice());

                    let btc_perpare_apdu = if x == self.unspents.len() - 1 {
                        BtcApdu::btc_legacy_sign(0x00, 0x80, &input_data_vec)
                    } else if x == 0 {
                        BtcApdu::btc_legacy_sign(0x00, 0x40, &input_data_vec)
                    } else {
                        BtcApdu::btc_legacy_sign(0x00, 0x00, &input_data_vec)
                    };
                    //send perpare apdu to device
                    ApduCheck::check_response(&send_apdu(btc_perpare_apdu)?)?;
                }

                let btc_sign_apdu = BtcApdu::btc_legacy_sign(
                    0x80 as u8,
                    0x80 as u8,
                    &path_pubkey.path.as_bytes().to_vec(),
                );
                //sign data
                let btc_sign_apdu_return = send_apdu(btc_sign_apdu)?;
                ApduCheck::check_response(&btc_sign_apdu_return)?;
                let btc_sign_apdu_return =
                    &btc_sign_apdu_return[..btc_sign_apdu_return.len() - 4].to_string();
                let sign_result_str =
                    btc_sign_apdu_return[2..btc_sign_apdu_return.len() - 2].to_string();

                lock_script_ver
                    .push(self.build_lock_script(sign_result_str.as_str(), &path_pubkey.pub_key)?);
                // witnesses
                witnesses.push((
                    hex::decode(&sign_result_str)?,
                    hex::decode(&path_pubkey.pub_key)?,
                ));

            // segwit
            } else {
                //txhash and vout
                let txhash_data = serialize(&txin.previous_output);
                data.extend(txhash_data.iter());

                //lock script
                let pub_key_bytes = hex::decode(&path_pubkey.pub_key)?;
                let pub_key_hash = hash160::Hash::hash(&pub_key_bytes).into_inner();
                let script_hex = format!("76a914{}88ac", hex::encode(pub_key_hash));
                let script = Script::from(hex::decode(script_hex)?);
                let script_data = serialize(&script);
                data.extend(script_data.iter());

                //amount
                let mut utxo_amount = num_bigint::BigInt::from(unspent.amount).to_signed_bytes_le();
                while utxo_amount.len() < 8 {
                    utxo_amount.push(0x00);
                }
                data.extend(utxo_amount.iter());

                //sequence
                data.extend(hex::decode("FFFFFFFF").unwrap());

                //set length
                data.insert(0, data.len() as u8);

                //address
                let mut address_data: Vec<u8> = vec![];
                let sign_path = &path_pubkey.path;
                address_data.push(sign_path.as_bytes().len() as u8);
                address_data.extend_from_slice(sign_path.as_bytes());

                data.extend(address_data.iter());
                let segwit_sign_apdu = if index == self.unspents.len() - 1 {
                    BtcApdu::btc_segwit_sign(true, 0x01, data)
                } else {
                    BtcApdu::btc_segwit_sign(false, 0x01, data)
                };

                //send sign apdu
                let sign_apdu_return_data = send_apdu(segwit_sign_apdu.clone())?;
                ApduCheck::check_response(&sign_apdu_return_data)?;
                //build signature obj
                let sign_result_vec =
                    Vec::from_hex(&sign_apdu_return_data[2..sign_apdu_return_data.len() - 6])
                        .unwrap();

                let mut signature_obj = Signature::from_compact(sign_result_vec.as_slice())?;
                signature_obj.normalize_s();
                //generator der sign data
                let mut sign_result_vec = signature_obj.serialize_der().to_vec();
                //add hash type
                sign_result_vec.push(SigHashType::All.as_u32() as u8);

                witnesses.push((sign_result_vec, hex::decode(&path_pubkey.pub_key)?));

                lock_script_ver.push(Script::new());
            }

            txinputs.push(txin.clone());
        }

        tx_to_sign.input = txinputs;

        let input_with_sigs: Result<Vec<TxIn>> = tx_to_sign
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let script_pubkey = self.unspents.get(i).unwrap().script_pubkey.clone();
                if script_pubkey.starts_with("76a914") || script_pubkey.starts_with("76A914") {
                    Ok(TxIn {
                        script_sig: lock_script_ver.get(i).unwrap().clone(),
                        witness: vec![],
                        ..*txin
                    })
                // segwit
                } else if script_pubkey.starts_with("a914") || script_pubkey.starts_with("A914") {
                    let hash = hash160::Hash::hash(
                        hex_to_bytes(&path_and_pubkeys.get(i).unwrap().pub_key)
                            .unwrap()
                            .as_slice(),
                    )
                    .into_inner();
                    let hex = format!("160014{}", hex::encode(&hash));
                    Ok(TxIn {
                        script_sig: Script::from(hex::decode(hex).unwrap()),
                        witness: vec![witnesses[i].0.clone(), witnesses[i].1.clone()],
                        ..*txin
                    })
                } else if script_pubkey.starts_with("0014") {
                    Ok(TxIn {
                        script_sig: Script::new(),
                        witness: vec![witnesses[i].0.clone(), witnesses[i].1.clone()],
                        ..*txin
                    })
                } else {
                    return Err(CoinError::UnsupportedScriptPubkey.into());
                }
            })
            .collect();

        tx_to_sign.input = input_with_sigs?;
        let tx_bytes = serialize(&tx_to_sign);

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: tx_to_sign.txid().to_hex(),
            wtx_id: tx_to_sign.wtxid().to_hex(),
        })
    }

    pub fn build_omni_output(&self, property_id: i32, amount: i64) -> TxOut {
        let mut property_id_bytes = num_bigint::BigInt::from(property_id).to_signed_bytes_le();
        while property_id_bytes.len() < 4 {
            property_id_bytes.push(0x00);
        }
        property_id_bytes.reverse();
        let mut omni_data = hex::decode("6f6d6e6900000000").unwrap();
        omni_data.extend(property_id_bytes.iter());
        let mut amount_bytes = num_bigint::BigInt::from(amount).to_signed_bytes_le();
        while amount_bytes.len() < 8 {
            amount_bytes.push(0x00);
        }
        amount_bytes.reverse();
        omni_data.extend(amount_bytes.iter());
        TxOut {
            value: 0u64,
            script_pubkey: Builder::new()
                .push_opcode(opcodes::all::OP_RETURN)
                .push_slice(&omni_data[..])
                .into_script(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{BtcTransaction, Utxo};
    use bitcoin::{Address, Network};
    use std::str::FromStr;

    use device::device_binding::bind_test;
    use device::device_binding::DeviceManage;
    use transport::hid_api::hid_connect;

    #[test]
    fn test_sign_transaction() {
        //binding device
        bind_test();

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
        };
        let sign_result = transaction_req_data.sign_omni_transaction(
            Network::Testnet,
            &"m/44'/1'/0'".to_string(),
            31,
        );
        assert_eq!(
            "36a25fa2005b5d4922d18f6f819bf068dca479d4103904ce225a9438a2c1f5a0",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "cae12904648aa67844484be85482fc5f65c99b9d45d56adc9c46db3fad7ba17b",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_segwit_transaction() {
        //binding device
        bind_test();

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
        };
        let sign_result = transaction_req_data.sign_omni_segwit_transaction(
            Network::Testnet,
            &"m/49'/1'/0'/".to_string(),
            31,
        );

        assert_eq!(
            "e664888c4a67cfed29786e5ada0c24cb25b91cafca4ae699fb7b90e7071e88bc",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "acde42849de610fcd79da9f9b782bc9dc08545af82be830ca6e28362865099ba",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_segwit_transaction_8utxo() {
        //binding device
        bind_test();

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
        };
        let sign_result = transaction_req_data.sign_omni_segwit_transaction(
            Network::Bitcoin,
            &"m/49'/0'/0'/".to_string(),
            31,
        );
        assert_eq!(
            "79ec1ab9008e3ce2809419d7b25c58de0f03a782e81f15d0e92042e16f141434",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "de169a96937270d8f82155cf48dd13705ae2ddf0d4a8dcc49d0a660b24b95323",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_segwit_transaction_mainnet() {
        //binding device
        bind_test();

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
        };
        let sign_result = transaction_req_data.sign_omni_segwit_transaction(
            Network::Bitcoin,
            &"m/49'/0'/0'/".to_string(),
            31,
        );
        assert_eq!(
            "0f3365929829d1d519751ed65bc0751cae6fe4480bc7b2098efa8c634e8b11b5",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "8d96dd2aaea48ea0b5aea066c4390bef46f855a58714d8c40dd9701c52367d41",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn exceeded_max_utxo_number_test() {
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
        for _x in 0..253 {
            utxos.push(utxo.clone());
        }
        let transaction_req_data = BtcTransaction {
            to: Address::from_str("3PGEDofNu6aJ3KfgK9PHGt3EW3oZK5qY1a").unwrap(),
            amount: 345678,
            unspents: utxos,
            fee: 502130,
        };
        let sign_result = transaction_req_data.sign_omni_segwit_transaction(
            Network::Bitcoin,
            &"m/49'/0'/0'".to_string(),
            31,
        );
        assert_eq!(
            format!("{}", sign_result.err().unwrap()),
            "imkey_exceeded_max_utxo_number"
        );

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
        for _x in 0..253 {
            utxos.push(utxo.clone());
        }

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            amount: 10050000000,
            unspents: utxos,
            fee: 4000,
        };
        let sign_result = transaction_req_data.sign_omni_transaction(
            Network::Testnet,
            &"m/44'/1'/0'".to_string(),
            31,
        );

        assert_eq!(
            format!("{}", sign_result.err().unwrap()),
            "imkey_exceeded_max_utxo_number"
        );
    }

    #[test]
    fn amount_less_than_minimum_test() {
        let utxo = Utxo {
            txhash: "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
            vout: 0,
            amount: 1000,
            address: Address::from_str("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/22".to_string(),
            sequence: 0,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("3PGEDofNu6aJ3KfgK9PHGt3EW3oZK5qY1a").unwrap(),
            amount: 100,
            unspents: utxos,
            fee: 900,
        };
        let sign_result = transaction_req_data.sign_omni_segwit_transaction(
            Network::Bitcoin,
            &"m/49'/0'/0'/".to_string(),
            31,
        );
        assert_eq!(
            format!("{}", sign_result.err().unwrap()),
            "imkey_amount_less_than_minimum"
        );

        let utxo = Utxo {
            txhash: "0dd195c815c5086c5995f43a0c67d28344ae5fa130739a5e03ef40fea54f2031".to_string(),
            vout: 0,
            amount: 1000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 4294967295,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3").unwrap(),
            amount: 100,
            unspents: utxos,
            fee: 900,
        };
        let sign_result = transaction_req_data.sign_omni_transaction(
            Network::Testnet,
            &"m/44'/1'/0'".to_string(),
            31,
        );

        assert_eq!(
            format!("{}", sign_result.err().unwrap()),
            "imkey_amount_less_than_minimum"
        );
    }

    #[test]
    fn test_sign_mixed_single_legacy_utxo_transaction() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "1cd9bfa2cabf071aca138e38e7ba281fa0aa26dd554d3518a2f3f74d33e9d3f5".to_string(),
            vout: 0,
            amount: 100000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("2N5z4KZBCQNULTegkETDftMiNHWEFjrH3m2").unwrap(),
            amount: 30000,
            unspents: utxos,
            fee: 10000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "0200000001f5d3e9334df7f3a218354d55dd26aaa01f28bae7388e13ca1a07bfcaa2bfd91c000000006b483045022100f0b4c89a26e40a55852a4cb470b82bdeb7ecfcbe735ff597d998bb0a02cd488a02201bdd7a29cedd5b33315a3334c654d96d980e117159f685096efba49adb21b6330121033d710ab45bb54ac99618ad23b3c1da661631aa25f23bfe9d22b41876f1d46e4effffffff036e5d010000000000160014229ecfce73bdf1f979a55802acb9372fe2230242220200000000000017a9148bbb53570df9656926ea0ef029cd2ee84dbc7d0f870000000000000000166a146f6d6e69000000000000001f000000000000753000000000",
            sign_result.as_ref().unwrap().signature
        );
        assert_eq!(
            "b6baf1dc1441a9c256edbb43260e595684b94f36e697c58e2bf88f206b2db633",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "b6baf1dc1441a9c256edbb43260e595684b94f36e697c58e2bf88f206b2db633",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_mixed_p2shp2wpkh_utxo_haschange() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "8e7bb196a518413b80f08439b71cd7cfcf8b0c19e493f2f19aea1890fb834afe".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("2N5z4KZBCQNULTegkETDftMiNHWEFjrH3m2").unwrap(),
            amount: 50000,
            unspents: utxos,
            fee: 10000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "02000000000101fe4a83fb9018ea9af1f293e4190c8bcfcfd71cb73984f0803b4118a596b17b8e0100000017160014654fbb08267f3d50d715a8f1abb55979b160dd5bffffffff036e5d010000000000160014229ecfce73bdf1f979a55802acb9372fe2230242220200000000000017a9148bbb53570df9656926ea0ef029cd2ee84dbc7d0f870000000000000000166a146f6d6e69000000000000001f000000000000c35002473044022013c52ba4cebb804aac8dbd844afd8382306c15d74ebef053c298a12fcd310fbb0220719a3e28f40c8bafac33047003fc6d9b03229a5badafc5e8fd54713a5c4d445d0121031aee5e20399d68cf0035d1a21564868f22bc448ab205292b4279136b15ecaebc00000000",
            sign_result.as_ref().unwrap().signature
        );

        assert_eq!(
            "943fe03184831f4b68b2ca28214c91e2bc12055aaa189d02ac244cbe04631c0a",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "5e574abd655b07bfff948ca2844d192c4fe34f54786a51fcbf0cfd3819ffd92e",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_mixed_single_bech32_utxo_haschange() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "41eb7058313847d1f1b0cfee964a436d55eab5ca29fdbb42dbb5107a85afdda7".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("tb1qrfaf3g4elgykshfgahktyaqj2r593qkrae5v95").unwrap(),
            script_pubkey: "00141a7a98a2b9fa09685d28edecb2741250e85882c3".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("tb1qpma75pm648xmd9tfzah029edarqn4xtndqhp99").unwrap(),
            amount: 30000,
            unspents: utxos,
            fee: 10000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "02000000000101a7ddaf857a10b5db42bbfd29cab5ea556d434a96eecfb0f1d14738315870eb410100000000ffffffff036e5d010000000000160014229ecfce73bdf1f979a55802acb9372fe223024222020000000000001600140efbea077aa9cdb69569176ef5172de8c13a99730000000000000000166a146f6d6e69000000000000001f00000000000075300247304402206fd4b23581b6a25c36020d0f2187c9e47dec82eaa829f51819abd5f6fc71b96d02207c5c1613d037cbb147ea9775bae83ed4d86126a815045d345e7763de8005a0db012102e24f625a31c9a8bae42239f2bf945a306c01a450a03fd123316db0e837a660c000000000",
            sign_result.as_ref().unwrap().signature
        );

        assert_eq!(
            "03a066183fd2616f2d1afc24b233c7e6e49fc26ed975e85010ea19604e511d7f",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "86c1713ee5d8edcbb3205f869bf697b0b6b3df59d39a7d718b76fe0b74629230",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_mixed_single_legacy_and_segwit_utxo_haschange() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "356d5e8628466f072c1de991e14320226ceef944cfebec251dd5c87ea925823c".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo2 = Utxo {
            txhash: "b63ca3592561fd7c8b41017fbb0deff12ce6f7d351128c818dcf4ed1a0beae0e".to_string(),
            vout: 1,
            amount: 1418852,
            address: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("2N5z4KZBCQNULTegkETDftMiNHWEFjrH3m2").unwrap(),
            amount: 1508852,
            unspents: utxos,
            fee: 10000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "020000000001023c8225a97ec8d51d25ecebcf44f9ee6c222043e191e91d2c076f4628865e6d35010000006b483045022100b38f63482c9a5bb808006e396ebaafb2bd59aa194dc51cfe803e06a0deaf7f1202206519842f1cf655fe7bd7ad4020c78ee971861d5cea68421d47ac9cd9a31aeedb0121033d710ab45bb54ac99618ad23b3c1da661631aa25f23bfe9d22b41876f1d46e4effffffff0eaebea0d14ecf8d818c1251d3f7e62cf1ef0dbb7f01418b7cfd612559a33cb60100000017160014654fbb08267f3d50d715a8f1abb55979b160dd5bffffffff03d203170000000000160014229ecfce73bdf1f979a55802acb9372fe2230242220200000000000017a9148bbb53570df9656926ea0ef029cd2ee84dbc7d0f870000000000000000166a146f6d6e69000000000000001f00000000001705f400024730440220140398fdfd8d5e60e65985fafdba1d3f4cbf82312de0cf355394f5930abb3781022038e4ff4b0795de5fb46f72667ba1ac88d6757035164363204b8e3babb06b8d3f0121031aee5e20399d68cf0035d1a21564868f22bc448ab205292b4279136b15ecaebc00000000",
            sign_result.as_ref().unwrap().signature
        );

        assert_eq!(
            "c15046b04213add1c327b4b0506f414081af1c55edd7947286a062ec373bb16a",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "f09917e7c2b0147d1d442a1def392fe0282bfb8513f0a2fb42c333acdba966c6",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_mixed_p2sh_p2wpkh_and_bech32_utxo_haschange() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "0a7937fe1c6d03fb835aced9f3ca5fd3b2f1c78ed1f5f394ad742a01897157d7".to_string(),
            vout: 0,
            amount: 100000,
            address: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let utxo2 = Utxo {
            txhash: "94fbcc624b34c6a1e7681312b490f0fbfaf3fb6efe90efb16a57815ea0c34edd".to_string(),
            vout: 0,
            amount: 100000,
            address: Address::from_str("tb1qrfaf3g4elgykshfgahktyaqj2r593qkrae5v95").unwrap(),
            script_pubkey: "00141a7a98a2b9fa09685d28edecb2741250e85882c3".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("tb1qpma75pm648xmd9tfzah029edarqn4xtndqhp99").unwrap(),
            amount: 90000,
            unspents: utxos,
            fee: 10000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "02000000000102d7577189012a74ad94f3f5d18ec7f1b2d35fcaf3d9ce5a83fb036d1cfe37790a0000000017160014654fbb08267f3d50d715a8f1abb55979b160dd5bffffffffdd4ec3a05e81576ab1ef90fe6efbf3fafbf090b4121368e7a1c6344b62ccfb940000000000ffffffff030ee4020000000000160014229ecfce73bdf1f979a55802acb9372fe223024222020000000000001600140efbea077aa9cdb69569176ef5172de8c13a99730000000000000000166a146f6d6e69000000000000001f0000000000015f900248304502210089675fbe6b1904bc7f22a18d8d9c312244a547a41d2a1190317f86eb593c3e0702202189b122ed22ac6ad741082cf075317d39df4a86f532f223a00e8e17b5843d6f0121031aee5e20399d68cf0035d1a21564868f22bc448ab205292b4279136b15ecaebc02473044022053e3b7b9fa063e517f924be0e6cbddd8f03966d9878d9f53963f442267e3924d022064a64488d473bcc7ec67131793b0e2836429a2c44895b54b6084e5241237aa28012102e24f625a31c9a8bae42239f2bf945a306c01a450a03fd123316db0e837a660c000000000",
            sign_result.as_ref().unwrap().signature
        );

        assert_eq!(
            "980a5d02ec3e84b5f6048ad6cfe3c3158f5f80d6665b685a9a7b56cb5346488b",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "836dbe5acdb3a0a79fe8688af7affaf99666fb2fa36dad23bb33bc978430719b",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_mixed_transaction() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "b7b05e82cd4dad038d7f7545f02940ed959aa8f54b1701688927649f99021e60".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN").unwrap(),
            script_pubkey: "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo2 = Utxo {
            txhash: "36671b4b8f72542ae9b9708725119837b233177d28a710204b839343b8a811a0".to_string(),
            vout: 0,
            amount: 100000,
            address: Address::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap(),
            script_pubkey: "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo3 = Utxo {
            txhash: "6459945baee1c250c9099f2f23e24af5dbd73292f0d994bef076d3f65356563a".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("tb1qrfaf3g4elgykshfgahktyaqj2r593qkrae5v95").unwrap(),
            script_pubkey: "00141a7a98a2b9fa09685d28edecb2741250e85882c3".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let utxo4 = Utxo {
            txhash: "6d1d8f16f93fe99de489e20d5d08b59f0d98754e0a84824889d9a59cc640ffac".to_string(),
            vout: 1,
            amount: 70000,
            address: Address::from_str("tb1q0qz6vdsay5edatqmvtyn9z92zkfs3hxqvk8k8k").unwrap(),
            script_pubkey: "00141a7a98a2b9fa09685d28edecb2741250e85882c3".to_string(),
            derive_path: "1/53".to_string(),
            sequence: 0,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);
        utxos.push(utxo3);
        utxos.push(utxo4);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("tb1qpma75pm648xmd9tfzah029edarqn4xtndqhp99").unwrap(),
            amount: 310000,
            unspents: utxos,
            fee: 10000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "02000000000104601e02999f6427896801174bf5a89a95ed4029f045757f8d03ad4dcd825eb0b7010000006a4730440220534d6a8bbc517033474d8dd77302ec9757b24dbeed9a3c7552dacdf65f6b4f04022005d48c18fb68dd683e9567b2ceba177bfdf9ce9a8c1a4a62d396fb9fab7fc3a40121033d710ab45bb54ac99618ad23b3c1da661631aa25f23bfe9d22b41876f1d46e4effffffffa011a8b84393834b2010a7287d1733b2379811258770b9e92a54728f4b1b67360000000017160014654fbb08267f3d50d715a8f1abb55979b160dd5bffffffff3a565653f6d376f0be94d9f09232d7dbf54ae2232f9f09c950c2e1ae5b9459640100000000ffffffffacff40c69ca5d9894882840a4e75980d9fb5085d0de289e49de93ff9168f1d6d0100000000ffffffff031e7c050000000000160014229ecfce73bdf1f979a55802acb9372fe223024222020000000000001600140efbea077aa9cdb69569176ef5172de8c13a99730000000000000000166a146f6d6e69000000000000001f000000000004baf0000247304402202390051593504c1c5183c808164f0fa4580358c0191b09dffb094aca10d3172a02204b19867cf383b9b362f49fee917f562c8875ff2b8d4c4bb864837c7bfc77ac3f0121031aee5e20399d68cf0035d1a21564868f22bc448ab205292b4279136b15ecaebc02473044022045e31d536220f9185ab7b319c84afd13a5234653a13bfebed8ec7a0a022ac69a02205f2bf6c7525c45296b47d1ba9372a035c4054d8da0f01895d7f31a2e5934726b012102e24f625a31c9a8bae42239f2bf945a306c01a450a03fd123316db0e837a660c00247304402207b5b63bfb80c9c7bfc9c1be92a8c66798d1a3c5a7cf404d239febb88950d20dc0220607239838f8b5b57a7562734f9c9c732ad5f70f385cba28593a70366e674540001210383f26c44bf1607224237a93e8735ff69a23655878ddb22c46fcdd850417097a400000000",
            sign_result.as_ref().unwrap().signature
        );
        assert_eq!(
            "9d0dc9f491b60e179f6f5c129add2c69dd13326dcad6d96471c337d943bc2d8b",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "89e6f589fd51156121040f9a4084cb69f66cf598e058a6cf58fd3f86f6bd536d",
            sign_result.as_ref().unwrap().wtx_id
        );
    }

    #[test]
    fn test_sign_mixed_multi_bech32_utxo_has_change() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "80f482aa891508c205a8b2fc52756b827d61aeda63ce909c51403d7bea3b040d".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("tb1qrfaf3g4elgykshfgahktyaqj2r593qkrae5v95").unwrap(),
            script_pubkey: "00141a7a98a2b9fa09685d28edecb2741250e85882c3".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let utxo2 = Utxo {
            txhash: "14b3966c886a64e85829a8ed01498495f5514851121048754cc39824b54aaf7f".to_string(),
            vout: 1,
            amount: 100000,
            address: Address::from_str("tb1qrfaf3g4elgykshfgahktyaqj2r593qkrae5v95").unwrap(),
            script_pubkey: "00141a7a98a2b9fa09685d28edecb2741250e85882c3".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };

        let mut utxos = Vec::new();
        utxos.push(utxo);
        utxos.push(utxo2);

        let transaction_req_data = BtcTransaction {
            to: Address::from_str("tb1qpma75pm648xmd9tfzah029edarqn4xtndqhp99").unwrap(),
            amount: 110000,
            unspents: utxos,
            fee: 20000,
        };
        let sign_result = transaction_req_data.sign_omni_mixed_transaction(Network::Testnet, 31);

        assert_eq!(
            "020000000001020d043bea7b3d40519c90ce63daae617d826b7552fcb2a805c2081589aa82f4800100000000ffffffff7faf4ab52498c34c75481012514851f595844901eda82958e8646a886c96b3140100000000ffffffff03febc020000000000160014229ecfce73bdf1f979a55802acb9372fe223024222020000000000001600140efbea077aa9cdb69569176ef5172de8c13a99730000000000000000166a146f6d6e69000000000000001f000000000001adb00247304402206e90b497ee3fde4c86ea70e909a026216c9c0a198c2d32ff498f74aa2587e618022062f5cbd698098fdbd2723d60d8029cdce2a6c26bb9894194ae5bc5075ba48eb5012102e24f625a31c9a8bae42239f2bf945a306c01a450a03fd123316db0e837a660c00247304402206ec05587abcb777cbb5003651ec341f31fbc387202ac9b0487ca5d247024b3be02205fb3c36b8dc6159fabf9d309437fff06b14694c781a05d60564493be12f79396012102e24f625a31c9a8bae42239f2bf945a306c01a450a03fd123316db0e837a660c000000000",
            sign_result.as_ref().unwrap().signature
        );

        assert_eq!(
            "9fe3d6972506e3149ec198793ca4fbb6952c3a4e2013b0204b900b99d416a728",
            sign_result.as_ref().unwrap().tx_hash
        );
        assert_eq!(
            "2345d855d7eb4ab06841d49c5ea1129ecced229008227c469a064e15d446ac0e",
            sign_result.as_ref().unwrap().wtx_id
        );
    }
}
