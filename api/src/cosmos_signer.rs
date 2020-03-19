use crate::api::SignParam;
//use crate::cosmosapi::{CosmosTxInput, CosmosTxOutput};
use crate::wallet_handler::encode_message;
use common::sign_res::TxSignResult;
use hex;
use prost::Message;
use std::str::FromStr;
use common::cosmosapi::{CosmosTxInput, CosmosTxOutput};
use coin_cosmos::transaction::{StdFee, Coin, Msg, MsgValue, SignData, CosmosTransaction};
use common::constants;
use crate::error_handling::Result;

pub fn sign_cosmos_transaction(param: &SignParam) -> Result<Vec<u8>> {
    let input: CosmosTxInput =
        CosmosTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("CosmosTxInput");

    // fee
    let mut coins = Vec::new();

    let input_sign_data = input.sign_data.unwrap();
    let input_fee = &input_sign_data.fee.unwrap();
    for itme in &input_fee.amount {
        let coin = Coin{
            amount: itme.amount.clone(),
            denom: itme.denom.clone()
        };
        coins.push(coin);
    }
    let stdfee = StdFee{
        amount: coins,
        gas: input_fee.gas.clone()
    };

    //msgs
    let mut msgs = Vec::new();
    for item in &input_sign_data.msgs {
        let mut coins = Vec::new();
        let item_msg = item.value.as_ref().unwrap();
        for itemCoin in &item_msg.amount {
            let coin = Coin{
                amount: itemCoin.amount.clone(),
                denom: itemCoin.denom.clone()
            };
            coins.push(coin);
        }

        let msg = Msg{
            ttype: item.r#type.clone(),
            value: MsgValue{
                amount: coins,
                delegator_address: item.value.as_ref().unwrap().delegator_address.clone(),
                validator_address: item.value.as_ref().unwrap().validator_address.clone(),
            },
        };
        msgs.push(msg);
    }

    //SignData
    let sign_data = SignData{
        account_number: input_sign_data.account_number.clone(),
        chain_id: input_sign_data.chain_id,
        fee: stdfee,
        memo: Some(input_sign_data.memo),
        msgs: msgs,
        sequence: input_sign_data.sequence
    };

    let mut cosmosInput = CosmosTransaction {
        sign_data: sign_data,
        path: input.path,
        payment_dis: input.payment_dis,
        to_dis: input.to_dis,
        fee_dis: input.fee_dis,
    };
    let cosmosTxOutput = cosmosInput.sign()?;

    encode_message(cosmosTxOutput)
}
