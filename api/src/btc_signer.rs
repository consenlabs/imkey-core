use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::{Address, Network};

use coin_bitcoin::btcapi::{BtcTxInput, BtcTxOutput};
use coin_bitcoin::transaction::{BtcTransaction, Utxo};
use common::SignParam;
use prost::Message;
use std::str::FromStr;

pub fn sign_btc_transaction(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    let input: BtcTxInput = BtcTxInput::decode(data).expect("BtcTxInput");

    if (input.protocol.to_uppercase() == "OMNI") {
        if input.is_seg_wit {
            sign_usdt_segwit_transaction(&input, sign_param)
        } else {
            sign_usdt_transaction(&input, sign_param)
        }
    } else {
        if input.is_seg_wit {
            sign_segwit_transaction(&input, sign_param)
        } else {
            sign_legacy_transaction(&input, sign_param)
        }
    }
}

pub fn sign_legacy_transaction(param: &BtcTxInput, sign_param: &SignParam) -> Result<Vec<u8>> {
    let mut unspents = Vec::new();
    for utxo in &param.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash.to_string(),
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),
            script_pubkey: utxo.script_pub_key.to_string(),
            derive_path: utxo.derived_path.to_uppercase(),
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&param.to).unwrap(),
        //        change_idx: input.change_address_index as i32,
        amount: param.amount,
        unspents: unspents,
        fee: param.fee,
        //        extra_data: input.extra_data,
    };

    let network = if sign_param.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx.sign_transaction(
        network,
        &sign_param.path,
        param.change_address_index as i32,
        &param.extra_data,
    )?;
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        tx_hash: signed.tx_hash,
        wtx_hash: "".to_string(),
    };
    encode_message(tx_sign_result)
}

pub fn sign_segwit_transaction(param: &BtcTxInput, sign_param: &SignParam) -> Result<Vec<u8>> {
    let mut unspents = Vec::new();
    for utxo in &param.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash.to_string(),
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),
            script_pubkey: utxo.script_pub_key.to_string(),
            derive_path: utxo.derived_path.to_string(),
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&param.to).unwrap(),
        //        change_idx: input.change_address_index as i32,
        amount: param.amount,
        unspents: unspents,
        fee: param.fee,
    };

    let network = if sign_param.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx.sign_segwit_transaction(
        network,
        &sign_param.path,
        param.change_address_index as i32,
        &param.extra_data,
    )?;
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        wtx_hash: signed.wtx_id,
        tx_hash: signed.tx_hash,
    };
    encode_message(tx_sign_result)
}

pub fn sign_usdt_transaction(input: &BtcTxInput, sign_param: &SignParam) -> Result<Vec<u8>> {
    let mut unspents = Vec::new();
    for utxo in &input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash.to_string(),
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),
            script_pubkey: utxo.script_pub_key.to_string(),
            derive_path: utxo.derived_path.to_string(),
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&input.to).unwrap(),
        amount: input.amount,
        unspents: unspents,
        fee: input.fee,
    };

    let network = if sign_param.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed =
        btc_tx.sign_omni_transaction(network, &sign_param.path, input.property_id as i32)?;
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        tx_hash: signed.tx_hash,
        wtx_hash: "".to_string(),
    };
    encode_message(tx_sign_result)
}

pub fn sign_usdt_segwit_transaction(input: &BtcTxInput, sign_param: &SignParam) -> Result<Vec<u8>> {
    let mut unspents = Vec::new();
    for utxo in &input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash.to_string(),
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),
            script_pubkey: utxo.script_pub_key.to_string(),
            derive_path: utxo.derived_path.to_string(),
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&input.to).unwrap(),
        //        change_idx: input.change_address_index as i32,
        amount: input.amount,
        unspents: unspents,
        fee: input.fee,
    };

    let network = if sign_param.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed =
        btc_tx.sign_omni_segwit_transaction(network, &sign_param.path, input.property_id as i32)?;
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        wtx_hash: signed.wtx_id,
        tx_hash: signed.tx_hash,
    };
    encode_message(tx_sign_result)
}
