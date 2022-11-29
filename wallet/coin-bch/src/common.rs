use crate::address::BchAddress;
use crate::transaction::Utxo;
use crate::Result;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1 as BitcoinSecp256k1;
use bitcoin::util::base58;
use bitcoin::util::bip32::{ChainCode, ChildNumber, ExtendedPubKey};
use bitcoin::{Address, PublicKey};
use common::apdu::{ApduCheck, BtcForkApdu, CoinCommonApdu};
use common::error::CoinError;
use common::utility::sha256_hash;
use device::device_binding::KEY_MANAGER;
use secp256k1::{Message, PublicKey as Secp256k1PublicKey, Secp256k1, Signature};
use std::convert::TryFrom;
use std::str::FromStr;
use transport::message::send_apdu;

/**
utxo address verify
*/
pub fn address_verify(
    utxos: &Vec<Utxo>,
    public_key: &str,
    chain_code: &[u8],
    network: Network,
) -> Result<Vec<String>> {
    let mut utxo_pub_key_vec: Vec<String> = vec![];
    for utxo in utxos {
        //get utxo public key
        let secp256k1_pubkey = Secp256k1PublicKey::from_str(public_key)?;
        let public_key_obj = PublicKey {
            compressed: true,
            inner: secp256k1_pubkey,
        };
        //gen chain code obj
        let chain_code_obj = ChainCode::try_from(chain_code)?;
        //build extended public key
        let mut extend_public_key = ExtendedPubKey {
            network: network,
            depth: 0,
            parent_fingerprint: Default::default(),
            child_number: ChildNumber::from_normal_idx(0)?,
            public_key: secp256k1_pubkey,
            chain_code: chain_code_obj,
        };

        let bitcoin_secp = BitcoinSecp256k1::new();

        let se_gen_address_str = if utxo.derive_path.is_empty() {
            Address::p2pkh(&public_key_obj, network).to_string()
        } else {
            let index_number_vec: Vec<&str> = utxo.derive_path.as_str().split('/').collect();

            for index_number in index_number_vec {
                let test_chain_number =
                    ChildNumber::from_normal_idx(index_number.parse().unwrap())?;
                extend_public_key = extend_public_key.ckd_pub(&bitcoin_secp, test_chain_number)?;
            }
            Address::p2pkh(
                &PublicKey::from_str(extend_public_key.public_key.to_string().as_str())?,
                network,
            )
            .to_string()
        };
        // //verify address
        // let se_gen_address_str = Address::p2pkh(
        //     &PublicKey::from_str(extend_public_key.public_key.to_string().as_str())?,
        //     network,
        // )
        // .to_string();

        let utxo_address = utxo.address.clone();
        let bch_address = BchAddress::convert_to_legacy_if_need(utxo_address.as_ref())?;

        if !bch_address.eq(&se_gen_address_str) {
            return Err(CoinError::ImkeyAddressMismatchWithPath.into());
        }
        utxo_pub_key_vec.push(extend_public_key.public_key.to_string());
    }
    Ok(utxo_pub_key_vec)
}

/**
Transaction type identification
*/
pub enum TransTypeFlg {
    BTC,
    SEGWIT,
}

/**
get xpub
*/
pub fn get_xpub_data(path: &str, verify_flag: bool) -> Result<String> {
    let select_response = send_apdu(BtcForkApdu::select_applet())?;
    ApduCheck::check_response(&select_response)?;
    let xpub_data = send_apdu(BtcForkApdu::get_xpub(path, verify_flag))?;
    ApduCheck::check_response(&xpub_data)?;
    Ok(xpub_data)
}

/**
sign verify
*/
pub fn apdu_sign_verify(signed: &[u8], message: &[u8]) -> Result<bool> {
    let key_manager_obj = KEY_MANAGER.lock();
    let public = &key_manager_obj.se_pub_key.as_slice();
    let secp = Secp256k1::new();
    //build public
    let public_obj = Secp256k1PublicKey::from_slice(public)?;
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
pub fn get_address_version(network: Network, address: &str) -> Result<u8> {
    let legacy_address = BchAddress::convert_to_legacy_if_need(address)?;

    match network {
        Network::Bitcoin => {
            if !legacy_address.starts_with('1') && !legacy_address.starts_with('3') {
                return Err(CoinError::AddressTypeMismatch.into());
            }
        }
        Network::Testnet => {
            if !legacy_address.starts_with('m')
                && !legacy_address.starts_with('n')
                && !legacy_address.starts_with('2')
            {
                return Err(CoinError::AddressTypeMismatch.into());
            }
        }
        _ => {
            return Err(CoinError::ImkeySdkIllegalArgument.into());
        }
    }
    //get address version
    let address_bytes = base58::from(legacy_address.as_ref())?;
    Ok(address_bytes.as_slice()[0])
}

pub struct TxSignResult {
    pub signature: String,
    pub tx_hash: String,
    pub wtx_id: String,
}

#[cfg(test)]
mod test {
    use crate::common::get_address_version;
    use bitcoin::Network;

    #[test]
    fn get_address_version_test() {
        let address_version =
            get_address_version(Network::Bitcoin, "3CVD68V71no5jn2UZpLLq6hASpXu1jrByt");
        assert!(address_version.is_ok());
        assert_eq!(5, address_version.ok().unwrap());

        let address_version =
            get_address_version(Network::Bitcoin, "2CVD68V71no5jn2UZpLLq6hASpXu1jrByt");
        assert_eq!(
            format!("{}", address_version.err().unwrap()),
            "address_type_mismatch"
        );

        let address_version =
            get_address_version(Network::Testnet, "3CVD68V71no5jn2UZpLLq6hASpXu1jrByt");
        assert_eq!(
            format!("{}", address_version.err().unwrap()),
            "address_type_mismatch"
        );

        let address_version =
            get_address_version(Network::Regtest, "3CVD68V71no5jn2UZpLLq6hASpXu1jrByt");
        assert_eq!(
            format!("{}", address_version.err().unwrap()),
            "imkey_sdk_illegal_argument"
        );
    }
}
