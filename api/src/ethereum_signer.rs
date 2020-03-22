use crate::api::SignParam;
use crate::ethapi::{EthTxInput, EthTxOutput};
use crate::wallet_handler::encode_message;
use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use common::sign_res::TxSignResult;
use ethereum_types::{Address, H256, U256};
use hex;
use prost::Message;
use std::str::FromStr;
use crate::error_handling::Result;
use common::ethapi::{EthPersonalSignInput, EthPersonalSignOutput};

pub fn sign_eth_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: EthTxInput =
        EthTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EthTxInput");
    let mut data = "".to_string();


    if input.data.starts_with("0x"){
        data = hex::encode(&data[2..]);
    }

    let eth_tx = Transaction {
        nonce: U256::from_dec_str(&input.nonce).unwrap(),
        gas_price: U256::from_dec_str(&input.gas_price).unwrap(),
        gas_limit: U256::from_dec_str(&input.gas_limit).unwrap(),
        to: Action::Call(Address::from_str(&input.to).unwrap()),
        value: U256::from_dec_str(&input.value).unwrap(),
        data: Vec::from(data),
    };

    let chain_id = input.chain_id.parse::<u64>().unwrap();
    let tx_out = eth_tx.sign(
        Some(chain_id),
        &input.path,
        &input.payment,
        &input.receiver,
        &input.sender,
        &input.fee,
    )?;
    encode_message(tx_out)

    // let tx_sign_result = EthTxOutput {
    //     signature: "".to_string(),
    //     tx_hash: "".to_string(),
    // };
    // encode_message(tx_sign_result)
}

pub fn sign_eth_message(param: &SignParam) -> Result<Vec<u8>> {
    let input: EthPersonalSignInput =
        EthPersonalSignInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EosMessageInput");
    let signed = Transaction::sign_persional_message(input);//todo check
    encode_message(signed)
}

