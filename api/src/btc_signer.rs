use crate::api::SignParam;
use crate::btcapi::{BtcTxInput, BtcTxOutput};
use crate::wallet_handler::encode_message;
use bitcoin::{Address, Network};
use coin_bitcoin::transaction::{BtcTransaction, Utxo};
use prost::Message;
use std::str::FromStr;
use crate::error_handling::Result;

pub fn sign_btc_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: BtcTxInput =
        BtcTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("BtcTxInput");

    let mut unspents = Vec::new();
    for utxo in input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash,
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),//todo check
            script_pubkey: utxo.script_pub_key,
            derive_path: utxo.derived_path,
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&input.to).unwrap(),//todo check
//        change_idx: input.change_address_index as i32,
        amount: input.amount,
        unspents: unspents,
        fee: input.fee,
        payment: input.payment,
        to_dis: Address::from_str(&input.to_dis).unwrap(),//todo check
        from: Address::from_str(&input.from).unwrap(),//todo check
        fee_dis: input.fee_dis,
//        extra_data: input.extra_data,
    };

    let network = if input.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx
        .sign_transaction(network, &input.path_prefix, input.change_address_index as i32, &input.extra_data)?;//todo check
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        tx_hash: signed.tx_hash,
        wtx_id: signed.wtx_id,
    };
    encode_message(tx_sign_result)
}

pub fn sign_segwit_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: BtcTxInput =
        BtcTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("BtcTxInput");

    let mut unspents = Vec::new();
    for utxo in input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash,
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).unwrap(),//todo check
            script_pubkey: utxo.script_pub_key,
            derive_path: utxo.derived_path,
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&input.to).unwrap(),//todo check
//        change_idx: input.change_address_index as i32,
        amount: input.amount,
        unspents: unspents,
        fee: input.fee,
        payment: input.payment,
        to_dis: Address::from_str(&input.to_dis).unwrap(),//todo check
        from: Address::from_str(&input.from).unwrap(),//todo check
        fee_dis: input.fee_dis,
//        extra_data: input.extra_data,
    };

    let network = if input.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx
        .sign_segwit_transaction(network, &input.path_prefix, input.change_address_index as i32, &input.extra_data)?;//todo check
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        tx_hash: signed.tx_hash,
        wtx_id: signed.wtx_id,
    };
    encode_message(tx_sign_result)
}
