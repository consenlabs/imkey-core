use coin_eos::eosapi::{EosTxReq, EosMessageSignReq, EosMessageSignRes};
use crate::message_handler::encode_message;
use prost::Message;
use coin_eos::transaction::EosTransaction;
use crate::error_handling::Result;

pub fn sign_eos_transaction(data: &[u8]) -> Result<Vec<u8>> {
    let input: EosTxReq = EosTxReq::decode(data).expect("imkey_illegal_param");

    let signed = EosTransaction::sign_tx(input)?;//todo check
    encode_message(signed)
}

pub fn sign_eos_message(data: &[u8]) -> Result<Vec<u8>> {

    let input: EosMessageSignReq = EosMessageSignReq::decode(data).expect("imkey_illegal_param");

    let signed = EosTransaction::sign_message(input)?;//todo check
    let mes_sign_result = EosMessageSignRes {
        signature: signed.signature
    };
    encode_message(mes_sign_result)
}
