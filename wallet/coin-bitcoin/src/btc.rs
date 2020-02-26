use bitcoin::{Address, PublicKey, Network};
use std::str::FromStr;
use bitcoin::util::bip32::{ExtendedPubKey, ChainCode, ChildNumber, DerivationPath, Fingerprint};
use hex::decode;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::hashes::core::convert::TryFrom;
use bitcoin::hashes::{hash160, Hash};
use common::error::ImkeyError;
use common::path::check_path_validity;
use crate::common::get_xpub_data;

/**
get btc xpub by path
*/
pub fn get_xpub(network : Network, path : &str) -> Result<String, ImkeyError>{

    //path check
    let check_result = check_path_validity(path);
    if check_result.is_err() {
        return Err(ImkeyError::IMKEY_PATH_ILLEGAL);
    }

    //get xpub data
    let xpub_data_result = get_xpub_data(path, true);
    if xpub_data_result.is_err() {
        return Err(xpub_data_result.err().unwrap());
    }
    let xpub_data = xpub_data_result.ok().unwrap();
    let xpub_data = &xpub_data[..194].to_string();

    //get public key and chain code
    let pub_key = &xpub_data[..130];
    let chain_code = &xpub_data[130..];

    //build parent public key obj
    let parent_xpub_result = get_xpub_data(get_parent_path(path), true);
    if parent_xpub_result.is_err() {
        return Err(parent_xpub_result.err().unwrap());
    }
    let parent_xpub = parent_xpub_result.ok().unwrap();
    let parent_xpub = &parent_xpub[..130].to_string();

    let parent_pub_key_result = PublicKey::from_str(parent_xpub);
    if parent_pub_key_result.is_err() {
        return Err(ImkeyError::INVALID_PUBLIC_KEY);
    }
    let mut parent_pub_key_obj = parent_pub_key_result.ok().unwrap();
    parent_pub_key_obj.compressed = true;

    //build child public key obj
    let pub_key_result = PublicKey::from_str(pub_key);
    if pub_key_result.is_err() {
        return Err(ImkeyError::INVALID_PUBLIC_KEY);
    }
    let mut pub_key_obj = pub_key_result.ok().unwrap();
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
    let chain_number_vec: Vec<ChildNumber> = DerivationPath::from_str(path).unwrap().into();
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
pub fn get_address(network : Network, path : &str) -> Result<String, ImkeyError>{
    //path check
    let check_result = check_path_validity(path);
    if check_result.is_err() {
        return Err(ImkeyError::IMKEY_PATH_ILLEGAL);
    }

    //get xpub
    let xpub_data_result = get_xpub_data(path, true);
    if xpub_data_result.is_err() {
        return Err(xpub_data_result.err().unwrap());
    }
    let xpub_data = xpub_data_result.ok().unwrap();
    let pub_key = &xpub_data[..130];

    let pub_key_result = PublicKey::from_str(pub_key);
    if pub_key_result.is_err() {
        return Err(ImkeyError::INVALID_PUBLIC_KEY);
    }
    let mut pub_key_obj= pub_key_result.unwrap();
    pub_key_obj.compressed = true;

    Ok(Address::p2pkh(&pub_key_obj, network).to_string())
}

/**
get segwit address by path
*/
pub fn get_segwit_address(network : Network, path : &str) -> Result<String, ImkeyError>{
    //path check
    let check_result = check_path_validity(path);
    if check_result.is_err() {
        return Err(ImkeyError::IMKEY_PATH_ILLEGAL);
    }

    //get xpub
    let xpub_data_result = get_xpub_data(path, true);
    if xpub_data_result.is_err() {
        return Err(xpub_data_result.err().unwrap());
    }
    let xpub_data = xpub_data_result.ok().unwrap();
    let pub_key = &xpub_data[..130];

    let pub_key_result = PublicKey::from_str(pub_key);
    if pub_key_result.is_err() {
        return Err(ImkeyError::INVALID_PUBLIC_KEY);
    }
    let mut pub_key_obj= pub_key_result.unwrap();
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
        let path = "/Users/caixiaoguang/workspace/myproject/imkey-core/".to_string();
        let bind_code = "E4APZZRT".to_string();
        let mut device_manage = DeviceManage::new();
        let check_result = device_manage.bind_check(&path);
        if !"bound_this".eq(check_result.as_str()) { //如果未和本设备绑定则进行绑定操作
            let bind_result = device_manage.bind_acquire(&bind_code);
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