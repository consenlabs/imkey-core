use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use common::error::Error;
use common::sign_res::TxSignResult;
use ethereum_types::{Address, H256, U256};
use hex;
use crate::Result;

pub struct EthereumSigner {}

impl EthereumSigner {
    //@@XM TODO: just for demo debugging purpose, will delete
    pub fn sign_transaction(
        nonce: U256,
        gas_price: U256,
        gas_limit: U256,
        to: Action,
        value: U256,
        data: Vec<u8>,
        chain_id: Option<u64>,
        path: &str,
        payment: &str,
        receiver: &str,
        sender: &str,
        fee: &str,
    ) -> Result<TxSignResult> {
        let eth_tx = Transaction {
            nonce: nonce,
            gas_price: gas_price,
            gas_limit: gas_limit,
            to: to,
            value: value,
            data: data,
        };

        let signed = eth_tx.sign(chain_id, path, payment, receiver, sender, fee)?;
        let tx_sign_result = TxSignResult {
            signature: hex::encode(signed.0),
            tx_hash: signed.1.hash.to_string(), //@@XM TODO: check this conversion
            wtx_id: "".to_string(),
        };
        Ok(tx_sign_result)
    }
}
