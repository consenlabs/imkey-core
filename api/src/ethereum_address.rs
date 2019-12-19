use crate::api::AddressParam;
use coin_ethereum::address::EthAddress;
use common::apdu::EthApdu;
use common::error::Error;
use common::path::check_path_validity;
use common::utility::hex_to_bytes;
use ethereum_types::{Address, H256, U256};
use hex;
use mq::message::send_apdu;
use prost::Message;
use std::str::FromStr;
use crate::wallet_handler::encode_message;

pub fn get_eth_address(data: &AddressParam) -> Result<Vec<u8>, Error> {
    //let address_param: AddressParam = AddressParam::decode(data).expect("EthTxInput");

    check_path_validity(&data.path);

    EthApdu::select_applet();

    //get public
    let msg_pubkey = EthApdu::get_pubkey(&data.path, false);
    let res_msg_pubkey = send_apdu(hex::encode(msg_pubkey));

    let pubkey_raw = hex_to_bytes(&res_msg_pubkey[2..130]).map_err(|_err| Error::PubKeyError)?;

    let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;
    encode_message(address_main)
}
