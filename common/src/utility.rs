use hex::FromHexError;
use std::result::Result;

pub fn hex_to_bytes(value: &str) -> Result<Vec<u8>, FromHexError> {
    if value.to_lowercase().starts_with("0x") {
        let len = value.len();
        hex::decode(&value[2..len])
    } else {
        hex::decode(value)
    }
}
