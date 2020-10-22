use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_filecoin::filecoinapi::FilecoinTxReq;
use coin_filecoin::transaction::Transaction;
use prost::Message;

pub fn sign_filecoin_transaction(data: &[u8]) -> Result<Vec<u8>> {
    let input: FilecoinTxReq = FilecoinTxReq::decode(data).unwrap();
    let signed = Transaction::sign_tx(input)?;
    encode_message(signed)
}
