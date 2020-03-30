use coin_cosmos::cosmosapi::{CosmosTxReq};
use crate::message_handler::encode_message;
use prost::Message;
use coin_cosmos::transaction::{StdFee, Coin, Msg, SignData, CosmosTransaction, MsgValue};
use crate::error_handling::Result;
use linked_hash_map::LinkedHashMap;
use itertools::Itertools;

pub fn sign_cosmos_transaction(data: &[u8]) -> Result<Vec<u8>> {
    // let input: CosmosTxReq = CosmosTxReq::decode(data).expect("imkey_illegal_param");
    let input: CosmosTxReq = CosmosTxReq::decode(data).unwrap();

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
        for item_coin in &item_msg.amount {
            let coin = Coin{
                amount: item_coin.amount.clone(),
                denom: item_coin.denom.clone()
            };
            coins.push(coin);
        }

        let mut addresses = LinkedHashMap::new();

        let mut keys = Vec::new();
        for (key, value) in &item_msg.addresses {
            addresses.insert(key.to_string(),value.to_string());
            keys.push(key);
        }
        keys.sort();
        for key in keys {
            addresses.insert(key.to_string(),item_msg.addresses.get(key).unwrap().to_string());
        }

        let msg = Msg{
            ttype: item.r#type.clone(),
            value: MsgValue {
                amount: coins,
                // delegator_address: item.value.as_ref().unwrap().delegator_address.clone(),
                // validator_address: item.value.as_ref().unwrap().validator_address.clone(),
                extra: addresses
            },
        };
        msgs.push(msg);
    }

    //SignData
    let sign_data = SignData{
        account_number: input_sign_data.account_number.clone(),
        chain_id: input_sign_data.chain_id,
        fee: stdfee,
        memo: input_sign_data.memo,
        msgs: msgs,
        sequence: input_sign_data.sequence
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
