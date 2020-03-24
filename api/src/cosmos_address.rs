use crate::api::AddressParam;
use crate::wallet_handler::encode_message;
use coin_cosmos::address::CosmosAddress;
use common::cosmosapi::CosmosAddressResponse;
use crate::error_handling::Result;

pub fn display_cosmos_address(data: &AddressParam) -> Result<Vec<u8>> {
    let cosmos_address = CosmosAddress::display_address(&data.path)?;//todo check
    let address_message = CosmosAddressResponse { address: cosmos_address };
    encode_message(address_message)
}

pub fn get_cosmos_address(data: &AddressParam) -> Result<Vec<u8>> {
    let cosmos_address = CosmosAddress::get_address(&data.path)?;//todo check
    let address_message = CosmosAddressResponse { address: cosmos_address };
    encode_message(address_message)
}
