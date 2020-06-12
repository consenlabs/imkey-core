use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_cosmos::cosmosapi::CosmosTxReq;
use coin_cosmos::transaction::{Coin, CosmosTransaction, SignData, StdFee};
use prost::Message;

pub fn sign_cosmos_transaction(data: &[u8]) -> Result<Vec<u8>> {
    // let input: CosmosTxReq = CosmosTxReq::decode(data).expect("imkey_illegal_param");
    let input: CosmosTxReq = CosmosTxReq::decode(data).unwrap();

    // fee
    let mut coins = Vec::new();

    let input_sign_data = input.sign_data.unwrap();
    let input_fee = &input_sign_data.fee.unwrap();
    for itme in &input_fee.amount {
        let coin = Coin {
            amount: itme.amount.clone(),
            denom: itme.denom.clone(),
        };
        coins.push(coin);
    }
    let stdfee = StdFee {
        amount: coins,
        gas: input_fee.gas.clone(),
    };

    let msg_witout_slash = input_sign_data.msgs.replace("\\", "");

    let r = serde_json::from_str(&msg_witout_slash).unwrap();

    //SignData
    let sign_data = SignData {
        account_number: input_sign_data.account_number.clone(),
        chain_id: input_sign_data.chain_id,
        fee: stdfee,
        memo: input_sign_data.memo,
        msgs: r,
        sequence: input_sign_data.sequence,
    };

    let cosmos_input = CosmosTransaction {
        sign_data,
        path: input.path,
        payment_dis: input.payment_dis,
        to_dis: input.to_dis,
        fee_dis: input.fee_dis,
    };
    let cosmos_tx_output = cosmos_input.sign()?;

    encode_message(cosmos_tx_output)
}
