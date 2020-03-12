use ring::digest;
use secp256k1::{Secp256k1, Message, Signature, PublicKey as PublicKey2, SecretKey};
use num_bigint::BigInt;
use num_traits::{Num, FromPrimitive, Zero};
use num_integer::Integer;
use secp256k1::recovery::{RecoverableSignature, RecoveryId};
//use secp256k1::{self, Message as SecpMessage};
use crate::error::CommonError;
use crate::Result;


pub fn hex_to_bytes(value: &str) -> Result<Vec<u8>> {
    let mut ret_data = vec![];
    if value.to_lowercase().starts_with("0x") {
        ret_data = hex::decode(&value[2..value.len()])?
    } else {
        ret_data = hex::decode(value)?
    }
    Ok(ret_data)
}

pub fn sha256_hash(data : &[u8]) -> Vec<u8>{

    let digest_obj = digest::digest(
        &digest::SHA256,
        data,
    );
    digest_obj.as_ref().to_vec()
}

pub fn secp256k1_sign(private_key : &[u8], message : &[u8]) -> Result<Vec<u8>>{
    //calc twice sha256 hash
    let message_hash = sha256_hash(sha256_hash(message).as_ref());
    //generator SecretKey obj
    let secret_key = SecretKey::from_slice(private_key)?;
    //generator Message obj
    let message_data = Message::from_slice(message_hash.as_ref())?;
    let secp = Secp256k1::new();
    //sign data
    Ok(secp.sign(&message_data, &secret_key).serialize_der().to_vec())
}

pub fn secp256k1_sign_hash(private_key : &[u8], hash : &[u8]) -> Result<Vec<u8>>{
    //generator SecretKey obj
    let secret_key = SecretKey::from_slice(private_key)?;
    //generator Message obj
    let message_data = Message::from_slice(hash.as_ref())?;
    let secp = Secp256k1::new();
    //sign data
    Ok(secp.sign(&message_data, &secret_key).serialize_der().to_vec())
}

/**
sign verify
*/
pub fn secp256k1_sign_verify(public : &[u8], signed : &[u8], message : &[u8]) -> Result<bool>{

    let secp = Secp256k1::new();
    //build public
    let public_obj = PublicKey2::from_slice(public)?;
    //build message
    let hash_result = sha256_hash(message);
    let message_obj = Message::from_slice(hash_result.as_ref())?;
    //build signature obj
    let mut sig_obj = Signature::from_der(signed)?;
    sig_obj.normalize_s();
    //verify
    Ok(secp.verify(&message_obj, &sig_obj, &public_obj).is_ok())

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

pub fn retrieve_recid(
    msg: &[u8],
    sign_compact: &[u8],
    pubkey: &Vec<u8>,
) -> Result<RecoveryId> {
    let secp_context = secp256k1::Secp256k1::new();

    let mut recid_final = -1i32;
    for i in 0..3 {
        let rec_id = RecoveryId::from_i32(i as i32)?;
        let sig = RecoverableSignature::from_compact(sign_compact, rec_id)?;
        let msg_to_sign = Message::from_slice(msg)?;

        if let Ok(rec_pubkey) = secp_context.recover(&msg_to_sign, &sig) {
            let rec_pubkey_raw = rec_pubkey.serialize_uncompressed();
            let rec_pubkey = hex::encode(rec_pubkey_raw.iter());
            let pub_key = hex::encode(pubkey);
            if rec_pubkey_raw.to_vec() == *pubkey {
                recid_final = i;
                break;
            }
        } else {
            continue;
        }
    }

    let rec_id = RecoveryId::from_i32(recid_final)?;
    Ok(rec_id)
}