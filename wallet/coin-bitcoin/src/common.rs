use crate::transaction::Utxo;
//use secp256k1::{Secp256k1, Message, Signature, SecretKey};
use bitcoin::util::bip32::{ExtendedPubKey, ChainCode, ChildNumber};
use bitcoin::{Address, PublicKey, Network, TxOut, Transaction, TxIn, OutPoint, Script, SigHashType};
use bitcoin::secp256k1::Secp256k1;
use std::str::FromStr;
use crate::error::BtcError;

pub fn address_verify(utxos : &Vec<Utxo>, public_key : &str, chain_code : &[u8], network : Network, flg : &str) -> Result<Vec<String>, BtcError>{
    let mut utxo_pub_key_vec: Vec<String> = Vec::new();
    for utxo in utxos {
        //4.get utxo public key
        let mut public_key_obj = PublicKey::from_str(public_key).unwrap();
        public_key_obj.compressed = true;

//        let temp_chain_code_vec = Vec::from_hex(chain_code).unwrap();
        let chain_code_obj = ChainCode::from(chain_code);
        //build extended public key
        let mut extend_public_key = ExtendedPubKey {
            network: network,
            depth: 0,
            parent_fingerprint: Default::default(),
            child_number: ChildNumber::from_normal_idx(0).unwrap(),
            public_key: public_key_obj,
            chain_code: chain_code_obj,
        };

        let bitcoin_secp = Secp256k1::new();
        let index_number_vec: Vec<&str> = utxo.derive_path.as_str().split('/').collect();
        for index_number in index_number_vec {
            let test_chain_number =
                ChildNumber::from_normal_idx(index_number.parse().unwrap()).unwrap();
            extend_public_key = extend_public_key.ckd_pub(&bitcoin_secp, test_chain_number).unwrap();
        }
        //verify address
//        let se_gen_address = Address::p2pkh(
//            &PublicKey::from_str(extend_public_key.public_key.to_string().as_str()).unwrap(),
//            network,
//        ).to_string();

        let mut se_gen_address = String::new();
        if flg.eq("btc") {
            se_gen_address = Address::p2pkh(
                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str()).unwrap(),
                network,
            ).to_string();
        }else {
            se_gen_address = Address::p2shwpkh(
                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str()).unwrap(),
                network,
            ).to_string();
        }

        let utxo_address = utxo.address.to_string();

        if !se_gen_address.eq(&utxo_address) {
            return Err(BtcError::ImkeyAddressMismatchWithPath);
        }
        utxo_pub_key_vec.push(extend_public_key.public_key.to_string());

    }
    Ok(utxo_pub_key_vec)
}