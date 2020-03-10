use crate::api::SignParam;
use crate::ethapi::{EthTxInput, EthTxOutput};
use crate::wallet_handler::encode_message;
use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use common::error::Error;
use common::sign_res::TxSignResult;
use ethereum_types::{Address, H256, U256};
use hex;
use prost::Message;
use std::str::FromStr;
use crate::error_handling::Result;

pub fn sign_eth_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: EthTxInput =
        EthTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EthTxInput");
    //
    // let eth_tx = Transaction {
    //     nonce: U256::from_dec_str(&input.nonce).map_err(|_err| Error::DataError)?,
    //     gas_price: U256::from_dec_str(&input.gas_price).map_err(|_err| Error::DataError)?,
    //     gas_limit: U256::from_dec_str(&input.gas_limit).map_err(|_err| Error::DataError)?,
    //     to: Action::Call(Address::from_str(&input.to).map_err(|_err| Error::DataError)?),
    //     value: U256::from_dec_str(&input.value).map_err(|_err| Error::DataError)?,
    //     data: input.data,
    // };
    //
    // let signed = eth_tx.sign(
    //     Some(input.chain_id),
    //     &input.path,
    //     &input.payment,
    //     &input.receiver,
    //     &input.sender,
    //     &input.fee,
    // )?;
    // let tx_sign_result = EthTxOutput {
    //     signature: hex::encode(signed.0),
    //     tx_hash: signed.1.hash.to_string(),
    // };
    // encode_message(tx_sign_result)

    let tx_sign_result = EthTxOutput {
        signature: "".to_string(),
        tx_hash: "".to_string(),
    };
    encode_message(tx_sign_result)
}
