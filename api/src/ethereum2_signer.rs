use crate::error_handling::Result;
use crate::message_handler::encode_message;
use coin_ethereum2::eth2api::Eth2MsgSignInput;
use coin_ethereum2::transaction::Eth2Sign;
use coin_substrate::substrateapi::SubstrateRawTxIn;
use common::SignParam;
use prost::Message;

pub fn sign_eth2_message(data: &[u8], sign_param: &SignParam) -> Result<Vec<u8>> {
    let sign_data: Eth2MsgSignInput = Eth2MsgSignInput::decode(data).expect("imkey_illegal_param");
    let signed = Eth2Sign::msg_sign(sign_data, sign_param)?;
    encode_message(signed)
}
