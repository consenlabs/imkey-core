use coin_ethereum::address::EthAddress;
use common::apdu;
use common::error::Error;
use common::utility::hex_to_bytes;

pub struct EthereumAddress {}

impl EthereumAddress {
    pub fn get_address(path: &String) -> Result<String, Error> {
        //@@XM TODO: check path

        apdu::Apdu::eth_select();

        //get public
        let msg_pubkey = apdu::Apdu::eth_pub(&path, false);
        //@@XM TODO: send through bluetooth

        let pubkey_res = String::from("mock for pubkey"); //@@XM TODO: replace with real result
        let pubkey_raw = hex_to_bytes(&pubkey_res[2..130]).map_err(|_err| Error::PubKeyError)?;

        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;
        Ok(address_main)
    }
}
