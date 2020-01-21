use crate::api::AddressParam;
use crate::wallet_handler::encode_message;
use common::error::Error;
use common::utility::hex_to_bytes;
use mq::message::send_apdu;
use prost::Message;
use std::str::FromStr;
use coin_cosmos::address::CosmosAddress;
use crate::cosmosapi::CosmosAddressResponse;

pub fn display_cosmos_address(data: &AddressParam) -> Result<Vec<u8>, Error> {
    let cosmos_address = CosmosAddress::display_address(&data.path).map_err(|_err| Error::PubKeyError)?;
    let address_message = CosmosAddressResponse { address: cosmos_address };
    encode_message(address_message)
}

pub fn get_cosmos_address(data: &AddressParam) -> Result<Vec<u8>, Error> {
    let cosmos_address = CosmosAddress::get_address(&data.path).map_err(|_err| Error::PubKeyError)?;
    let address_message = CosmosAddressResponse { address: cosmos_address };
    encode_message(address_message)
}
