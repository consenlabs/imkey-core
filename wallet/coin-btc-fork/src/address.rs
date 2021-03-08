use crate::btc_fork_network::BtcForkNetwork;
use crate::common::get_xpub_data;
use crate::Result;
use bitcoin::util::address::Payload;
use bitcoin::util::base58;
use bitcoin::{Address, Network, PublicKey};
use common::path::check_path_validity;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkAddress {
    pub network: BtcForkNetwork,
    pub payload: Payload,
}

impl BtcForkAddress {
    pub fn p2pkh(network: &BtcForkNetwork, path: &str) -> Result<String> {
        //path check
        check_path_validity(path)?;

        //get xpub
        let xpub_data = get_xpub_data(path, true)?;
        let pub_key = &xpub_data[..130];

        let mut pub_key_obj = PublicKey::from_str(pub_key)?;
        pub_key_obj.compressed = true;
        //let s = Address::p2pkh(&pub_key_obj, Network::Bitcoin);
        let addr = Address::p2pkh(&pub_key_obj, Network::Bitcoin);
        let btc_fork_address = BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        };

        Ok(btc_fork_address.to_string())
    }

    pub fn p2shwpkh(network: &BtcForkNetwork, path: &str) -> Result<String> {
        //path check
        check_path_validity(path)?;

        //get xpub
        let xpub_data = get_xpub_data(path, true)?;
        let pub_key = &xpub_data[..130];

        let mut pub_key_obj = PublicKey::from_str(pub_key)?;
        pub_key_obj.compressed = true;
        //let s = Address::p2pkh(&pub_key_obj, Network::Bitcoin);
        let addr = Address::p2shwpkh(&pub_key_obj, Network::Bitcoin).unwrap();
        let btc_fork_address = BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        };

        Ok(btc_fork_address.to_string())
    }

    pub fn p2wpkh(network: &BtcForkNetwork, path: &str) -> Result<String> {
        //path check
        check_path_validity(path)?;

        //get xpub
        let xpub_data = get_xpub_data(path, true)?;
        let pub_key = &xpub_data[..130];

        let mut pub_key_obj = PublicKey::from_str(pub_key)?;
        pub_key_obj.compressed = true;
        let addr = Address::p2wpkh(&pub_key_obj, Network::Bitcoin).unwrap();
        let btc_fork_address = BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        };

        Ok(btc_fork_address.to_string())
    }
}

impl Display for BtcForkAddress {
    fn fmt(&self, fmt: &mut Formatter) -> core::fmt::Result {
        match self.payload {
            Payload::PubkeyHash(ref hash) => {
                let mut prefixed = [0; 21];
                prefixed[0] = self.network.p2pkh_prefix;
                prefixed[1..].copy_from_slice(&hash[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            }
            Payload::ScriptHash(ref hash) => {
                let mut prefixed = [0; 21];
                prefixed[0] = self.network.p2sh_prefix;
                prefixed[1..].copy_from_slice(&hash[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            }
            Payload::WitnessProgram {
                version: ver,
                program: ref prog,
            } => {
                let hrp = self.network.hrp;
                let mut bech32_writer = bech32::Bech32Writer::new(hrp, fmt)?;
                bech32::WriteBase32::write_u5(&mut bech32_writer, ver)?;
                bech32::ToBase32::write_base32(&prog, &mut bech32_writer)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::address::BtcForkAddress;
    use crate::btc_fork_network::network_from_param;
    use bitcoin::Network;
    use device::device_binding::bind_test;

    #[test]
    fn test_btc_fork_address() {
        bind_test();
        let network = network_from_param("LITECOIN", "MAINNET", "NONE").unwrap();
        let path: &str = "m/44'/2'/0'/0/0";
        let get_address_result = BtcForkAddress::p2pkh(&network, path);

        assert!(get_address_result.is_ok());
        let addr = get_address_result.ok().unwrap();
        assert_eq!("Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP", addr);

        let network = network_from_param("LITECOIN", "MAINNET", "P2WPKH").unwrap();
        let path: &str = "m/44'/2'/0'/0/0";
        let get_address_result = BtcForkAddress::p2shwpkh(&network, path);

        assert!(get_address_result.is_ok());
        let addr = get_address_result.ok().unwrap();
        assert_eq!("M7xo1Mi1gULZSwgvu7VVEvrwMRqngmFkVd", addr);

        let network = network_from_param("LITECOIN", "MAINNET", "SEGWIT").unwrap();
        let path: &str = "m/44'/2'/0'/0/0";
        let get_address_result = BtcForkAddress::p2wpkh(&network, path);

        assert!(get_address_result.is_ok());
        let addr = get_address_result.ok().unwrap();
        assert_eq!("ltc1qefxc4n0dd88y7pwsjfv5d5nplpkxwh7cl75fny", addr);
    }
}
