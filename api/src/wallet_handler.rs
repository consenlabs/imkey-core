use crate::api::{AddressParam, DeviceParam, SignParam};
use crate::deviceapi::{AppAction, AuthCode, BindCode, DeviceCert};
use crate::ethereum_address::get_eth_address;
use crate::ethereum_signer::sign_eth_transaction;
use bytes::BytesMut;
use common::error::Error;
use interface::ethereum_signer::EthereumSigner;
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
        "app_download" => Err(Error::ChainTypeError),
        "app_update" => Err(Error::ChainTypeError),
        "app_delete" => Err(Error::ChainTypeError),
        "check_cert" => Err(Error::ChainTypeError),
        "store_authcode" => Err(Error::ChainTypeError),
        "bind_acquire" => Err(Error::ChainTypeError),
        "set_device_name" => Err(Error::ChainTypeError),
        _ => Err(Error::ChainTypeError),
    }
}
