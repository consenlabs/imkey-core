use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_cosmos::address::CosmosAddress;
use coin_cosmos::cosmosapi::{CosmosAddressReq, CosmosAddressRes};
use prost::Message;

pub fn display_cosmos_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: CosmosAddressReq = CosmosAddressReq::decode(data).expect("imkey_illegal_param");
    let cosmos_address = CosmosAddress::display_address(&input.path)?;
    let address_message = CosmosAddressRes {
        address: cosmos_address,
    };
    encode_message(address_message)
}

pub fn get_cosmos_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: CosmosAddressReq = CosmosAddressReq::decode(data).expect("imkey_illegal_param");
    let cosmos_address = CosmosAddress::get_address(&input.path)?;
    let address_message = CosmosAddressRes {
        address: cosmos_address,
    };
    encode_message(address_message)
}
