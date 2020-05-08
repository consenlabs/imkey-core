use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_eos::eosapi::{EosMessageSignReq, EosMessageSignRes, EosTxReq};
use coin_eos::transaction::EosTransaction;
use prost::Message;

pub fn sign_eos_transaction(data: &[u8]) -> Result<Vec<u8>> {
    let input: EosTxReq = EosTxReq::decode(data).expect("imkey_illegal_param");

    let signed = EosTransaction::sign_tx(input)?;
    encode_message(signed)
}

pub fn sign_eos_message(data: &[u8]) -> Result<Vec<u8>> {
    let input: EosMessageSignReq = EosMessageSignReq::decode(data).expect("imkey_illegal_param");

    let signed = EosTransaction::sign_message(input)?;
    let mes_sign_result = EosMessageSignRes {
        signature: signed.signature,
    };
    encode_message(mes_sign_result)
}
