use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_eos::eosapi::{EosMessageSignParam, EosMessageSignResult, EosTxInput};
use coin_eos::transaction::EosTransaction;
use common::SignParam;
use prost::Message;

pub fn sign_eos_transaction(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    let input: EosTxInput = EosTxInput::decode(data).expect("imkey_illegal_param");

    let signed = EosTransaction::sign_tx(input, sign_param)?;
    encode_message(signed)
}

pub fn sign_eos_message(data: &[u8]) -> Result<Vec<u8>> {
    let input: EosMessageSignParam =
        EosMessageSignParam::decode(data).expect("imkey_illegal_param");

    let signed = EosTransaction::sign_message(input)?;
    let mes_sign_result = EosMessageSignResult {
        signature: signed.signature,
    };
    encode_message(mes_sign_result)
}
