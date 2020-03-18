use bitcoin::{Address, PublicKey, Network};
use std::str::FromStr;
use bitcoin::util::bip32::{ExtendedPubKey, ChainCode, ChildNumber, DerivationPath, Fingerprint};
use hex::decode;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::hashes::core::convert::TryFrom;
use bitcoin::hashes::{hash160, Hash};
use common::path::check_path_validity;
use crate::common::get_xpub_data;
use crate::Result;
use common::apdu::{BtcApdu, ApduCheck};
use mq::message::send_apdu;

/**
get btc xpub by path
*/
pub fn get_xpub(network : Network, path : &str) -> Result<String>{

    //path check
    check_path_validity(path)?;

    //get xpub data
    let xpub_data = get_xpub_data(path, true)?;
    let xpub_data = &xpub_data[..194].to_string();

    //get public key and chain code
    let pub_key = &xpub_data[..130];
    let chain_code = &xpub_data[130..];

    //build parent public key obj
    let parent_xpub = get_xpub_data(get_parent_path(path), true)?;
    let parent_xpub = &parent_xpub[..130].to_string();
    let mut parent_pub_key_obj = PublicKey::from_str(parent_xpub)?;
    parent_pub_key_obj.compressed = true;

    //build child public key obj
    let mut pub_key_obj = PublicKey::from_str(pub_key)?;
    pub_key_obj.compressed = true;

    //get parent public key fingerprint
    let chain_code_obj = ChainCode::from(hex::decode(chain_code).unwrap().as_slice());
    let parent_ext_pub_key = ExtendedPubKey {
        network: network,
        depth: 0 as u8,
        parent_fingerprint: Fingerprint::default(),
        child_number: ChildNumber::from_normal_idx(0).unwrap(),
        public_key: parent_pub_key_obj,
        chain_code: chain_code_obj,
    };
    let fingerprint_obj = parent_ext_pub_key.fingerprint();

    //build extend public key obj
    let chain_code_obj = ChainCode::from(hex::decode(chain_code).unwrap().as_slice());
    let chain_number_vec: Vec<ChildNumber> = DerivationPath::from_str(path)?.into();
    let mut extend_public_key = ExtendedPubKey {
        network: network,
        depth: chain_number_vec.len() as u8,
        parent_fingerprint: fingerprint_obj,
        child_number: *chain_number_vec.get(chain_number_vec.len() - 1).unwrap(),
        public_key: pub_key_obj,
        chain_code: chain_code_obj,
    };
    //get and return xpub
    Ok(extend_public_key.to_string())

}

/**
get btc address by path
*/
pub fn get_address(network : Network, path : &str) -> Result<String>{
    //path check
    check_path_validity(path)?;

    //get xpub
    let xpub_data = get_xpub_data(path, true)?;
    let pub_key = &xpub_data[..130];

    let mut pub_key_obj = PublicKey::from_str(pub_key)?;
    pub_key_obj.compressed = true;

    Ok(Address::p2pkh(&pub_key_obj, network).to_string())
}

/**
get segwit address by path
*/
pub fn get_segwit_address(network : Network, path : &str) -> Result<String>{
    //path check
    check_path_validity(path)?;

    //get xpub
    let xpub_data = get_xpub_data(path, true)?;
    let pub_key = &xpub_data[..130];

    let mut pub_key_obj = PublicKey::from_str(pub_key)?;
    pub_key_obj.compressed = true;

    Ok(Address::p2shwpkh(&pub_key_obj, network).to_string())
}

/**
get parent public key path
*/
pub fn get_parent_path(path: &str) -> &str{
    if path.ends_with("/") {
        return &path[..path.len() - 1 ];
    }
    let end_flg = path.rfind("/").unwrap();
    &path[..end_flg]
}

pub fn display_Address(network: Network, path: &str) -> Result<String>{
    //path check
    check_path_validity(path)?;
    let address_str =  get_address(network, path)?;
    let btc_coin_reg = BtcApdu::btc_coin_reg(address_str.clone().into_bytes());
    let apdu_res = send_apdu(BtcApdu::btc_coin_reg(address_str.clone().into_bytes()));
    ApduCheck::checke_response(apdu_res.as_str())?;
    Ok(address_str)
}

pub fn display_SegWit_Address(network: Network, path: &str) -> Result<String>{
    //path check
    check_path_validity(path)?;
    let address_str =  get_segwit_address(network, path)?;
    let btc_coin_reg = BtcApdu::btc_coin_reg(address_str.clone().into_bytes());
    let apdu_res = send_apdu(BtcApdu::btc_coin_reg(address_str.clone().into_bytes()));
    ApduCheck::checke_response(apdu_res.as_str())?;
    Ok(address_str)
}


#[cfg(test)]
mod test{
    use crate::btc::{get_xpub, get_address, get_segwit_address, get_parent_path};
    use bitcoin::Network;
    use device::device_binding::DeviceManage;

    #[test]
    fn get_xpub_test(){
        //device binding
        device_binding_test();

        let version : Network = Network::Bitcoin;
        let path : &str = "m/44'/0'/0'/0/0";
        let get_xpub_result = get_xpub(version, path);
        if get_xpub_result.is_ok() {
            let xpub = get_xpub_result.ok().unwrap();
            println!("xpub : {:?}", xpub);
            assert_eq!("xpub6FuzpGNBc46EfvmcvECyqXjrzGcKErQgpQcpvhw1tiC5yXvi1jUkzudMpdg5AaguiFstdVR5ASDbSceBswKRy6cAhpTgozmgxMUayPDrLLX", xpub);
        }else {
            panic!("get xpub error");
        }

    }

    #[test]
    fn get_address_test(){
        //device binding
        device_binding_test();

        let version : Network = Network::Bitcoin;
        let path : &str = "m/44'/0'/0'/0/0";
        let get_btc_address_result = get_address(version, path);
        if get_btc_address_result.is_ok() {
            let btc_address = get_btc_address_result.ok().unwrap();
            println!("btc address : {:?}", btc_address);
            assert_eq!("12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g", btc_address);
        }else {
            panic!("get btc address error");
        }

    }

    #[test]
    fn get_segwit_address_test(){
        //device binding
        device_binding_test();

        let version : Network = Network::Bitcoin;
        let path : &str = "m/49'/0'/0'/0/22";
        let segwit_address_result = get_segwit_address(version, path);
        if segwit_address_result.is_ok() {
            let segwit_address = segwit_address_result.ok().unwrap();
            println!("segwit address : {:?}", segwit_address);
            assert_eq!("37E2J9ViM4QFiewo7aw5L3drF2QKB99F9e", segwit_address);
        }else {
            panic!("get segwit address error");
        }
    }

    #[test]
    fn device_binding_test(){
        //设备绑定
        // let path = "/Users/caixiaoguang/workspace/myproject/imkey-core/".to_string();
        // let bind_code = "E4APZZRT".to_string();

        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let bind_code = "YDSGQPKX".to_string();
        // let mut device_manage = DeviceManage::new();
        let check_result = DeviceManage::bind_check(&path).unwrap_or_default();
        if !"bound_this".eq(check_result.as_str()) { //如果未和本设备绑定则进行绑定操作
            let bind_result = DeviceManage::bind_acquire(&bind_code).unwrap_or_default();
            if "5A".eq(bind_result.as_str()) {
                println!("{:?}", "binding success");
            }else {
                println!("{:?}", "binding error");
                return;
            }
        }
    }

    #[test]
    fn get_parent_path_test(){
        let path = "m/44'/0'/0'/0/0";
        let parent_path = get_parent_path(path);
        println!("parent path : {}", parent_path);
    }
}