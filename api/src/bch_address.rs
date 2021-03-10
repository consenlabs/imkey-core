use crate::api::{AddressParam, AddressResult};
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::Network;
use coin_bch::address::BchAddress;

pub fn get_address(param: &AddressParam) -> Result<Vec<u8>> {
    let network = match param.network.as_ref() {
        "MAINNET" => Network::Bitcoin,
        "TESTNET" => Network::Testnet,
        _ => Network::Testnet,
    };

    let address = BchAddress::get_address(network, &param.path)?;
    let address_message = AddressResult {
        path: param.path.to_owned(),
        chain_type: param.chain_type.to_string(),
        address: address,
    };
    encode_message(address_message)
}

#[cfg(test)]
mod tests {
    use crate::api::{
        AddressParam, AddressResult, BitcoinWallet, ExternalAddress, ExternalAddressParam,
    };
    use crate::bch_address::get_address;
    use device::device_binding::bind_test;

    #[test]
    fn test_btc_fork_address() {
        bind_test();

        let param = AddressParam {
            chain_type: "BITCOINCASH".to_string(),
            path: "m/44'/145'/0'/0/0".to_string(),
            network: "MAINNET".to_string(),
            is_seg_wit: true,
        };
        let message = get_address(&param);
        assert_eq!("0a116d2f3434272f313435272f30272f302f30120b424954434f494e434153481a2a717a6c643764617637643273666a646c367839736e6b76663672616a386c66786a636a35666138793272", hex::encode(message.unwrap()));

        let param = AddressParam {
            chain_type: "BITCOINCASH".to_string(),
            path: "m/44'/145'/0'/0/0".to_string(),
            network: "TESTNET".to_string(),
            is_seg_wit: true,
        };
        let message = get_address(&param);
        assert_eq!("0a116d2f3434272f313435272f30272f302f30120b424954434f494e434153481a2a717a6c643764617637643273666a646c367839736e6b76663672616a386c66786a636b786436396e646c", hex::encode(message.unwrap()));
    }
}
