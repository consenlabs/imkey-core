use mq::message::send_apdu;
use common::apdu::BtcApdu;
use bitcoin::{Address, PublicKey, Network};
use std::str::FromStr;
use bitcoin::util::bip32::{ExtendedPubKey, ChainCode, ChildNumber, DerivationPath, Fingerprint};
use hex::decode;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::hashes::core::convert::TryFrom;
use bitcoin::hashes::{hash160, Hash};

/**
get btc xpub by path
*/
pub fn get_xpub(network : Network, path : &str) -> String{

    //path check TODO

    //get main public key(xpub)
    let apdu_response = send_apdu(BtcApdu::select_applet());
    if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
        panic!("selcet btc error");
    }
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, true));
    if !"9000".eq(&xpub_data[xpub_data.len() - 4 ..]) {
        panic!("get xpub apdu error");
    }
    let xpub_data = &xpub_data[..194].to_string();
    //get public key and chain code
    let pub_key = &xpub_data[..130];
    let chain_code = &xpub_data[130..];

    //build parent public key obj
    let parent_xpub_data = send_apdu(BtcApdu::get_xpub(get_parent_path(path), true));
    let mut parent_pub_key_obj = PublicKey::from_str(&parent_xpub_data[..130]).expect("budile public obj error");
    parent_pub_key_obj.compressed = true;
    //build child public key obj
    let mut pub_key_obj = PublicKey::from_str(pub_key).expect("budile public obj error");
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
    extend_public_key.to_string()

}

/**
get btc address by path
*/
pub fn get_address(network : Network, path : &str) -> String{
    //path check TODO

    //get main public key(xpub)
    let apdu_response = send_apdu(BtcApdu::select_applet());
    if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
        panic!("selcet btc error");
    }
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, true));
    if !"9000".eq(&xpub_data[xpub_data.len() - 4 ..]) {
        panic!("get xpub apdu error");
    }
    let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();
    let pub_key = &xpub_data[..130];

    let mut pub_key_obj = PublicKey::from_str(pub_key).expect("budile public obj error");
    pub_key_obj.compressed = true;

    Address::p2pkh(&pub_key_obj, network).to_string()
}

/**
get segwit address by path
*/
pub fn get_segwit_address(network : Network, path : &str) -> String{
    //path check TODO

    //get main public key(xpub)
    let apdu_response = send_apdu(BtcApdu::select_applet());
    if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
        panic!("selcet btc error");
    }
    let xpub_data = send_apdu(BtcApdu::get_xpub(path, true));
    if !"9000".eq(&xpub_data[xpub_data.len() - 4 ..]) {
        panic!("get xpub apdu error");
    }
    let xpub_data = &xpub_data[..xpub_data.len() - 4].to_string();
    let pub_key = &xpub_data[..130];

    let mut pub_key_obj = PublicKey::from_str(pub_key).expect("budile public obj error");
    pub_key_obj.compressed = true;

    Address::p2shwpkh(&pub_key_obj, network).to_string()
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
        let xpub_str = get_xpub(version, path);
        println!("xpub : {:?}", xpub_str);
    }

    #[test]
    fn get_address_test(){
        //device binding
        device_binding_test();

        let version : Network = Network::Bitcoin;
        let path : &str = "m/44'/0'/0'/0/0";
        let btc_address_str = get_address(version, path);
        println!("btc address : {:?}", btc_address_str);
    }

    #[test]
    fn get_segwit_address_test(){
        //device binding
        device_binding_test();

        let version : Network = Network::Bitcoin;
        let path : &str = "m/49'/0'/0'/0/22";
        let segwit_address_str = get_segwit_address(version, path);
        println!("segwit address : {:?}", segwit_address_str);
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