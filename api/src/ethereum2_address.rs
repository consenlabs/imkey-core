use crate::api::PubKeyResult;
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use crate::PubKeyParam;
use coin_ethereum2::address::Eth2Address;
use prost::Message;

pub fn get_pub_key(param: &PubKeyParam) -> Result<Vec<u8>> {
    let pub_key = Eth2Address::get_pub_key(&param.path)?;
    let return_message = PubKeyResult {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        pub_key: pub_key,
        derived_mode: "".to_string(),
    };
    encode_message(return_message)
}
