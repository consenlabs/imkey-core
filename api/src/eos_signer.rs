use crate::api::SignParam;
use common::eosapi::{EosTxInput, EosTxOutput};
use crate::wallet_handler::encode_message;
use common::error::Error;
use prost::Message;
use coin_eos::transaction::EosTransaction;
use crate::error_handling::Result;

pub fn sign_eos_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: EosTxInput =
        EosTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EosTxInput");

    let signed = EosTransaction::sign_tx(input).map_err(|_err| Error::SignError)?;
    let tx_sign_result = EosTxOutput {
        hash: signed.hash,
        signs: signed.signs,
    };
    encode_message(tx_sign_result)
}
