use crate::api::AddressParam;
use crate::wallet_handler::encode_message;
use common::error::Error;
use common::utility::hex_to_bytes;
use mq::message::send_apdu;
use prost::Message;
use std::str::FromStr;

pub fn display_cosmos_address(data: &AddressParam) -> Result<Vec<u8>, Error> {
    //@@XM TODO: add in real api since not implemented yet
    Err(Error::AddressError)
}

pub fn get_cosmos_address(data: &AddressParam) -> Result<Vec<u8>, Error> {
    //@@XM TODO: add in real api since not implemented yet
    Err(Error::AddressError)
}
