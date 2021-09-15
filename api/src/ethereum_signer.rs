use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_ethereum::ethapi::{EthMessageInput, EthTxInput};
use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use common::SignParam;
use ethereum_types::{Address, U256, U64};
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

    let chain_id_parsed = input.chain_id.parse::<u64>();
    let chain_id = match chain_id_parsed {
        Ok(id) => id,
        Err(error) => {
            if input.chain_id.to_lowercase().starts_with("0x") {
                let without_prefix = &input.chain_id.trim_start_matches("0x");
                u64::from_str_radix(without_prefix, 16).unwrap()
            } else {
                u64::from_str_radix(&input.chain_id, 16).unwrap()
            }
        }
    };

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
    let signed = Transaction::sign_message(input, sign_param)?;
    encode_message(signed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use device::device_binding::{bind_test, DeviceManage};
    use ethereum_types::{Address, U256};
    use std::str::FromStr;
    use transport::hid_api::hid_connect;

    #[test]
    fn u256_from_str() {
        let ret = U256::from_str("18").unwrap();
        assert_eq!(U256::from(24i128), ret);
    }

    #[test]
    fn sign_tx_int_chainid_test() {
        bind_test();
        let path = "m/44'/60'/0'/0/0".to_string();
        let data = hex::decode("0a0134120a353030303030303030301a0730783430343964222a3078396564306335333530626331376666343535323937323265623037353830666635363463336137382a0c3078653864346135313030303a0131").unwrap();
        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path,
            network: "MAINNET".to_string(),
            input: None,
            payment: "0.000001 BNB".to_string(),
            receiver: "0x9Ed0c5350Bc17FF45529722eb07580Ff564c3a78".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "0.001316 ETH".to_string(),
        };
        let x = sign_eth_transaction(data.as_slice(), &sign_param).unwrap();
        println!("sign");
    }

    #[test]
    fn sign_tx_hex_chainid_test() {
        bind_test();
        let path = "m/44'/60'/0'/0/0".to_string();
        let data = hex::decode("0a0134120a353030303030303030301a0730783430343964222a3078396564306335333530626331376666343535323937323265623037353830666635363463336137382a0c3078653864346135313030303a0430783338").unwrap();
        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path,
            network: "MAINNET".to_string(),
            input: None,
            payment: "0.000001 BNB".to_string(),
            receiver: "0x9Ed0c5350Bc17FF45529722eb07580Ff564c3a78".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "0.001316 ETH".to_string(),
        };
        let x = sign_eth_transaction(data.as_slice(), &sign_param).unwrap();
        println!("sign");
    }

    #[test]
    fn sign_tx_hex_chainid_test2() {
        bind_test();
        let path = "m/44'/60'/0'/0/0".to_string();
        let data = hex::decode("0a0134120a353030303030303030301a0730783430343964222a3078396564306335333530626331376666343535323937323265623037353830666635363463336137382a0c3078653864346135313030303a024133").unwrap();
        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path,
            network: "MAINNET".to_string(),
            input: None,
            payment: "0.000001 BNB".to_string(),
            receiver: "0x9Ed0c5350Bc17FF45529722eb07580Ff564c3a78".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "0.001316 ETH".to_string(),
        };
        let x = sign_eth_transaction(data.as_slice(), &sign_param).unwrap();
        println!("sign");
    }

    #[test]
    fn sign_tx_error_chainid_test() {
        bind_test();
        let path = "m/44'/60'/0'/0/0".to_string();
        let data = hex::decode("0a0134120a353030303030303030301a0730783430343964222a3078396564306335333530626331376666343535323937323265623037353830666635363463336137382a0c3078653864346135313030303a03787878").unwrap();
        let sign_param = SignParam {
            chain_type: "ETHEREUM".to_string(),
            path,
            network: "MAINNET".to_string(),
            input: None,
            payment: "0.000001 BNB".to_string(),
            receiver: "0x9Ed0c5350Bc17FF45529722eb07580Ff564c3a78".to_string(),
            sender: "0x6031564e7b2F5cc33737807b2E58DaFF870B590b".to_string(),
            fee: "0.001316 ETH".to_string(),
        };
        let x = sign_eth_transaction(data.as_slice(), &sign_param).unwrap();
        println!("sign");
    }
}
