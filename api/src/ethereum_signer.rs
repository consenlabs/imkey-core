use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_ethereum::ethapi::{EthMessageInput, EthTxInput};
use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use common::SignParam;
use ethereum_types::{Address, U256};
use hex;
use prost::Message;
use std::str::FromStr;

pub fn sign_eth_transaction(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    let input: EthTxInput = EthTxInput::decode(data).expect("imkey_illegal_param");
    let data_vec = if input.data.starts_with("0x") {
        hex::decode(&input.data[2..]).unwrap()
    } else {
        hex::decode(&input.data).unwrap()
    };

    let mut to = input.to;
    if to.starts_with("0x") {
        to = to[2..].to_string();
    }

    let eth_tx = Transaction {
        nonce: parse_eth_argument(&input.nonce)?,
        gas_price: parse_eth_argument(&input.gas_price)?,
        gas_limit: parse_eth_argument(&input.gas_limit)?,
        to: Action::Call(Address::from_str(&to).unwrap()),
        value: parse_eth_argument(&input.value)?,
        data: Vec::from(data_vec.as_slice()),
    };

    let chain_id = input.chain_id.parse::<u64>().unwrap();
    let tx_out = eth_tx.sign(
        Some(chain_id),
        &sign_param.path,
        &sign_param.payment,
        &sign_param.receiver,
        &sign_param.sender,
        &sign_param.fee,
    )?;
    encode_message(tx_out)
}

fn parse_eth_argument(str: &str) -> Result<U256> {
    if str.to_lowercase().starts_with("0x") {
        U256::from_str(&str[2..].to_string())
            .map_err(|_err| format_err!("unpack eth argument error"))
    } else {
        U256::from_dec_str(str).map_err(|_err| format_err!("unpack eth argument dec error"))
    }
}

pub fn sign_eth_message(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    let input: EthMessageInput = EthMessageInput::decode(data).expect("imkey_illegal_param");
    let signed = Transaction::sign_message(input, sign_param).unwrap();
    encode_message(signed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn u256_from_str() {
        let ret = U256::from_str("18").unwrap();
        assert_eq!(U256::from(24i128), ret);
    }
}
