use crate::address::BchAddress;
use crate::common::{
    address_verify, get_address_version, get_xpub_data, secp256k1_sign_verify, TransTypeFlg,
    TxSignResult,
};
use crate::Result;
use bitcoin::blockdata::{opcodes, script::Builder};
use bitcoin::consensus::serialize;
use bitcoin::hashes::core::str::FromStr;
use bitcoin::hashes::hex::FromHex;
use bitcoin::util::psbt::serialize::Serialize;
use bitcoin::{Address, Network, OutPoint, Script, SigHashType, Transaction, TxIn, TxOut};
use bitcoin_hashes::hash160;
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::Hash;
use common::apdu::{ApduCheck, BtcApdu};
use common::common::SignParam;
use common::constants::{
    EACH_ROUND_NUMBER, MAX_OPRETURN_SIZE, MAX_UTXO_NUMBER, MIN_NONDUST_OUTPUT, TIMEOUT_LONG,
};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::{bigint_to_byte_vec, hex_to_bytes, secp256k1_sign};
use device::device_binding::KEY_MANAGER;
use secp256k1::Signature;
use transport::message::{send_apdu, send_apdu_timeout};

#[derive(Clone)]
pub struct Utxo {
    pub txhash: String,
    pub vout: i32,
    pub amount: i64,
    pub address: String,
    pub script_pubkey: String,
    pub derive_path: String,
    pub sequence: i64,
}

pub struct BchTransaction {
    pub to: String,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub fee: i64,
}

impl BchTransaction {
    pub fn sign_transaction(
        &self,
        network: Network,
        path: &str,
        change_address: String,
        op_return: &Vec<u8>,
    ) -> Result<TxSignResult> {
        //path check
        check_path_validity(path)?;
        let mut path_str = path.to_string();
        if !path.ends_with("/") {
            path_str = format!("{}{}", path_str, "/");
        }
        //check utxo number
        if &self.unspents.len() > &MAX_UTXO_NUMBER {
            return Err(CoinError::ImkeyExceededMaxUtxoNumber.into());
        }

        //get xpub and sign data
        let xpub_data = get_xpub_data(path_str.as_str(), true)?;
        let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();

        //parsing xpub data
        let sign_source_val = &xpub_data[..194];
        let sign_result = &xpub_data[194..];
        let pub_key = &sign_source_val[..130];
        let chain_code = &sign_source_val[130..];

        //use se public key verify sign
        let key_manager_obj = KEY_MANAGER.lock().unwrap();
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
        )?;

        //calc utxo total amount
        if self.get_total_amount() < self.amount {
            return Err(CoinError::ImkeyInsufficientFunds.into());
        }

        //add send to output
        let mut txouts: Vec<TxOut> = Vec::new();
        txouts.push(self.build_send_to_output());

        //add change output
        if self.get_change_amount() > MIN_NONDUST_OUTPUT {
            txouts.push(TxOut {
                value: self.get_change_amount() as u64,
                script_pubkey: BchAddress::script_pubkey(change_address.as_ref()).unwrap(),
            });
        }

        //output data serialize
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

        //add address version
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

        let btc_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x01, &output_pareper_data);
        //send output pareper command
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

            //set sequence
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
                sign_apdu_vec.push(BtcApdu::btc_segwit_sign(true, 0x41, data));
            } else {
                sign_apdu_vec.push(BtcApdu::btc_segwit_sign(false, 0x41, data));
            }

            txinputs.push(txin.clone());
        }
        tx_to_sign.input = txinputs;

        let mut txhash_vout_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x40, &txhash_vout_vec);
        let mut sequence_prepare_apdu_vec = BtcApdu::btc_prepare(0x31, 0x80, &sequence_vec);
        txhash_vout_prepare_apdu_vec.append(&mut sequence_prepare_apdu_vec);
        for apdu in txhash_vout_prepare_apdu_vec {
            ApduCheck::check_response(&send_apdu(apdu)?)?;
        }

        //send sign apdu
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (index, wegwit_sign_apdu) in sign_apdu_vec.iter().enumerate() {
            //send sign apdu
            let sign_apdu_return_data = send_apdu(wegwit_sign_apdu.clone())?;
            ApduCheck::check_response(&sign_apdu_return_data)?;
            //build signature obj
            let sign_result_vec =
                Vec::from_hex(&sign_apdu_return_data[2..sign_apdu_return_data.len() - 6]).unwrap();
            let mut signature_obj = Signature::from_compact(sign_result_vec.as_slice())?;
            signature_obj.normalize_s();
            //generator der sign data
            let mut sign_result_vec = signature_obj.serialize_der().to_vec();
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
            script_pubkey: BchAddress::script_pubkey(self.to.as_ref()).unwrap(),
        }
    }

    pub fn build_op_return_output(&self, extra_data: &Vec<u8>) -> TxOut {
        let opreturn_script = Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(&extra_data[..])
            .into_script();
        TxOut {
            value: 0u64,
            script_pubkey: opreturn_script,
        }
    }

    pub fn build_lock_script(&self, signed: &str, utxo_public_key: &str) -> Result<Script> {
        let signed_vec = Vec::from_hex(&signed)?;
        let mut signature_obj = Signature::from_compact(signed_vec.as_slice())?;
        signature_obj.normalize_s();
        let mut signed_vec = signature_obj.serialize_der().to_vec();

        //add hash type
        signed_vec.push(SigHashType::All.as_u32() as u8);
        Ok(Builder::new()
            .push_slice(&signed_vec)
            .push_slice(Vec::from_hex(utxo_public_key)?.as_slice())
            .into_script())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bchapi::BchTxInput;
    use bitcoin::{Address, Network};
    use hex::FromHex;
    use std::str::FromStr;

    use bitcoin::blockdata::transaction::SigHashType::None;
    use common::error::CoinError;
    use device::device_binding::bind_test;
    use device::device_binding::DeviceManage;
    use transport::hid_api::hid_connect;

    #[test]
    fn sign_test() {
        //binding device
        bind_test();

        let utxo = Utxo {
            txhash: "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2".to_string(),
            vout: 0,
            amount: 100000,
            address: "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r".to_string(),
            script_pubkey: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac".to_string(),
            derive_path: "0/0".to_string(),
            sequence: 0,
        };
        let mut utxos = Vec::new();
        utxos.push(utxo);
        let transaction_req_data = BchTransaction {
            to: "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s".to_string(),
            amount: 93454,
            unspents: utxos,
            fee: 502130,
        };

        let sign_result = transaction_req_data.sign_transaction(
            Network::Bitcoin,
            &"m/44'/145'/0'/".to_string(),
            "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r".to_string(),
            &vec![],
        );

        assert_eq!(
            "0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006a473044022064fb81c11181e6604aa56b29ed65e31680fc1203f5afb6f67c5437f2d68192d9022022282d6c3c35ffdf64a427df5e134aa0edb8528efb6151cb1c3b21422fdfd6e041210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a914bedf37acf35504c9bfd18b09d989d0fb23fd269688ac00000000",
            sign_result.as_ref().unwrap().signature
        );
    }
}
