use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_cosmos::address::CosmosAddress;

use crate::api::{AddressParam, AddressResult};
use prost::Message;

pub fn get_address(param: &AddressParam) -> Result<Vec<u8>> {
    let address = CosmosAddress::get_address(&param.path)?;
    let address_message = AddressResult {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        address,
    };
    encode_message(address_message)
}

pub fn display_cosmos_address(param: &AddressParam) -> Result<Vec<u8>> {
    let cosmos_address = CosmosAddress::display_address(&param.path)?;
    let address_message = AddressResult {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        address: cosmos_address,
    };
    encode_message(address_message)
}
