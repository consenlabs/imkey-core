use crate::message_handler::encode_message;
use coin_bitcoin::btcapi::{BtcTxReq, BtcTxRes, BtcSegwitTxReq, BtcSegwitTxRes};
use bitcoin::{Address, Network};
use coin_bitcoin::transaction::{BtcTransaction, Utxo};
use prost::Message;
use std::str::FromStr;
use crate::error_handling::Result;

pub fn sign_btc_transaction(data: &[u8]) -> Result<Vec<u8>> {
    let input: BtcTxReq = BtcTxReq::decode(data).expect("BtcTxInput");

    let mut unspents = Vec::new();
    for utxo in input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash,
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),
            script_pubkey: utxo.script_pub_key,
            derive_path: utxo.derived_path,
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
//        extra_data: input.extra_data,
    };

    let network = if input.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx
        .sign_transaction(network, &input.path_prefix, input.change_address_index as i32, &input.extra_data)?;
    let tx_sign_result = BtcTxRes {
        tx_data: signed.signature,
        tx_hash: signed.tx_hash,
    };
    encode_message(tx_sign_result)
}

pub fn sign_segwit_transaction(data: &[u8]) -> Result<Vec<u8>> {
    let input: BtcSegwitTxReq = BtcSegwitTxReq::decode(data).expect("BtcTxInput");

    let mut unspents = Vec::new();
    for utxo in input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash,
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),
            script_pubkey: utxo.script_pub_key,
            derive_path: utxo.derived_path,
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

    let network = if input.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx
        .sign_segwit_transaction(network, &input.path_prefix, input.change_address_index as i32, &input.extra_data)?;
    let tx_sign_result = BtcSegwitTxRes {
        witness_tx_data: signed.signature,
        wtx_hash: signed.wtx_id,
        tx_hash: signed.tx_hash,
    };
    encode_message(tx_sign_result)
}
