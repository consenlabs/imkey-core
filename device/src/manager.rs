use super::app_delete;
use super::app_download;
use super::app_update;
use super::se_activate;
use super::se_query::{se_query_request, service_response};
use super::se_secure_check::se_secure_check_request;
use crate::app_delete::app_delete_request;
use crate::app_download::app_download_request;
use app_update::app_update_request;
use common::constants;
use common::applet;
use mq::message::send_apdu;
use se_activate::se_activate_request;
use common::{error::ImkeyError};
use hex::decode;
use futures::future::err;
use common::apdu::Apdu;


pub fn get_se_id() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF028101".to_string());
    String::from(&res[0..res.len()-4])
    //res.chars().take(res.len() - 4).collect()
}

pub fn get_sn() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CA004400".to_string());
    let hex_decode = hex::decode(String::from(&res[0..res.len()-4]));
    match hex_decode {
        Ok(sn) => String::from_utf8(sn).unwrap(),
        Err(error) => error.to_string()
    }
}

pub fn get_ram_size() -> String {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF02814600".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn get_firmware_version() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF02800300".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn get_battery_power() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("00D6FEED01".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn get_life_time() -> String {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("FFDCFEED00".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn get_ble_name() -> String {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("FFDB465400".to_string());
    res.chars().collect()
}

pub fn set_ble_name(ble_name: String) -> String {
    let apdu = Apdu::set_ble_name(ble_name.as_ref());
    let res = send_apdu(apdu);
    res.chars().take(res.len() - 4).collect()
}

pub fn get_ble_version() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF02810000".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn get_cert() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CABF2106A6048302151800".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn check_device() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert: String = get_cert();

    match se_secure_check_request::build_request_data(seid, sn, device_cert).se_secure_check() {
        Ok(()) => println!("success!"),
        Err(e) => println!("{}", e),
    }
}

pub fn active_device() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert: String = get_cert();

    se_activate_request::build_request_data(seid, sn, device_cert).se_activate();
}

pub fn check_update() -> Result<service_response, ImkeyError>  {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let sdk_version = Some(constants::VERSION.to_string());
    se_query_request::build_request_data(seid, sn, sdk_version).se_query()
}

pub fn app_download(app_name: &str) {
    let seid: String = get_se_id();
    let device_cert: String = get_cert();
    let sdk_version = Some(constants::VERSION.to_string());
    let instance_aid: String = applet::get_instid_by_appname(app_name).unwrap().to_string();
    app_download_request::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .app_download();
}

pub fn app_update(app_name: &str) {
    let seid: String = get_se_id();
    let device_cert: String = get_cert();
    let sdk_version = Some(constants::VERSION.to_string());
    let instance_aid: String = applet::get_instid_by_appname(app_name).unwrap().to_string();
    app_update_request::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .app_update();
}

pub fn app_delete(app_name: &str) {
    let seid: String = get_se_id();
    let device_cert: String = get_cert();
    let instance_aid: String = applet::get_instid_by_appname(app_name).unwrap().to_string();
    app_delete_request::build_request_data(seid, instance_aid, device_cert).app_delete();
}

