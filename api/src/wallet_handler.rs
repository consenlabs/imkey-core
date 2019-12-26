use crate::api::{AddressParam, DeviceParam, SignParam};
use crate::device_manager::{
    device_activate, device_app_delete, device_app_download, device_app_update,
    device_bind_acquire, device_bind_check, device_cert_check, device_display_bind_code,
    device_query, device_secure_check, device_store_authcode,
};
use crate::ethereum_address::get_eth_address;
use crate::ethereum_signer::sign_eth_transaction;
use bytes::BytesMut;
use common::error::Error;
use prost::Message;

pub fn encode_message(msg: impl Message) -> Result<Vec<u8>, Error> {
    println!("{:#?}", msg);
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf).map_err(|_err| Error::ProtoError)?;
    Ok(buf.to_vec())
}

pub fn sign_tx(data: &[u8]) -> Result<Vec<u8>, Error> {
    let param: SignParam = SignParam::decode(data).expect("SignTxParam");

    match param.chain_type.as_str() {
        "ETH" => sign_eth_transaction(&param),
        _ => Err(Error::ChainTypeError),
    }
}

pub fn get_address(data: &[u8]) -> Result<Vec<u8>, Error> {
    let param: AddressParam = AddressParam::decode(data).expect("AddressParam");

    match param.chain_type.as_str() {
        "ETH" => get_eth_address(&param),
        _ => Err(Error::ChainTypeError),
    }
}

pub fn device_manage(data: &[u8]) -> Result<Vec<u8>, Error> {
    let param: DeviceParam = DeviceParam::decode(data).expect("AddressParam");

    match param.action.as_str() {
        "app_download" => device_app_download(&param),
        "app_update" => device_app_update(&param),
        "app_delete" => device_app_delete(&param),
        "check_device_cert" => device_cert_check(&param),
        "store_authcode" => device_store_authcode(&param),
        "se_activate" => device_activate(&param),
        "se_query" => device_query(&param),
        "se_secure_check" => device_secure_check(&param),
        "bind_acquire" => device_bind_acquire(&param),
        "bind_check" => device_bind_check(&param),
        "bind_display" => device_display_bind_code(&param),
        "set_device_name" => Err(Error::DeviceOpError), //not implemeted yet
        _ => Err(Error::DeviceOpError),
    }
}
