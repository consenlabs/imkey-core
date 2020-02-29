use crate::api::SignParam;
//use crate::cosmosapi::{CosmosTxInput, CosmosTxOutput};
use crate::wallet_handler::encode_message;
use common::error::Error;
use common::sign_res::TxSignResult;
use hex;
use prost::Message;
use std::str::FromStr;
use common::cosmosapi::CosmosTxInput;
use coin_cosmos::transaction::{StdFee, Coin, Msg, MsgValue, SignData, CosmosTransaction};
use common::constants;

pub fn sign_cosmos_transaction(param: &SignParam) -> Result<Vec<u8>, Error> {
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
    let cosmosTxOutput = cosmosInput.sign();

    encode_message(cosmosTxOutput)
    /* @@XM TODO: to replace using proper cosmos api. now haven't implemented
    let eth_tx = Transaction {
        nonce: U256::from_dec_str(&input.nonce).map_err(|_err| Error::DataError)?,
        gas_price: U256::from_dec_str(&input.gas_price).map_err(|_err| Error::DataError)?,
        gas_limit: U256::from_dec_str(&input.gas_limit).map_err(|_err| Error::DataError)?,
        to: Action::Call(Address::from_str(&input.to).map_err(|_err| Error::DataError)?),
        value: U256::from_dec_str(&input.value).map_err(|_err| Error::DataError)?,
        data: input.data,
    };

    let signed = eth_tx.sign(
        Some(input.chain_id),
        &input.path,
        &input.payment,
        &input.receiver,
        &input.sender,
        &input.fee,
    )?;
    let tx_sign_result = EthTxOutput {
        signature: hex::encode(signed.0),
        tx_hash: signed.1.hash.to_string(),
    };
    encode_message(tx_sign_result)
    */
//    Err(Error::SignError)
}
