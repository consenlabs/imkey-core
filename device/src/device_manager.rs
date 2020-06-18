use super::app_update;
use super::se_activate;
use super::se_query::SeQueryRequest;
use super::se_secure_check::SeSecureCheckRequest;
use crate::app_delete::AppDeleteRequest;
use crate::app_download::AppDownloadRequest;
use crate::cos_check_update::{CosCheckUpdateRequest, CosCheckUpdateResponse};
use crate::cos_upgrade::CosUpgradeRequest;
use crate::device_binding::DeviceManage;
use crate::se_query::SeQueryResponse;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use app_update::AppUpdateRequest;
use common::apdu::{Apdu, ApduCheck};
use common::applet;
use common::constants;
use se_activate::SeActivateRequest;
use transport::message::send_apdu;

pub fn get_se_id() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CB800005DFFF028101".to_string())?;
    ApduCheck::checke_response(res.as_str())?;
    Ok(String::from(&res[0..res.len() - 4]))
}

pub fn get_sn() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CA004400".to_string())?;
    ApduCheck::checke_response(res.as_str())?;
    let hex_decode = hex::decode(String::from(&res[0..res.len() - 4]));
    match hex_decode {
        Ok(sn) => Ok(String::from_utf8(sn).unwrap()),
        Err(error) => Err(error.into()),
    }
}

pub fn get_ram_size() -> Result<String> {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF02814600".to_string())?;
    ApduCheck::checke_response(res.as_str())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_firmware_version() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("80CB800005DFFF02800300".to_string())?;
    ApduCheck::checke_response(res.as_str())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_battery_power() -> Result<String> {
    send_apdu("00A4040000".to_string())?;
    let res = send_apdu("00D6FEED01".to_string())?;
    ApduCheck::checke_response(res.as_str())?;
    Ok(res.chars().take(res.len() - 4).collect())
}

pub fn get_life_time() -> Result<String> {
    //send_apdu("00A4040000".to_string());
    let res = send_apdu("FFDCFEED00".to_string())?;
    ApduCheck::checke_response(res.as_str())?;
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
    let seid: String = get_se_id()?;
    let sn: String = get_sn()?;
    let device_cert: String = get_cert()?;
    SeSecureCheckRequest::build_request_data(seid, sn, device_cert).send_message()
}

pub fn active_device() -> Result<()> {
    let seid: String = get_se_id()?;
    let sn: String = get_sn()?;
    let device_cert: String = get_cert()?;
    SeActivateRequest::build_request_data(seid, sn, device_cert).send_message()
}

pub fn check_update() -> Result<ServiceResponse<SeQueryResponse>> {
    let seid: String = get_se_id()?;
    let sn: String = get_sn()?;
    let sdk_version = Some(constants::VERSION.to_string());
    SeQueryRequest::build_request_data(seid, sn, sdk_version).send_message()
}

pub fn app_download(app_name: &str) -> Result<()> {
    let seid: String = get_se_id()?;
    let device_cert: String = get_cert()?;
    let sdk_version = Some(constants::VERSION.to_string());
    let instance_aid: String = applet::get_instid_by_appname(app_name)
        .expect("imkey_app_name_not_exist")
        .to_string();
    AppDownloadRequest::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .send_message()
}

pub fn app_update(app_name: &str) -> Result<()> {
    let seid: String = get_se_id()?;
    let device_cert: String = get_cert()?;
    let sdk_version = Some(constants::VERSION.to_string());
    let instance_aid: String = applet::get_instid_by_appname(app_name)
        .expect("imkey_app_name_not_exist")
        .to_string();
    AppUpdateRequest::build_request_data(seid, instance_aid, device_cert, sdk_version)
        .send_message()
}

pub fn app_delete(app_name: &str) -> Result<()> {
    let seid: String = get_se_id()?;
    let device_cert: String = get_cert()?;
    let instance_aid: String = applet::get_instid_by_appname(app_name)
        .expect("imkey_app_name_not_exist")
        .to_string();
    AppDeleteRequest::build_request_data(seid, instance_aid, device_cert).send_message()
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

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn cos_check_update() -> Result<ServiceResponse<CosCheckUpdateResponse>> {
    let seid = get_se_id()?;
    let mut cos_version = get_firmware_version()?;
    cos_version = format!(
        "{}.{}.{}",
        cos_version[0..1].to_string(),
        cos_version[1..2].to_string(),
        cos_version[2..].to_string()
    );
    CosCheckUpdateRequest::build_request_data(seid, cos_version).send_message()
}
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn is_bl_status() -> Result<bool> {
    let res_data = send_apdu(Apdu::select_applet(constants::BL_AID))?;
    let check_result = ApduCheck::checke_response(res_data.as_str());
    if check_result.is_err() {
        return Ok(false);
    }
    Ok(true)
}

#[cfg(test)]
mod test {
    use crate::device_manager::{
        active_device, app_delete, app_download, app_update, bind_check, get_se_id, is_bl_status,
    };
    use common::constants;
    use transport::hid_api::hid_connect;

    #[test]
    fn is_bl_status_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = is_bl_status();
        assert!(result.is_ok());
    }

    #[test]
    fn app_download_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = app_download("BTC");
        assert!(result.is_ok());
    }

    #[test]
    fn app_download_wrong_appname_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        //Enter the wrong app name
        let result = app_download("TEST");
        println!("{}", result.err().unwrap());
    }

    #[test]
    fn app_update_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = app_update("BTC");
        assert!(result.is_ok());
    }

    #[test]
    fn app_update_wrong_app_name_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = app_update("TEST");
        assert!(result.is_ok());
    }

    #[test]
    fn app_delete_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = app_delete("COSMOS");
        assert!(result.is_ok());
    }

    #[test]
    fn app_delete_wrong_app_name_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = app_delete("TEST");
        assert!(result.is_ok());
    }

    #[test]
    fn bind_check_wrong_path_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = bind_check("/test/");
        assert!(result.is_ok());
    }

    #[test]
    fn active_device_test() {
        assert!(hid_connect(constants::DEVICE_MODEL_NAME).is_ok());
        let result = active_device();
        assert!(result.is_ok());
    }
}
