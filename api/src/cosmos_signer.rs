use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_cosmos::cosmosapi::CosmosTxInput;
use coin_cosmos::transaction::{Coin, CosmosTransaction, SignData, StdFee};
use common::SignParam;
use prost::Message;

pub fn sign_cosmos_transaction(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    // let input: CosmosTxReq = CosmosTxReq::decode(data).expect("imkey_illegal_param");
    let input: CosmosTxInput = CosmosTxInput::decode(data).unwrap();

    // fee
    let mut coins = Vec::new();

    // let input_sign_data = input.sign_data.unwrap();
    let input_fee = &input.fee.expect("cosmos tx input std fee");
    // todo: why foreach for option
    for item in &input_fee.amount {
        let coin = Coin {
            amount: item.amount.clone(),
            denom: item.denom.clone(),
        };
        coins.push(coin);
    }
    let stdfee = StdFee {
        amount: coins,
        gas: input_fee.gas.clone(),
    };

    let msg_without_slash = input.msgs.replace("\\", "");

    let r = serde_json::from_str(&msg_without_slash).unwrap();

    //SignData
    let sign_data = SignData {
        account_number: input.account_number.clone(),
        chain_id: input.chain_id,
        fee: stdfee,
        memo: input.memo,
        msgs: r,
        sequence: input.sequence,
    };

    let cosmos_input = CosmosTransaction {
        sign_data,
        path: sign_param.path.to_string(),
        payment_dis: sign_param.payment.to_string(),
        to_dis: sign_param.receiver.to_string(),
        fee_dis: sign_param.fee.to_string(),
    };
    let cosmos_tx_output = cosmos_input.sign()?;

    encode_message(cosmos_tx_output)
}
