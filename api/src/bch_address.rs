use crate::api::{
    AddressParam, AddressResult, BitcoinWallet, ExternalAddress, ExternalAddressParam,
};
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::Network;
use coin_bch::address::BchAddress;
use prost::Message;

pub fn get_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let account_path = param.path.to_string();
    let main_address: String;
    let receive_address: String;

    main_address = BchAddress::get_address(network, format!("{}/0/0", account_path).as_str())?;
    receive_address = BchAddress::get_address(network, format!("{}/0/1", account_path).as_str())?;

    let external_address = ExternalAddress {
        address: receive_address,
        derived_path: "0/1".to_string(),
        r#type: "EXTERNAL".to_string(),
    };

    let address_message = BitcoinWallet {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        address: main_address,
        enc_x_pub: "",
        external_address: Some(external_address),
    };
    encode_message(address_message)
}

pub fn register_bch_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let path = format!("{}/0/0", param.path);
    let address = BchAddress::display_address(network, &path)?;

    let address_message = AddressResult {
        path: param.path.to_string(),
        chain_type: param.chain_type.to_string(),
        address,
    };
    encode_message(address_message)
}
