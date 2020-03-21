use crate::api::SignParam;
use common::eosapi::{EosTxInput, EosTxOutput, EosMessageInput, EosMessageOutput};
use crate::wallet_handler::encode_message;
use prost::Message;
use coin_eos::transaction::EosTransaction;
use crate::error_handling::Result;

pub fn sign_eos_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: EosTxInput =
        EosTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EosTxInput");

    let signed = EosTransaction::sign_tx(input)?;//todo check
    encode_message(signed)
}

pub fn sign_eos_message(param: &SignParam) -> Result<Vec<u8>> {

    let input: EosMessageInput =
        EosMessageInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EosMessageInput");

    let signed = EosTransaction::sign_message(input);//todo check
    let mes_sign_result = EosMessageOutput {
        signature: signed.signature
    };
    encode_message(mes_sign_result)
}
