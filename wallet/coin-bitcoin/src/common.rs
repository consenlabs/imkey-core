use crate::transaction::Utxo;
//use secp256k1::{Secp256k1, Message, Signature, SecretKey};
use bitcoin::util::bip32::{ExtendedPubKey, ChainCode, ChildNumber};
use bitcoin::{Address, PublicKey, Network, TxOut, Transaction, TxIn, OutPoint, Script, SigHashType};
use bitcoin::secp256k1::Secp256k1 as BitcoinSecp256k1;
use std::str::FromStr;
use crate::error::BtcError;
use mq::message::send_apdu;
use common::apdu::{BtcApdu, ApduCheck};
use secp256k1::{Secp256k1, Message, Signature, PublicKey as PublicKey2, SecretKey, Error};
use common::utility::{sha256_hash};
use bitcoin::util::base58;
use crate::Result;


/**
utxo address verify
*/
pub fn address_verify(utxos : &Vec<Utxo>, public_key : &str, chain_code : &[u8], network : Network, flg : &str) -> Result<Vec<String>>{
    let mut utxo_pub_key_vec: Vec<String> = vec![];
    for utxo in utxos {
        //get utxo public key
        let mut public_key_obj = PublicKey::from_str(public_key)?;
        public_key_obj.compressed = true;
        //gen chain code obj
        let chain_code_obj = ChainCode::from(chain_code);
        //build extended public key
        let mut extend_public_key = ExtendedPubKey {
            network: network,
            depth: 0,
            parent_fingerprint: Default::default(),
            child_number: ChildNumber::from_normal_idx(0)?,
            public_key: public_key_obj,
            chain_code: chain_code_obj,
        };

        let bitcoin_secp = BitcoinSecp256k1::new();
        let index_number_vec: Vec<&str> = utxo.derive_path.as_str().split('/').collect();
        for index_number in index_number_vec {
            let test_chain_number =
                ChildNumber::from_normal_idx(index_number.parse().unwrap())?;
            extend_public_key = extend_public_key.ckd_pub(&bitcoin_secp, test_chain_number)?;
        }
        //verify address
//        let mut se_gen_address = String::new();
//        if flg.eq("btc") {
//            se_gen_address = Address::p2pkh(
//                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str())?,
//                network,
//            ).to_string();
//        }else {
//            se_gen_address = Address::p2shwpkh(
//                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str())?,
//                network,
//            ).to_string();
//        }
        let se_gen_address: Result<String> = match flg {
            "btc" => Ok(Address::p2pkh(
                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str())?,
                network).to_string()),
            "segwit" => Ok(Address::p2shwpkh(
                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str())?,
                network).to_string()),
            _ => return Err(BtcError::IMKEY_ADDRESS_MISMATCH_WITH_PATH.into()),//TODO 返回错误信息不对
        };
        let se_gen_address_str = se_gen_address?;
        let utxo_address = utxo.address.to_string();
        if !se_gen_address_str.eq(&utxo_address) {
            return Err(BtcError::IMKEY_ADDRESS_MISMATCH_WITH_PATH.into());
        }
        utxo_pub_key_vec.push(extend_public_key.public_key.to_string());
    }
    Ok(utxo_pub_key_vec)
}

/**
get xpub
*/
pub fn get_xpub_data(path: &str, verify_flag: bool) -> Result<String>{
    let select_response = send_apdu(BtcApdu::select_applet());
    ApduCheck::checke_response(&select_response)?;
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, verify_flag));
    ApduCheck::checke_response(&xpub_data)?;
    Ok(xpub_data)
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

/**
get address version
*/
pub fn get_address_version(network: Network, address: &str) -> Result<u8>{
    //check address
//    if network == Network::Bitcoin{
//        if !address.starts_with('1') && !address.starts_with('3') {
//            return Err(BtcError::AddressTypeMismatch);
//        }
//    }else if network == Network::Testnet {
//        if !address.starts_with('m') &&
//            !address.starts_with('n') &&
//            !address.starts_with('2') {
//            return Err(BtcError::AddressTypeMismatch);
//        }
//    }else {
//        //TODO
//    }
    match network {
        Network::Bitcoin => {
            if !address.starts_with('1') && !address.starts_with('3') {
                return Err(BtcError::ADDRESS_TYPE_MISMATCH.into());
            }
        },
        Network::Testnet => {
            if !address.starts_with('m') &&
                !address.starts_with('n') &&
                !address.starts_with('2') {
                return Err(BtcError::ADDRESS_TYPE_MISMATCH.into());
            }
        },
        _ => {
            return Err(BtcError::IMKEY_SDK_ILLEGAL_ARGUMENT.into());
        },
    }
    //get address version
    let address_bytes = base58::from(address)?;
    Ok(address_bytes.as_slice()[0])
}

pub struct TxSignResult {
    pub signature: String,
    pub tx_hash: String,
    pub wtx_id: String,
}

#[cfg(test)]
mod test{
    use crate::common::get_address_version;
    use bitcoin::Network;

    #[test]
    fn get_address_version_test(){
        let address_version = get_address_version(Network::Bitcoin, "3CVD68V71no5jn2UZpLLq6hASpXu1jrByt");
        if address_version.is_ok() {
            println!("address version is : {}", address_version.ok().unwrap());
        }else {
            println!("get address version error");
        }

    }
}
