use crate::api::SignParam;
use crate::cosmosapi::{CosmosTxInput, CosmosTxOutput};
use crate::wallet_handler::encode_message;
use common::error::Error;
use common::sign_res::TxSignResult;
use hex;
use prost::Message;
use std::str::FromStr;

pub fn sign_cosmos_transaction(param: &SignParam) -> Result<Vec<u8>, Error> {
    let input: CosmosTxInput =
        CosmosTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("CosmosTxInput");

    /* @@XM TODO: to replace using proper cosmos api. now haven't implemented
    let eth_tx = Transaction {
        nonce: U256::from_dec_str(&input.nonce).map_err(|_err| Error::DataError)?,
        gas_price: U256::from_dec_str(&input.gas_price).map_err(|_err| Error::DataError)?,
        gas_limit: U256::from_dec_str(&input.gas_limit).map_err(|_err| Error::DataError)?,
        to: Action::Call(Address::from_str(&input.to).map_err(|_err| Error::DataError)?),
        value: U256::from_dec_str(&input.value).map_err(|_err| Error::DataError)?,
        data: input.data,
    };

    let signed = eth_tx.sign(
        Some(input.chain_id),
        &input.path,
        &input.payment,
        &input.receiver,
        &input.sender,
        &input.fee,
    )?;
    let tx_sign_result = EthTxOutput {
        signature: hex::encode(signed.0),
        tx_hash: signed.1.hash.to_string(),
    };
    encode_message(tx_sign_result)
    */
    Err(Error::SignError)
}
