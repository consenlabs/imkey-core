use crate::api::{AddressParam, AddressResult};
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::Network;
use coin_bitcoin::address::BtcAddress;
use coin_bitcoin::btcapi::{BtcXpubReq, BtcXpubRes};
use prost::Message;

pub fn get_btc_xpub(data: &[u8]) -> Result<Vec<u8>> {
    let input: BtcXpubReq = BtcXpubReq::decode(data).expect("imkey_illegal_param");

    let network = match input.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let xpub = BtcAddress::get_xpub(network, input.path.as_ref())?;

    let address_message = BtcXpubRes { xpub };
    encode_message(address_message)
}

pub fn get_address(param: &AddressParam) -> Result<Vec<u8>> {
    if param.is_seg_wit {
        get_segwit_address(param)
    } else {
        get_btc_legacy_address(param)
    }
}

pub fn get_btc_legacy_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::get_address(network, param.path.as_ref())?;

    let address_message = AddressResult {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        address,
    };
    encode_message(address_message)
}

pub fn get_segwit_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::get_segwit_address(network, param.path.as_ref())?;

    let address_message = AddressResult {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        address,
    };
    encode_message(address_message)
}

pub fn register_btc_address(param: &AddressParam) -> Result<Vec<u8>> {
    if param.is_seg_wit {
        display_segwit_address(param)
    } else {
        display_btc_legacy_address(param)
    }
}

pub fn display_btc_legacy_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::display_address(network, param.path.as_ref())?;

    let address_message = AddressResult {
        address,
        path: param.path.to_string(),
        chain_type: param.chain_type.to_string(),
    };
    encode_message(address_message)
}

pub fn display_segwit_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::display_segwit_address(network, param.path.as_ref())?;

    let address_message = AddressResult {
        path: param.path.to_string(),
        chain_type: param.chain_type.to_string(),
        address,
    };
    encode_message(address_message)
}
