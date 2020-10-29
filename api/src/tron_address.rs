use crate::error_handling::Result;
use crate::message_handler::encode_message;
use prost::Message;
use coin_tron::tronapi::{TronAddressReq, TronAddressRes};
use coin_tron::address::TronAddress;

pub fn get_tron_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: TronAddressReq = TronAddressReq::decode(data).expect("imkey_illegal_param");
    let address = TronAddress::get_address(&input.path)?;
    let address_message = TronAddressRes {
        address,
    };
    encode_message(address_message)
}

pub fn display_tron_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: TronAddressReq = TronAddressReq::decode(data).expect("imkey_illegal_param");
    let address = TronAddress::display_address(&input.path)?;
    let address_message = TronAddressRes {
        address,
    };
    encode_message(address_message)
}
