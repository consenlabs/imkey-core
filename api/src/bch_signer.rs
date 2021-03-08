use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::{Address, Network};

use coin_bch::bchapi::{BchTxInput, BchTxOutput};
use coin_bch::transaction::{BchTransaction, Utxo};
use common::utility::hex_to_bytes;
use common::SignParam;
use prost::Message;
use std::str::FromStr;

pub fn sign_transaction(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    let input: BchTxInput = BchTxInput::decode(data).expect("BchTxInput");

    let mut unspents = Vec::new();
    for utxo in &input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash.to_string(),
            vout: utxo.vout,
            amount: utxo.amount,
            address: utxo.address,
            script_pubkey: utxo.script_pub_key.to_string(),
            derive_path: utxo.derived_path.to_string(),
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let bch_tx = BchTransaction {
        to: &input.to,
        amount: input.amount,
        unspents: &input.unspents,
        fee: input.fee,
    };

    let network = if input.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };

    let op_return = hex_to_bytes(&input.op_return).expect("decode btc extra op_return");

    let signed = bch_tx.sign_transaction(network, &sign_param.path, input.change_address, &op_return)?;
    let tx_sign_result = BchTxOutput {
        signature: signed.signature,
        tx_hash: signed.tx_hash,
    };
    encode_message(tx_sign_result)
}
