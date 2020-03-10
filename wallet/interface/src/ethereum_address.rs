use coin_ethereum::address::EthAddress;
use common::apdu::{EthApdu, EosApdu};
use common::error::Error;
use common::path::check_path_validity;
use common::utility::hex_to_bytes;
use mq::message::send_apdu;
use crate::Result;

pub struct EthereumAddress {}

impl EthereumAddress {
    pub fn get_address(path: &str) -> Result<String> {
        check_path_validity(path);

        let select_apdu = EthApdu::select_applet();
        let select_response = send_apdu(select_apdu);

        //get public
        let msg_pubkey = EthApdu::get_pubkey(&path, false);
        let res_msg_pubkey = send_apdu(msg_pubkey);

//        let pubkey_raw =
//            hex_to_bytes(&res_msg_pubkey[2..130]).map_err(|_err| Error::PubKeyError)?;//TODO
        let pubkey_raw =
            hex_to_bytes(&res_msg_pubkey[2..130]).map_err(|_err| Error::PubKeyError).expect("hex_to_bytes_error");

        let address_main = EthAddress::address_from_pubkey(pubkey_raw.clone())?;
        Ok(address_main)
    }

    pub fn display_address(path: &str) -> Result<String> {
        let address = EthereumAddress::get_address(path).unwrap();
        let reg_apdu = EthApdu::register_address(address.as_bytes());
        let res_reg = send_apdu(reg_apdu);
        //todo: check response
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use common::constants;
    use crate::ethereum_address::EthereumAddress;

    #[test]
    fn test_get_address() {
        let address = EthereumAddress::get_address(constants::ETH_PATH);
        println!("address:{}",address.unwrap());
    }

    #[test]
    fn test_display_address() {
        let address = EthereumAddress::display_address(constants::ETH_PATH);
        println!("address:{}", address.unwrap());
    }
}
