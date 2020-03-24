use crate::api::AddressParam;
use bitcoin::Network;
use crate::wallet_handler::encode_message;
use coin_bitcoin::btc;
use crate::btcapi::BtcAddressResponse;
use crate::error_handling::Result;


pub fn get_btc_xpub(data: &AddressParam) -> Result<Vec<u8>> {

    let network = match data.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = btc::get_xpub(network, data.path.as_ref())?;

    let address_message = BtcAddressResponse {
        address: address
    };
    encode_message(address_message)
}


pub fn get_btc_address(data: &AddressParam) -> Result<Vec<u8>> {

    let network = match data.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = btc::get_address(network, data.path.as_ref())?;

    let address_message = BtcAddressResponse {
        address: address
    };
    encode_message(address_message)
}

pub fn get_segwit_address(data: &AddressParam) -> Result<Vec<u8>> {

    let network = match data.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = btc::get_segwit_address(network, data.path.as_ref())?;

    let address_message = BtcAddressResponse {
        address: address
    };
    encode_message(address_message)
}

pub fn display_btc_address(data: &AddressParam) -> Result<Vec<u8>> {

    let network = match data.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = btc::display_address(network, data.path.as_ref())?;

    let address_message = BtcAddressResponse {
        address: address
    };
    encode_message(address_message)
}

pub fn display_segwit_address(data: &AddressParam) -> Result<Vec<u8>> {

    let network = match data.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = btc::display_segwit_address(network, data.path.as_ref())?;

    let address_message = BtcAddressResponse {
        address: address
    };
    encode_message(address_message)
}