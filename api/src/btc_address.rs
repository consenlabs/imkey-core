use coin_bitcoin::btcapi::{BtcXpubReq, BtcXpubRes, BtcAddressReq, BtcAddressRes};
use bitcoin::Network;
use crate::message_handler::encode_message;
use coin_bitcoin::address::BtcAddress;
use crate::error_handling::Result;
use prost::Message;

pub fn get_btc_xpub(data: &[u8]) -> Result<Vec<u8>> {

    let input: BtcXpubReq = BtcXpubReq::decode(data).expect("imkey_illegal_param");

    let network = match input.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let xpub = BtcAddress::get_xpub(network, input.path.as_ref())?;

    let address_message = BtcXpubRes {
        xpub
    };
    encode_message(address_message)
}


pub fn get_btc_address(data: &[u8]) -> Result<Vec<u8>> {

    let input: BtcAddressReq = BtcAddressReq::decode(data).expect("imkey_illegal_param");

    let network = match input.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::get_address(network, input.path.as_ref())?;

    let address_message = BtcAddressRes {
        address
    };
    encode_message(address_message)
}

pub fn get_segwit_address(data: &[u8]) -> Result<Vec<u8>> {

    let input: BtcAddressReq = BtcAddressReq::decode(data).expect("imkey_illegal_param");

    let network = match input.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::get_segwit_address(network, input.path.as_ref())?;

    let address_message = BtcAddressRes {
        address
    };
    encode_message(address_message)
}

pub fn display_btc_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: BtcAddressReq = BtcAddressReq::decode(data).expect("imkey_illegal_param");

    let network = match input.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::display_address(network, input.path.as_ref())?;

    let address_message = BtcAddressRes {
        address
    };
    encode_message(address_message)
}

pub fn display_segwit_address(data: &[u8]) -> Result<Vec<u8>> {
    let input: BtcAddressReq = BtcAddressReq::decode(data).expect("imkey_illegal_param");
    let network = match input.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BtcAddress::display_segwit_address(network, input.path.as_ref())?;

    let address_message = BtcAddressRes {
        address
    };
    encode_message(address_message)
}