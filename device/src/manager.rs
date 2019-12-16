use super::app_delete;
use super::app_download;
use super::app_update;
use super::se_activate;
use super::se_query::se_query_request;
use super::se_secure_check::se_secure_check_request;
use crate::app_delete::app_delete_request;
use crate::app_download::app_download_request;
use app_update::app_update_request;
use common::constants;
use mq::message::send_apdu;
use se_activate::se_activate_request;

pub fn get_se_id() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF028101".to_string());
    res.chars().take(res.len() - 4).collect()
}

pub fn get_sn() -> String {
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CA004400".to_string());
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

pub fn check_update() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert: String = get_cert();
    let sdk_version = Some(constants::VERSION.to_string());

    let instance_aid: String = "695F657468".to_string();
    //todo: param an return
    se_query_request::build_request_data(seid, sn, sdk_version).se_query();
}

pub fn app_download() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert: String = get_cert();
    let sdk_version = Some(constants::VERSION.to_string());

    let instance_aid: String = "695F657468".to_string();
    //todo: param an return
    app_download_request::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .app_download();
}

pub fn app_update() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert: String = get_cert();
    let sdk_version = Some(constants::VERSION.to_string());

    let instance_aid: String = "695F657468".to_string();
    //todo: param an return
    app_update_request::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .app_update();
}

pub fn app_delete() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert: String = get_cert();

    let instance_aid: String = "695F657468".to_string();
    //todo: param an return
    app_delete_request::build_request_data(seid, instance_aid, device_cert).app_delete();
}
