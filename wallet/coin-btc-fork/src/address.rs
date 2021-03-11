use crate::btc_fork_network::{network_form_hrp, network_from_coin, BtcForkNetwork};
use crate::common::get_xpub_data;
use crate::Result;
use bitcoin::hash_types::{PubkeyHash, ScriptHash};
use bitcoin::util::address::Payload;
use bitcoin::util::base58;
use bitcoin::{Address, Network, PublicKey};
use bitcoin_hashes::Hash;

use common::coin_info::coin_info_from_param;
use common::coin_info::CoinInfo;
use common::error::CoinError;
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

    pub fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        let ret = BtcForkAddress::from_str(address);
        if ret.is_err() {
            false
        } else {
            let addr: BtcForkAddress = ret.unwrap();
            addr.network.network == coin.network
        }
    }
}

impl FromStr for BtcForkAddress {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<BtcForkAddress> {
        // try bech32
        let bech32_network = bech32_network(s);
        if let Some(network) = bech32_network {
            // decode as bech32
            let (_, payload) = bech32::decode(s)?;
            if payload.is_empty() {
                return Err(CoinError::EmptyBech32Payload.into());
            }

            // Get the script version and program (converted from 5-bit to 8-bit)
            let (version, program): (bech32::u5, Vec<u8>) = {
                let (v, p5) = payload.split_at(1);
                (v[0], bech32::FromBase32::from_base32(p5)?)
            };

            // Generic segwit checks.
            if version.to_u8() > 16 {
                return Err(CoinError::InvalidWitnessVersion.into());
            }
            if program.len() < 2 || program.len() > 40 {
                return Err(CoinError::InvalidWitnessProgramLength.into());
            }

            // Specific segwit v0 check.
            if version.to_u8() == 0 && (program.len() != 20 && program.len() != 32) {
                return Err(CoinError::InvalidSegwitV0ProgramLength.into());
            }

            return Ok(BtcForkAddress {
                payload: Payload::WitnessProgram { version, program },
                network,
            });
        }

        let data = decode_base58(s)?;
        let (network, payload) = match data[0] {
            0 => {
                let coin_info = coin_info_from_param("BITCOIN", "MAINNET", "NONE", "")
                    .expect("BtcForkNetwork coin_info");
                (
                    network_from_coin(&coin_info).expect("btc"),
                    Payload::PubkeyHash(PubkeyHash::from_slice(&data[1..]).unwrap()),
                )
            }
            5 => {
                let coin_info = coin_info_from_param("BITCOIN", "MAINNET", "P2WPKH", "")
                    .expect("BITCOIN-P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("btc"),
                    Payload::ScriptHash(ScriptHash::from_slice(&data[1..]).unwrap()),
                    //Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            0x30 => {
                let coin_info = coin_info_from_param("LITECOIN", "MAINNET", "NONE", "")
                    .expect("LITECOIN coin_info");
                (
                    network_from_coin(&coin_info).expect("ltc-L"),
                    Payload::PubkeyHash(PubkeyHash::from_slice(&data[1..]).unwrap()),
                    //Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            0x32 => {
                let coin_info = coin_info_from_param("LITECOIN", "MAINNET", "P2WPKH", "")
                    .expect("LITECOIN-P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("ltc"),
                    Payload::ScriptHash(ScriptHash::from_slice(&data[1..]).unwrap()),
                    //Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            0x3a => {
                let coin_info = coin_info_from_param("LITECOIN", "TESTNET", "P2WPKH", "")
                    .expect("LITECOIN TESTNET P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("ltc-testnet"),
                    Payload::ScriptHash(ScriptHash::from_slice(&data[1..]).unwrap()),
                    //Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            111 => {
                let coin_info = coin_info_from_param("BITCOIN", "TESTNET", "NONE", "")
                    .expect("BITCOIN-TESTNET coin_info");
                (
                    network_from_coin(&coin_info).expect("btc-testnet"),
                    Payload::PubkeyHash(PubkeyHash::from_slice(&data[1..]).unwrap()),
                    //Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            196 => {
                let coin_info = coin_info_from_param("BITCOIN", "TESTNET", "P2WPKH", "")
                    .expect("BITCOIN-TESTNET-P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("btc-testnet"),
                    Payload::ScriptHash(ScriptHash::from_slice(&data[1..]).unwrap()),
                    //Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            x => {
                return Err(CoinError::InvalidVersion.into());
            }
        };

        Ok(BtcForkAddress { network, payload })
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

/// Extract the bech32 prefix.
/// Returns the same slice when no prefix is found.
fn bech32_network(bech32: &str) -> Option<BtcForkNetwork> {
    let bech32_prefix = match bech32.rfind('1') {
        None => None,
        Some(sep) => Some(bech32.split_at(sep).0),
    };
    match bech32_prefix {
        Some(prefix) => network_form_hrp(prefix),
        None => None,
    }
}

fn decode_base58(addr: &str) -> Result<Vec<u8>> {
    // Base58
    if addr.len() > 50 {
        return Err(CoinError::InvalidAddrLength.into());
    }
    let data = base58::from_check(&addr)?;
    if data.len() != 21 {
        return Err(CoinError::InvalidAddrLength.into());
    } else {
        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use crate::address::BtcForkAddress;
    use crate::btc_fork_network::network_from_param;
    use bitcoin::Network;
    use device::device_binding::bind_test;
    use std::str::FromStr;

    #[test]
    fn test_btc_fork_address() {
        bind_test();
        let network = network_from_param("LITECOIN", "MAINNET", "NONE").unwrap();
        let path: &str = "m/44'/2'/0'/0/0";
        let get_address_result = BtcForkAddress::p2pkh(&network, path);

        assert!(get_address_result.is_ok());
        let addr = get_address_result.ok().unwrap();
        assert_eq!("Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP", addr);

        let network = network_from_param("LITECOIN", "TESTNET", "NONE").unwrap();
        let path: &str = "m/44'/2'/0'/0/0";
        let get_address_result = BtcForkAddress::p2pkh(&network, path);

        assert!(get_address_result.is_ok());
        let addr = get_address_result.ok().unwrap();
        assert_eq!("myxdgXjCRgAskD2g1b6WJttJbuv67hq6sQ", addr);

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

    #[test]
    pub fn test_btc_fork_address_from_str() {
        let addr = BtcForkAddress::from_str("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW").unwrap();
        assert_eq!(addr.network.coin, "LITECOIN");
        assert_eq!(addr.network.seg_wit, "P2WPKH");
        assert_eq!(addr.network.network, "MAINNET");
        let addr = BtcForkAddress::from_str("ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf").unwrap();
        assert_eq!(addr.network.coin, "LITECOIN");
        assert_eq!(addr.network.seg_wit, "SEGWIT");
        assert_eq!(addr.network.network, "MAINNET");

        let addr = BtcForkAddress::from_str("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "P2WPKH");
        assert_eq!(addr.network.network, "MAINNET");
        let addr = BtcForkAddress::from_str("bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "SEGWIT");
        assert_eq!(addr.network.network, "MAINNET");
        let addr = BtcForkAddress::from_str("12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "NONE");
        assert_eq!(addr.network.network, "MAINNET");

        let addr = BtcForkAddress::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "P2WPKH");
        assert_eq!(addr.network.network, "TESTNET");
    }
}
