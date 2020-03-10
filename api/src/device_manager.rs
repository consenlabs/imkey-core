use crate::api::DeviceParam;
use crate::deviceapi::{AppAction, SdkInfoResponse};
use crate::deviceapi::DeviceCert;
use crate::deviceapi::DeviceName;
use crate::deviceapi::EmptyResponse;
use crate::deviceapi::{AuthCode, AuthCodeResponse, AuthCodeServiceResponse};
use crate::deviceapi::{
    BindAcquire, BindAcquireResponse, BindCheck, BindDisplay, BindDisplayResponse, ApduResponse
};
use crate::deviceapi::{SeAction, SeQueryResponse, CheckUpdateResponse, AvailableAppBean, BleAction};
use crate::wallet_handler::encode_message;
use common::error::Error;
use common::constants;
use common::applet;
use device::app_delete::app_delete_request;
use device::app_download::app_download_request;
use device::app_update::app_update_request;
use device::auth_code_storage::auth_code_storage_request;
use device::device_binding::{display_bind_code, DeviceManage};
use device::device_cert_check::device_cert_check_request;
use device::se_activate::se_activate_request;
use device::se_query::{se_query_request, available_app_bean};
use device::se_secure_check::se_secure_check_request;
use prost::Message;
use device::manager;
use mq::message;
use std::ffi::{CStr, CString};
use crate::error_handling::Result;


pub fn device_app_delete(param: &DeviceParam) -> Result<Vec<u8>> {
    let app_action: AppAction =
        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("app_action");
    manager::app_delete(app_action.app_name.as_ref());
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_app_download(param: &DeviceParam) -> Result<Vec<u8>> {
//    let app_action: AppAction =
//        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
//            .expect("app_action");
//    let mut request = app_download_request::build_request_data(
//        app_action.se_id,
//        app_action.instance_aid,
//        app_action.device_cert,
//        Some(app_action.sdk_version),
//    );
//    let _response = request
//        .app_download()
//        .map_err(|_err| Error::DeviceOpError)?;
    let app_action: AppAction =
        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("app_action");
    manager::app_download(app_action.app_name.as_ref());
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_app_update(param: &DeviceParam) -> Result<Vec<u8>> {
//    let app_action: AppAction =
//        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
//            .expect("app_action");
//    let mut request = app_update_request::build_request_data(
//        app_action.se_id,
//        app_action.instance_aid,
//        app_action.device_cert,
//        Some(app_action.sdk_version),
//    );
//    let _response = request.app_update().map_err(|_err| Error::DeviceOpError)?;
    let app_action: AppAction =
        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("app_action");
    manager::app_update(app_action.app_name.as_ref());
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_activate(param: &DeviceParam) -> Result<Vec<u8>> {
//    let se_action: SeAction =
////        SeAction::decode(&param.param.as_ref().expect("device_param").value.clone())
////            .expect("se_activate");
////    let mut request = se_activate_request::build_request_data(
////        se_action.se_id,
////        se_action.sn,
////        se_action.device_cert,
////    );
////    let _response = request.se_activate().map_err(|_err| Error::DeviceOpError)?;

    manager::active_device();
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_query(param: &DeviceParam) -> Result<Vec<u8>> {

    let response = manager::check_update().map_err(|_err| Error::DeviceOpError)?;

    let mut available_bean_list: Vec<AvailableAppBean> = Vec::new();
    for (index, value) in response._ReturnData.availableAppBeanList.unwrap().iter().enumerate() {

        let version = match value.installedVersion.as_ref() {
            Some(version) => version,
            None => "none",
        };

        available_bean_list.insert(index, AvailableAppBean {
            app_name : applet::get_appname_by_instid(value.instanceAid.as_ref().unwrap()).unwrap().to_string(),
            app_logo: value.appLogo.as_ref().unwrap().to_string(),
            installed_version: version.to_string(),
            last_updated: value.lastUpdated.as_ref().unwrap().to_string(),
            latest_version: value.latestVersion.as_ref().unwrap().to_string(),
            install_mode: value.installMode.as_ref().unwrap().to_string(),
        });

    }

    let return_code = response._ReturnCode;
    let mut status = constants::IMKEY_DEV_STATUS_LATEST;
    if (return_code == constants::TSM_RETURNCODE_DEV_INACTIVATED.to_string()) {
        status = constants::IMKEY_DEV_STATUS_INACTIVATED;
    }

    let response_msg = CheckUpdateResponse {
        se_id: response._ReturnData.seid.unwrap(),
        sn: response._ReturnData.sn.unwrap(),
        status : status.to_string(),
        sdk_mode : response._ReturnData.sdkMode.unwrap(),
        available_app_list : available_bean_list,
    };
    encode_message(response_msg)
}

pub fn device_secure_check(param: &DeviceParam) -> Result<Vec<u8>> {
    manager::check_device();
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_bind_check(param: &DeviceParam) -> Result<Vec<u8>> {
    let bind_check: BindCheck =
        BindCheck::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("bind_check");
    let _check_result = DeviceManage::new().bind_check(&bind_check.file_path);
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_bind_acquire(param: &DeviceParam) -> Result<Vec<u8>> {
    let bind_acquire: BindAcquire =
        BindAcquire::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("bind_acquire");
    let bind_result = DeviceManage::new().bind_acquire(&bind_acquire.bind_code).ok().expect("bind_acquire_error");
    let response_msg = BindAcquireResponse {
        bind_result: bind_result,
    };
    encode_message(response_msg)
}

pub fn device_display_bind_code(param: &DeviceParam) -> Result<Vec<u8>> {
    let _bind_display: BindDisplay =
        BindDisplay::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("bind_display_code");
    let display_result = display_bind_code().ok().expect("display_bind_code_error"); //no param
    let response_msg = BindDisplayResponse {
        bind_display_result: display_result,
    };
    encode_message(response_msg)
}



pub fn get_seid(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_se_id().ok().expect("get_seid_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_sn(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_sn().ok().expect("get_sn_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_ram_size(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_ram_size().ok().expect("get_ram_size_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_firmware_version(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_firmware_version().ok().expect("get_firmware_version_error");

    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_battery_power(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_battery_power().ok().expect("get_battery_power_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_life_time(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_life_time().ok().expect("get_life_time_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_ble_name(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_ble_name().ok().expect("get_ble_name_error");

    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn set_ble_name(param: &DeviceParam) -> Result<Vec<u8>> {

    let ble_action: BleAction =
        BleAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("ble_action");

    let result = manager::set_ble_name(ble_action.ble_name).ok().expect("set_ble_name_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_ble_version(param: &DeviceParam) -> Result<Vec<u8>> {

    let result = manager::get_ble_version().ok().expect("get_ble_version_error");
    let response_msg = ApduResponse {
        result: result,
    };
    encode_message(response_msg)
}

pub fn get_sdk_info(param: &DeviceParam) -> Result<Vec<u8>> {
    let response_msg = SdkInfoResponse {
        sdk_version: constants::VERSION.to_string(),
    };
    encode_message(response_msg)
}
