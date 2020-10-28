use crate::api::{PubKeyParam, PubKeyResult};
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_eos::pubkey::EosPubkey;
use prost::Message;

pub fn display_eos_pubkey(param: &PubKeyParam) -> Result<Vec<u8>> {
    let eos_pubkey = EosPubkey::display_pubkey(&param.path)?;
    let pubkey_message = PubKeyResult {
        path: param.path.to_string(),
        chain_type: param.chain_type.to_uppercase(),
        pub_key: eos_pubkey,
        derived_mode: "PATH_DIRECTLY".to_string(),
    };
    encode_message(pubkey_message)
}

pub fn get_eos_pubkey(param: &PubKeyParam) -> Result<Vec<u8>> {
    let eos_pubkey = EosPubkey::get_pubkey(&param.path)?;
    let pubkey_message = PubKeyResult {
        path: param.path.to_string(),
        chain_type: param.chain_type.to_uppercase(),
        pub_key: eos_pubkey,
        derived_mode: "PATH_DIRECTLY".to_string(),
    };
    encode_message(pubkey_message)
}
