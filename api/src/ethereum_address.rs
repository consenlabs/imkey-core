use crate::api::AddressParam;
use crate::wallet_handler::encode_message;
use coin_ethereum::address::EthAddress;
use common::apdu::EthApdu;
use common::path::check_path_validity;
use common::utility::hex_to_bytes;
use ethereum_types::{Address, H256, U256};
use hex;
use mq::message::send_apdu;
use prost::Message;
use std::str::FromStr;
use crate::ethapi::EthAddressResponse;
use crate::error_handling::Result;

pub fn get_eth_address(data: &AddressParam) -> Result<Vec<u8>> {
    //let address_param: AddressParam = AddressParam::decode(data).expect("EthTxInput");

    check_path_validity(&data.path);

    let select_apdu = EthApdu::select_applet();
    let select_response = send_apdu(select_apdu);

    //get public
    let msg_pubkey = EthApdu::get_pubkey(&data.path, false);
    let res_msg_pubkey = send_apdu(msg_pubkey);

    let pubkey_raw = hex_to_bytes(&res_msg_pubkey[0..130])?;

    let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;//todo check
    let address_message = EthAddressResponse { address: address_main };
    encode_message(address_message)
}

pub fn display_eth_address(data: &AddressParam) -> Result<Vec<u8>> {
    check_path_validity(&data.path);

    let select_apdu = EthApdu::select_applet();
    let select_response = send_apdu(select_apdu);

    //get public
    let msg_pubkey = EthApdu::get_pubkey(&data.path, false);
    let res_msg_pubkey = send_apdu(msg_pubkey);

    let pubkey_raw = hex_to_bytes(&res_msg_pubkey[0..130])?;//todo check

    let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;
    let reg_apdu = EthApdu::register_address(address_main.as_bytes());
    let res_reg = send_apdu(reg_apdu);
    let address_message = EthAddressResponse { address: address_main };
    encode_message(address_message)
}
