use coin_ethereum::ethapi::{EthAddressReq, EthAddressRes};
use crate::message_handler::encode_message;
use coin_ethereum::address::EthAddress;
use crate::error_handling::Result;
use prost::Message;

pub fn get_eth_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: EthAddressReq = EthAddressReq::decode(data).expect("imkey_illegal_param");
    let address = EthAddress::get_address(&input.path).unwrap();
    let address_message = EthAddressRes{
        address
    };
    encode_message(address_message)
}

pub fn display_eth_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: EthAddressReq = EthAddressReq::decode(data).expect("imkey_illegal_param");
    let address = EthAddress::display_address(&input.path).unwrap();
    let address_message = EthAddressRes{
        address
    };
    encode_message(address_message)
}
