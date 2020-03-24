use crate::api::AddressParam;
use common::eosapi::EosPubkeyResponse;
use crate::wallet_handler::encode_message;
use coin_eos::pubkey::EosPubkey;
use crate::error_handling::Result;

pub fn display_eos_pubkey(data: &AddressParam) -> Result<Vec<u8>> {
    let eos_pubkey = EosPubkey::display_pubkey(&data.path)?;//todo check
    let pubkey_message = EosPubkeyResponse { pubkey: eos_pubkey };
    encode_message(pubkey_message)
}

pub fn get_eos_pubkey(data: &AddressParam) -> Result<Vec<u8>> {
    let eos_pubkey = EosPubkey::get_pubkey(&data.path)?;//todo check
    let pubkey_message = EosPubkeyResponse { pubkey: eos_pubkey };
    encode_message(pubkey_message)
}
