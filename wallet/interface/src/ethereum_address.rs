use coin_ethereum::address::EthAddress;
use common::apdu;
use common::error::Error;
use common::path::check_path_validity;
use common::utility::hex_to_bytes;
use mq::message::send_apdu;

pub struct EthereumAddress {}

impl EthereumAddress {
    pub fn get_address(path: &String) -> Result<String, Error> {
        check_path_validity(path);

        apdu::Apdu::eth_select();

        //get public
        let msg_pubkey = apdu::Apdu::eth_pub(&path, false);
        let res_msg_pubkey = send_apdu(hex::encode(msg_pubkey));

        let pubkey_raw = hex_to_bytes(&res_msg_pubkey[2..130]).map_err(|_err| Error::PubKeyError)?;

        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;
        Ok(address_main)
    }
}
