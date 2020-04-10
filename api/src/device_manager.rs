use device::deviceapi::{AppDownloadReq, AppUpdateReq, AppDeleteReq, CheckUpdateRes, AvailableAppBean,
                        BindCheckReq, BindCheckRes, BindAcquireReq, BindAcquireRes, GetSeidRes,
                        GetSnRes, GetRamSizeRes, GetFirmwareVersionRes, GetBatteryPowerRes,
                        GetLifeTimeRes, GetBleNameRes, SetBleNameReq, GetBleVersionRes, GetSdkInfoRes,
                        EmptyResponse, DeviceModelListRes};
use crate::message_handler::encode_message;
use common::constants;
use common::applet;
use prost::Message;
use device::manager;
use crate::error_handling::Result;
use common::constants::DEVICE_MODEL_NAME;
use mq::hid_api::device_connect as device_conn;

pub fn app_download(data: &[u8]) -> Result<Vec<u8>> {
    let request: AppDownloadReq = AppDownloadReq::decode(data).expect("imkey_illegal_param");
    manager::app_download(request.app_name.as_ref())?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn app_update(data: &[u8]) -> Result<Vec<u8>> {
    let request: AppUpdateReq = AppUpdateReq::decode(data).expect("imkey_illegal_prarm");
    manager::app_update(request.app_name.as_ref())?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn app_delete(data: &[u8]) -> Result<Vec<u8>> {
    let request: AppDeleteReq = AppDeleteReq::decode(data).expect("imkey_illegal_param");
    manager::app_delete(request.app_name.as_ref())?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn se_activate() -> Result<Vec<u8>> {
    manager::active_device()?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn check_update() -> Result<Vec<u8>> {

    let response = manager::check_update()?;

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
    if return_code == constants::TSM_RETURNCODE_DEV_INACTIVATED.to_string() {
        status = constants::IMKEY_DEV_STATUS_INACTIVATED;
    }

    let response_msg = CheckUpdateRes {
        se_id: response._ReturnData.seid.unwrap(),
        sn: response._ReturnData.sn.unwrap(),
        status : status.to_string(),
        sdk_mode : response._ReturnData.sdkMode.unwrap(),
        available_app_list : available_bean_list,
    };
    encode_message(response_msg)
}

pub fn se_secure_check() -> Result<Vec<u8>> {
    manager::check_device()?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn bind_check(data: &[u8]) -> Result<Vec<u8>> {
    let bind_check: BindCheckReq = BindCheckReq::decode(data).expect("imkey_illegal_param");
    let check_result = manager::bind_check(&bind_check.file_path)?;
    let response_msg = BindCheckRes{
        bind_status: check_result
    };
    encode_message(response_msg)
}

pub fn bind_display_code() -> Result<Vec<u8>> {
    manager::bind_display_code()?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn bind_acquire(data: &[u8]) -> Result<Vec<u8>> {
    let bind_acquire: BindAcquireReq = BindAcquireReq::decode(data).expect("imkey_illegal_param");
    let bind_result = manager::bind_acquire(&bind_acquire.bind_code)?;
    let response_msg = BindAcquireRes {
        bind_result,
    };
    encode_message(response_msg)
}

pub fn get_seid() -> Result<Vec<u8>> {

    let seid = manager::get_se_id().ok().expect("get_seid_error");
    let response_msg = GetSeidRes {
        seid,
    };
    encode_message(response_msg)
}

pub fn get_sn() -> Result<Vec<u8>> {

    let sn = manager::get_sn().ok().expect("get_sn_error");
    let response_msg = GetSnRes {
        sn,
    };
    encode_message(response_msg)
}

pub fn get_ram_size() -> Result<Vec<u8>> {

    let ram_size = manager::get_ram_size().ok().expect("get_ram_size_error");
    let response_msg = GetRamSizeRes {
        ram_size,
    };
    encode_message(response_msg)
}

pub fn get_firmware_version() -> Result<Vec<u8>> {

    let firmware_version = manager::get_firmware_version().ok().expect("get_firmware_version_error");

    let response_msg = GetFirmwareVersionRes {
        firmware_version,
    };
    encode_message(response_msg)
}

pub fn get_battery_power() -> Result<Vec<u8>> {

    let battery_power = manager::get_battery_power().ok().expect("get_battery_power_error");
    let response_msg = GetBatteryPowerRes {
        battery_power,
    };
    encode_message(response_msg)
}

pub fn get_life_time() -> Result<Vec<u8>> {

    let life_time = manager::get_life_time().ok().expect("get_life_time_error");
    let response_msg = GetLifeTimeRes {
        life_time,
    };
    encode_message(response_msg)
}

pub fn get_ble_name() -> Result<Vec<u8>> {

    let ble_name = manager::get_ble_name().ok().expect("get_ble_name_error");

    let response_msg = GetBleNameRes {
        ble_name,
    };
    encode_message(response_msg)
}

pub fn set_ble_name(data: &[u8]) -> Result<Vec<u8>> {

    let request: SetBleNameReq = SetBleNameReq::decode(data).expect("ble_action");

    manager::set_ble_name(request.ble_name).ok().expect("set_ble_name_error");
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn get_ble_version() -> Result<Vec<u8>> {

    let ble_version = manager::get_ble_version().ok().expect("get_ble_version_error");
    let response_msg = GetBleVersionRes {
        ble_version,
    };
    encode_message(response_msg)
}

pub fn get_sdk_info() -> Result<Vec<u8>> {
    let response_msg = GetSdkInfoRes {
        sdk_version: constants::VERSION.to_string(),
    };
    encode_message(response_msg)
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn cos_update() -> Result<Vec<u8>> {
    manager::cos_upgrade()?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_model_list() -> Result<Vec<u8>>{
    let response_msg = DeviceModelListRes {
        device_model_name: DEVICE_MODEL_NAME.to_string(),
    };
    encode_message(response_msg)
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn device_connect() -> Result<Vec<u8>>{

    device_conn()?;

    encode_message(EmptyResponse{})
}
