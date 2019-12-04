use hex;
use keccak_hash::keccak;
use regex::Regex;

#[derive(Debug)]
pub struct EthAddress {}

impl EthAddress {
    pub fn address_from_pubkey(pubkey: Vec<u8>) -> String {
        let pubkey_hash = keccak(pubkey);
        let addr_bytes = &pubkey_hash[12..];
        hex::encode(addr_bytes) //@@XM TODO: check this encode
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
