use super::app_update;
use super::se_activate;
use super::se_query::{SeQueryRequest};
use super::se_secure_check::SeSecureCheckRequest;
use crate::app_delete::AppDeleteRequest;
use crate::app_download::AppDownloadRequest;
use crate::cos_upgrade::CosUpgradeRequest;
use app_update::AppUpdateRequest;
use common::constants;
use common::applet;
use mq::message::send_apdu;
use se_activate::SeActivateRequest;
use common::apdu::{Apdu, ApduCheck};
use crate::Result;
use crate::device_binding::DeviceManage;
use crate::ServiceResponse;
use crate::se_query::SeQueryResponse;

pub fn get_se_id() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CB800005DFFF028101".to_string())?;
    Ok(String::from(&res[0..res.len()-4]))
    //res.chars().take(res.len() - 4).collect()
}

pub fn get_sn() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CA004400".to_string())?;
    let hex_decode = hex::decode(String::from(&res[0..res.len()-4]));
    match hex_decode {
        Ok(sn) => Ok(String::from_utf8(sn).unwrap()),
        Err(error) => Err(error.into()),
    }
}

pub fn get_ram_size() -> Result<String> {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF02814600".to_string())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_firmware_version() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CB800005DFFF02800300".to_string())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_battery_power() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("00D6FEED01".to_string())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_life_time() -> Result<String> {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("FFDCFEED00".to_string())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_ble_name() -> Result<String> {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("FFDB465400".to_string())?;
    Ok(res.chars().collect())
}

pub fn set_ble_name(ble_name: String) -> Result<String> {
    let apdu = Apdu::set_ble_name(ble_name.as_ref());
    let res = send_apdu(apdu)?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_ble_version() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CB800005DFFF02810000".to_string())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_cert() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CABF2106A6048302151800".to_string())?;
    ApduCheck::checke_response(&res)?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn check_device() -> Result<()> {
    let seid: String = get_se_id().ok().unwrap();
    let sn: String = get_sn().ok().unwrap();
    let device_cert: String = get_cert()?;
    SeSecureCheckRequest::build_request_data(seid, sn, device_cert).se_secure_check()
}

pub fn active_device() -> Result<()>{
    let seid: String = get_se_id().ok().unwrap();
    let sn: String = get_sn().ok().unwrap();
    let device_cert: String = get_cert()?;
    SeActivateRequest::build_request_data(seid, sn, device_cert).se_activate()
}

pub fn check_update() -> Result<ServiceResponse<SeQueryResponse>>  {
    let seid: String = get_se_id().ok().unwrap();
    let sn: String = get_sn().ok().unwrap();
    let sdk_version = Some(constants::VERSION.to_string());
    SeQueryRequest::build_request_data(seid, sn, sdk_version).se_query()
}

pub fn app_download(app_name: &str) -> Result<()> {
    let seid: String = get_se_id().ok().unwrap();
    let device_cert: String = get_cert()?;
    let sdk_version = Some(constants::VERSION.to_string());
    let instance_aid: String = applet::get_instid_by_appname(app_name).expect("imkey_app_name_not_exist").to_string();
    AppDownloadRequest::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .app_download()
}

pub fn app_update(app_name: &str) -> Result<()> {
    let seid: String = get_se_id().ok().unwrap();
    let device_cert: String = get_cert()?;
    let sdk_version = Some(constants::VERSION.to_string());
    let instance_aid: String = applet::get_instid_by_appname(app_name).expect("imkey_app_name_not_exist").to_string();
    AppUpdateRequest::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .app_update()
}

pub fn app_delete(app_name: &str) -> Result<()> {
    let seid: String = get_se_id().ok().unwrap();
    let device_cert: String = get_cert()?;
    let instance_aid: String = applet::get_instid_by_appname(app_name).expect("imkey_app_name_not_exist").to_string();
    AppDeleteRequest::build_request_data(seid, instance_aid, device_cert).app_delete()
}


pub fn bind_check(file_path: &str) -> Result<String> {
    DeviceManage::bind_check(&file_path.to_string())
}

pub fn bind_display_code() -> Result<()> {
    DeviceManage::display_bind_code()
}

pub fn bind_acquire(bind_code: &str) -> Result<String> {
    DeviceManage::bind_acquire(&bind_code.to_string())
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn cos_upgrade() -> Result<()> {
    CosUpgradeRequest::cos_upgrade(None)
}

