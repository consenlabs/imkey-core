use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_tron::signer::TronSigner;
use coin_tron::tronapi::{TronMessageSignReq, TronMessageSignRes, TronTxReq};
use prost::Message;

pub fn sign_tron_transaction(data: &[u8]) -> Result<Vec<u8>> {
    let input: TronTxReq = TronTxReq::decode(data).expect("imkey_illegal_param");

    let signed = TronSigner::sign_transaction(input)?;
    encode_message(signed)
}

pub fn sign_tron_message(data: &[u8]) -> Result<Vec<u8>> {
    let input: TronMessageSignReq = TronMessageSignReq::decode(data).expect("imkey_illegal_param");

    let signed = TronSigner::sign_message(input)?;
    let mes_sign_result = TronMessageSignRes {
        signature: signed.signature,
    };
    encode_message(mes_sign_result)
}
