use crate::api::{AddressParam, DeviceParam, SignParam};
use crate::btc_signer::{sign_btc_transaction, sign_segwit_transaction};
use crate::usdt_signer::{sign_usdt_transaction, sign_usdt_segwit_transaction};
use crate::btc_address::{get_btc_xpub, get_btc_address, get_segwit_address};
use crate::cosmos_address::{display_cosmos_address, get_cosmos_address};
use crate::cosmos_signer::sign_cosmos_transaction;
use crate::device_manager::{device_activate, device_app_delete, device_app_download, device_app_update, device_bind_acquire, device_bind_check, device_display_bind_code, device_query, device_secure_check, get_seid, get_sn, get_ram_size, get_firmware_version, get_battery_power, get_life_time, get_ble_name, set_ble_name, get_ble_version, get_sdk_info};
use crate::eos_pubkey::{display_eos_pubkey, get_eos_pubkey};
use crate::eos_signer::{sign_eos_transaction, sign_eos_message};
use crate::ethereum_address::{get_eth_address, display_eth_address};
use crate::ethereum_signer::sign_eth_transaction;
use bytes::BytesMut;
use prost::Message;
use crate::error_handling::Result;

pub fn encode_message(msg: impl Message) -> Result<Vec<u8>> {
    println!("{:#?}", msg);
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
//    msg.encode(&mut buf).map_err(|_err| Error::ProtoError)?;
        msg.encode(&mut buf)?;//TODO
    Ok(buf.to_vec())
}

pub fn sign_tx(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignParam = SignParam::decode(data).expect("SignTxParam");

    match param.chain_type.as_str() {
        "BTC" => sign_btc_transaction(&param),
        "BTC_SEGWIT" => sign_segwit_transaction(&param),
        "OMINI" => sign_usdt_transaction(&param),
        "OMINI_SEGWIT" => sign_usdt_segwit_transaction(&param),
        "ETH" => sign_eth_transaction(&param),
        "EOS" => sign_eos_transaction(&param),
        "COSMOS" => sign_cosmos_transaction(&param),
        _ => Err(format_err!("unsupported chain")),//TODO
    }
}

pub fn sign_msg(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignParam = SignParam::decode(data).expect("SignMsgParam");
    match param.chain_type.as_str() {
        "ETH" => sign_eth_transaction(&param),
        "EOS" => sign_eos_message(&param),
        _ => Err(format_err!("unsupported chain")),//TODO
    }
}

pub fn get_address(data: &[u8]) -> Result<Vec<u8>> {
    let param: AddressParam = AddressParam::decode(data).expect("AddressParam");

    match param.chain_type.as_str() {
        "BTC" => get_btc_address(&param),
        "BTC_XPUB" => get_btc_xpub(&param),
        "BTC_SEGWIT" => get_segwit_address(&param),
        "ETH" => get_eth_address(&param),
        "EOS" => get_eos_pubkey(&param),
        "COSMOS" => get_cosmos_address(&param),
//        _ => Err(Error::ChainTypeError),
        _ => Err(format_err!("unsupported chain")),//TODO
    }
}

pub fn register_coin(data: &[u8]) -> Result<Vec<u8>> {
    let param: AddressParam = AddressParam::decode(data).expect("AddressParam");

    match param.chain_type.as_str() {
        "ETH" => display_eth_address(&param),
        "EOS" => display_eos_pubkey(&param),
        "COSMOS" => display_cosmos_address(&param),
//        _ => Err(Error::ChainTypeError),
        _ => Err(format_err!("unsupported chain")),//TODO
    }
}

pub fn device_manage(data: &[u8]) -> Result<Vec<u8>> {
    let param: DeviceParam = DeviceParam::decode(data).expect("AddressParam");

    match param.action.as_str() {
        "app_download" => device_app_download(&param),
        "app_update" => device_app_update(&param),
        "app_delete" => device_app_delete(&param),
        "se_activate" => device_activate(&param),
        "check_update" => device_query(&param),
        "se_secure_check" => device_secure_check(&param),
        "bind_acquire" => device_bind_acquire(&param),
        "bind_check" => device_bind_check(&param),
        "bind_display" => device_display_bind_code(&param),
        "get_seid" => get_seid(&param),
        "get_sn" => get_sn(&param),
        "get_ram_size" => get_ram_size(&param),
        "get_firmware_version" => get_firmware_version(&param),
        "get_battery_power" => get_battery_power(&param),
        "get_life_time" => get_life_time(&param),
        "get_ble_name" => get_ble_name(&param),
        "set_ble_name" => set_ble_name(&param),
        "get_ble_version" => get_ble_version(&param),
        "get_sdk_info" => get_sdk_info(&param),
//        _ => Err(Error::DeviceOpError),
        _ => Err(format_err!("device_open_error")),//TODO
    }
}
