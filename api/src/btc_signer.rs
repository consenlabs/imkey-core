use crate::api::SignParam;
use crate::btcapi::{BtcTxInput, BtcTxOutput};
use crate::wallet_handler::encode_message;
use bitcoin::{Address, Network};
use coin_bitcoin::transaction::{BtcTransaction, Utxo};
use common::error::Error;
use prost::Message;
use std::str::FromStr;

pub fn sign_btc_transaction(param: &SignParam) -> Result<Vec<u8>, Error> {
    let input: BtcTxInput =
        BtcTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("BtcTxInput");

    let mut unspents = Vec::new();
    for utxo in input.unspents {
        let new_utxo = Utxo {
            txhash: utxo.tx_hash,
            vout: utxo.vout,
            amount: utxo.amount,
            address: Address::from_str(&utxo.address).map_err(|_err| Error::AddressError)?,
            script_pubkey: utxo.script_pub_key,
            derive_path: utxo.derived_path,
            sequence: utxo.sequence,
        };
        unspents.push(new_utxo);
    }

    let btc_tx = BtcTransaction {
        to: Address::from_str(&input.to).map_err(|_err| Error::AddressError)?,
//        change_idx: input.change_address_index as i32,
        amount: input.amount,
        unspents: unspents,
        fee: input.fee,
        payment: input.payment,
        to_dis: Address::from_str(&input.to_dis).map_err(|_err| Error::AddressError)?,
        from: Address::from_str(&input.from).map_err(|_err| Error::AddressError)?,
        fee_dis: input.fee_dis,
//        extra_data: input.extra_data,
    };

    let network = if input.network == "TESTNET".to_string() {
        Network::Testnet
    } else {
        Network::Bitcoin
    };
    let signed = btc_tx
        .sign_transaction(network, &input.path_prefix, input.change_address_index as i32, &input.extra_data)
        .map_err(|_err| Error::SignError)?;
    let tx_sign_result = BtcTxOutput {
        signature: signed.signature,
        tx_hash: signed.tx_hash,
    };
    encode_message(tx_sign_result)
}
