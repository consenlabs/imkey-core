use crate::api::AddressParam;
use common::eosapi::EosPubkeyResponse;
use crate::wallet_handler::encode_message;
use coin_eos::pubkey::EosPubkey;
use common::error::Error;
use common::utility::hex_to_bytes;
use mq::message::send_apdu;
use prost::Message;
use std::str::FromStr;

pub fn display_eos_pubkey(data: &AddressParam) -> Result<Vec<u8>, Error> {
    let eos_pubkey = EosPubkey::display_pubkey(&data.path).map_err(|_err| Error::PubKeyError)?;
    let pubkey_message = EosPubkeyResponse { pubkey: eos_pubkey };
    encode_message(pubkey_message)
}

pub fn get_eos_pubkey(data: &AddressParam) -> Result<Vec<u8>, Error> {
    let eos_pubkey = EosPubkey::get_pubkey(&data.path).map_err(|_err| Error::PubKeyError)?;
    let pubkey_message = EosPubkeyResponse { pubkey: eos_pubkey };
    encode_message(pubkey_message)
}
