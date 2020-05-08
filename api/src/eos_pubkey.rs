use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_eos::eosapi::{EosPubkeyReq, EosPubkeyRes};
use coin_eos::pubkey::EosPubkey;
use prost::Message;

pub fn display_eos_pubkey(data: &[u8]) -> Result<Vec<u8>> {
    let input: EosPubkeyReq = EosPubkeyReq::decode(data).expect("imkey_illegal_param");
    let eos_pubkey = EosPubkey::display_pubkey(&input.path)?;
    let pubkey_message = EosPubkeyRes { pubkey: eos_pubkey };
    encode_message(pubkey_message)
}

pub fn get_eos_pubkey(data: &[u8]) -> Result<Vec<u8>> {
    let input: EosPubkeyReq = EosPubkeyReq::decode(data).expect("imkey_illegal_param");
    let eos_pubkey = EosPubkey::get_pubkey(&input.path)?;
    let pubkey_message = EosPubkeyRes { pubkey: eos_pubkey };
    encode_message(pubkey_message)
}
