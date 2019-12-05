use common::error::Error;
use hex;
use keccak_hash::keccak;
use regex::Regex;

#[derive(Debug)]
pub struct EthAddress {}

impl EthAddress {
    pub fn address_from_pubkey(pubkey: Vec<u8>) -> Result<String, Error> {
        //length check
        if pubkey.len() != 64 {
            return Err(Error::PubKeyError);
        }

        let pubkey_hash = keccak(pubkey);
        let addr_bytes = &pubkey_hash[12..];
        Ok(hex::encode(addr_bytes))
    }

    pub fn address_checksummed(address: &str) -> String {
        let re = Regex::new(r"^0x").unwrap();
        let address = address.to_lowercase();
        let address = re.replace_all(&address, "").to_string();

        let mut checksum_address = "0x".to_string();
        /*
        let mut hasher = Sha3::keccak256();
        hasher.input_str(&address);
        let address_hash = hasher.result_str();
        */

        let address_hash = keccak(&address);
        let address_hash_hex = hex::encode(address_hash); //@@XM TODO: checkt this encode

        for i in 0..address.len() {
            let n = i64::from_str_radix(&address_hash_hex.chars().nth(i).unwrap().to_string(), 16)
                .unwrap();
            let ch = address.chars().nth(i).unwrap();
            // make char uppercase if ith character is 9..f
            if n > 7 {
                checksum_address = format!("{}{}", checksum_address, ch.to_uppercase().to_string());
            } else {
                checksum_address = format!("{}{}", checksum_address, ch.to_string());
            }
        }

        return checksum_address;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pubkey_to_address() {
        let pubkey_string = "efb99d9860f4dec4cb548a5722c27e9ef58e37fbab9719c5b33d55c216db49311221a01f638ce5f255875b194e0acaa58b19a89d2e56a864427298f826a7f887";

        let address_derived =
            EthAddress::address_from_pubkey(hex::decode(pubkey_string).unwrap()).unwrap();
        println!("address is {}", address_derived);
        assert_eq!(
            address_derived,
            "c2d7cf95645d33006175b78989035c7c9061d3f9".to_string()
        );
    }

    #[test]
    fn test_pubkey_to_address_error() {
        let pubkey_string = "04efb99d9860f4dec4cb548a5722c27e9ef58e37fbab9719c5b33d55c216db49311221a01f638ce5f255875b194e0acaa58b19a89d2e56a864427298f826a7f887";

        let address_derived = EthAddress::address_from_pubkey(hex::decode(pubkey_string).unwrap());
        println!("testing length checking");
        assert_eq!(address_derived, Err(Error::PubKeyError));
    }

    #[test]
    fn test_checksummed_address() {}
}
