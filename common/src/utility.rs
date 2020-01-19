use hex::FromHexError;
use std::result::Result;
use ring::digest;
use secp256k1::{Secp256k1, Message, Signature, PublicKey as PublicKey2, SecretKey};
use num_bigint::BigInt;
use num_traits::{Num, FromPrimitive, Zero};
use num_integer::Integer;

pub fn hex_to_bytes(value: &str) -> Result<Vec<u8>, FromHexError> {
    if value.to_lowercase().starts_with("0x") {
        let len = value.len();
        hex::decode(&value[2..len])
    } else {
        hex::decode(value)
    }
}

pub fn sha256_hash(data : &[u8]) -> Vec<u8>{

    let digest_obj = digest::digest(
        &digest::SHA256,
        data,
    );
    digest_obj.as_ref().to_vec()
}

pub fn secp256k1_sign_verify(public : &[u8], signed : &[u8], message : &[u8]) -> bool{

    let secp = Secp256k1::new();
    //build public
    let public_obj = PublicKey2::from_slice(public).expect("public error");
    //build message
    let hash_result = sha256_hash(message);
    let message_obj = Message::from_slice(hash_result.as_ref()).expect("build message obj error");
    //build signature obj
    let mut sig_obj = Signature::from_der(signed).expect("build signature obj error");
    sig_obj.normalize_s();
    //verify
    secp.verify(&message_obj, &sig_obj, &public_obj).is_ok()

}

pub fn secp256k1_sign(private_key : &[u8], message : &[u8]) -> Vec<u8>{
    //calc twice sha256 hash
    let message_hash = sha256_hash(sha256_hash(message).as_ref());
    //generator SecretKey obj
    let secret_key = SecretKey::from_slice(private_key).expect("private error");
    //generator Message obj
    let message_data = Message::from_slice(message_hash.as_ref()).expect("build message obj error");
    let secp = Secp256k1::new();
    //sign data
    secp.sign(&message_data, &secret_key).serialize_der().to_vec()
}

pub fn bigint_to_byte_vec(val : i64) -> Vec<u8>{
    let mut return_data = BigInt::from(val).to_signed_bytes_be();
    while return_data.len() < 8 {
        return_data.insert(0, 0x00);
    }
    return_data
}

pub fn uncompress_pubkey_2_compress(uncomprs_pubkey: &str) -> String {
    let x = &uncomprs_pubkey[2..66];
    let y = &uncomprs_pubkey[66..130];
    let y_bint = BigInt::from_str_radix(&y,16).unwrap();
    let two_bint = BigInt::from_i64(2).unwrap();

    let (_d, m) = y_bint.div_mod_floor(&two_bint);
    return if m.is_zero() {
        "02".to_owned() + x
    } else {
        "03".to_owned() + x
    }
}