use crate::api::{
    AddressParam, AddressResult, BitcoinWallet, ExternalAddress, ExternalAddressParam,
};
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use bitcoin::Network;
use coin_btc_fork::address::BtcForkAddress;
use coin_btc_fork::btc_fork_network::network_from_param;
use prost::Message;

pub fn get_address(param: &AddressParam) -> Result<Vec<u8>> {
    let address: String;

    if param.is_seg_wit {
        let set_wit = "P2WPKH";
        let network = network_from_param(&param.chain_type, &param.network, &set_wit).unwrap();
        address = BtcForkAddress::p2shwpkh(&network, &param.path)?;
    } else {
        let set_wit = "NONE";
        let network = network_from_param(&param.chain_type, &param.network, &set_wit).unwrap();
        address = BtcForkAddress::p2pkh(&network, &param.path)?;
    }
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
    use crate::btc_fork_address::get_address;
    use device::device_binding::bind_test;

    #[test]
    fn test_btc_fork_address() {
        bind_test();
        let param = AddressParam {
            chain_type: "LITECOIN".to_string(),
            path: "m/44'/2'/0'/0/0".to_string(),
            network: "MAINNET".to_string(),
            is_seg_wit: false,
        };
        let message = get_address(&param);
        assert_eq!("0a0f6d2f3434272f32272f30272f302f3012084c495445434f494e1a224c64666465677833684a796744754644554137526b7a6a6a78386766466850394450", hex::encode(message.unwrap()));

        let param = AddressParam {
            chain_type: "LITECOIN".to_string(),
            path: "m/44'/2'/0'/0/0".to_string(),
            network: "MAINNET".to_string(),
            is_seg_wit: true,
        };
        let message = get_address(&param);
        assert_eq!("0a0f6d2f3434272f32272f30272f302f3012084c495445434f494e1a224d37786f314d693167554c5a5377677675375656457672774d52716e676d466b5664", hex::encode(message.unwrap()));
    }
}
