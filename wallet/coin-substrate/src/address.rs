use crate::common::constants;
use crate::Result;
use common::apdu::{ApduCheck, BtcApdu, CoinCommonApdu};
use secp256k1::PublicKey;
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::sr25519::Public;
use sp_core::Public as TraitPublic;
use std::str::FromStr;
use transport::message::send_apdu;

pub struct PolkadotAddress();
impl PolkadotAddress {
    /**
    get address
    */
    pub fn get_address(address_type: AddressType) -> Result<String> {
        //get public key form imkey
        //TODO

        let pk_bytes: Vec<u8> =
            hex::decode("1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap(); //TODO

        let public_obj = Public::from_slice(pk_bytes.as_slice());
        let address = match address_type {
            AddressType::Polkadot => {
                public_obj.to_ss58check_with_version(Ss58AddressFormat::PolkadotAccount)
            }
            AddressType::Kusama => {
                public_obj.to_ss58check_with_version(Ss58AddressFormat::KusamaAccount)
            }
            _ => panic!("address type support"),
        };

        Ok(address)
    }

    /**
    get public key
    */
    pub fn get_pubkey() -> Result<String> {
        Ok(String::new())
    }

    pub fn display_address(address_type: AddressType) -> Result<String> {
        Ok(String::new())
    }
}

/**
get xpub
*/
pub fn get_pub_key(path: &str, verify_flag: bool) -> Result<String> {
    let select_response = send_apdu(BtcApdu::select_applet())?;
    ApduCheck::checke_response(&select_response)?;
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, verify_flag))?;
    ApduCheck::checke_response(&xpub_data)?;
    Ok(xpub_data[..130].to_string())
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum AddressType {
    Polkadot,
    Kusama,
}

#[cfg(test)]
mod test {
    use schnorrkel::{ExpansionMode, MiniSecretKey};
    use sp_core::crypto::Ss58AddressFormat;
    use sp_core::crypto::Ss58Codec;
    use sp_core::sr25519::{Pair, Public};
    use sp_core::Pair as TraitPair;

    #[test]
    fn key_test() {
        let mut seed = Pair::generate().1.to_vec();
        seed = hex::decode("1111111111111111111111111111111111111111111111111111111111111111")
            .unwrap();
        println!("{}", hex::encode_upper(seed.clone()));
        let mini_key: MiniSecretKey = MiniSecretKey::from_bytes(seed.as_slice())
            .expect("32 bytes can always build a key; qed");

        let kp = mini_key.expand_to_keypair(ExpansionMode::Ed25519);
        let gen_pair = Pair::from(kp);
        let polakdot_address = gen_pair
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::PolkadotAccount);
        println!("polakdot_address : {}", polakdot_address);
        let polakdot_address = gen_pair
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::KusamaAccount);
        println!("kusama_address : {}", polakdot_address);
        let polakdot_address = gen_pair
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::SubstrateAccount);
        println!("substrate_address : {}", polakdot_address);
        let temp_pub_key = gen_pair.public().0;
        let temp_private = gen_pair.to_raw_vec();
        println!("public key: {}", hex::encode_upper(temp_pub_key));
        println!("private key: {}", hex::encode_upper(temp_private));
    }
}
