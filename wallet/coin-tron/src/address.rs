use crate::Result;
use bitcoin::util::base58;
use keccak_hash::keccak;
use common::path::check_path_validity;
use common::apdu::{Secp256k1Apdu, ApduCheck, CoinCommonApdu};
use transport::message::send_apdu;

pub struct TronAddress {}

impl TronAddress {
    pub fn address_from_pubkey(pubkey: &[u8]) -> Result<String> {
        let pubkey_hash = keccak(pubkey[1..].as_ref());
        let address = [vec![0x41], pubkey_hash[12..].to_vec()].concat();
        let base58_address = base58::check_encode_slice(&address);
        Ok(base58_address)
    }

    pub fn get_address(path: &str) -> Result<String> {
        check_path_validity(path).unwrap();

        let select_apdu = Secp256k1Apdu::select_applet();
        let select_response = send_apdu(select_apdu)?;
        ApduCheck::checke_response(&select_response)?;

        //get public
        let msg_pubkey = Secp256k1Apdu::get_xpub(&path, false);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        ApduCheck::checke_response(&res_msg_pubkey)?;

        let pubkey_raw = hex::decode(&res_msg_pubkey[..130]).unwrap();

        let address = TronAddress::address_from_pubkey(pubkey_raw.as_slice())?;
        Ok(address)
    }

    pub fn display_address(path: &str) -> Result<String> {
        let address = TronAddress::get_address(path).unwrap();
        let reg_apdu = Secp256k1Apdu::register_address(address.as_bytes());
        let res_reg = send_apdu(reg_apdu)?;
        ApduCheck::checke_response(&res_reg)?;
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::TronAddress;
    use common::constants;
    use device::device_binding::bind_test;

    #[test]
    fn tron_address() {
        let bytes = hex::decode("04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8").unwrap();

        assert_eq!(
            TronAddress::address_from_pubkey(&bytes).unwrap(),
            "THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq"
        );
    }

    #[test]
    fn test_get_address() {
        bind_test();
        let address = TronAddress::get_address(constants::TRON_PATH).unwrap();
        println!("address:{}", &address);
        assert_eq!(&address, "");
    }

    #[test]
    fn test_display_address() {
        bind_test();
        let address = TronAddress::display_address(constants::TRON_PATH).unwrap();
        println!("address:{}", &address);
        assert_eq!(&address, "");
    }
}
