use crate::api::{AddressParam, SignParam};
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
