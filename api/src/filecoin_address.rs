use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::Network;
use coin_filecoin::address::FilecoinAddress;
use coin_filecoin::filecoinapi::{FilecoinAddressReq, FilecoinAddressRes};
use prost::Message;

pub fn get_filecoin_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: FilecoinAddressReq = FilecoinAddressReq::decode(data).expect("imkey_illegal_param");
    let address = FilecoinAddress::get_address(input.path.as_ref(), input.network.as_ref())?;

    let address_message = FilecoinAddressRes { address };
    encode_message(address_message)
}

pub fn display_filecoin_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: FilecoinAddressReq = FilecoinAddressReq::decode(data).expect("imkey_illegal_param");
    let address = FilecoinAddress::display_address(input.path.as_ref(), input.network.as_ref())?;

    let address_message = FilecoinAddressRes { address };
    encode_message(address_message)
}
