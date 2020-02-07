use crate::api::SignParam;
use common::eosapi::{EosTxInput, EosTxOutput};
use crate::wallet_handler::encode_message;
use coin_eos::transaction::{EosSignData as EosSignDataFinal, EosTransaction};
use common::error::Error;
use prost::Message;

pub fn sign_eos_transaction(param: &SignParam) -> Result<Vec<u8>, Error> {
    let input: EosTxInput =
        EosTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("EosTxInput");

    let mut sign_datas = Vec::new();
    for sign_data in input.sign_datas {
        let new_sign_data = EosSignDataFinal {
            tx_data: sign_data.tx_hash,
            pub_keys: sign_data.pub_keys,
            chain_id: sign_data.chain_id,
            to: sign_data.to,
            from: sign_data.from,
            payment: sign_data.payment,
        };
        sign_datas.push(new_sign_data);
    }

    let mut eos_tx = EosTransaction {
        path: input.path,
        sign_datas: sign_datas,
    };

    let signed = eos_tx.sign_tx().map_err(|_err| Error::SignError)?;
    let tx_sign_result = EosTxOutput {
        hash: signed.hash,
        signs: signed.signs,
    };
    encode_message(tx_sign_result)
}
