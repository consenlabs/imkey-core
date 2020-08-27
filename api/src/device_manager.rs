use crate::api::CommonResponse;
use crate::error_handling::Result;
use crate::message_handler::encode_message;
use common::applet;
use common::constants;
use device::device_manager;
use device::deviceapi::{
    AppDeleteReq, AppDownloadReq, AppUpdateReq, AvailableAppBean, BindAcquireReq, BindAcquireRes,
    BindCheckReq, BindCheckRes, CheckUpdateRes, CosCheckUpdateRes, DeviceConnectReq,
    GetBatteryPowerRes, GetBleNameRes, GetBleVersionRes, GetFirmwareVersionRes, GetLifeTimeRes,
    GetRamSizeRes, GetSdkInfoRes, GetSeidRes, GetSnRes, IsBlStatusRes, SetBleNameReq,
};
use prost::Message;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
use transport::hid_api::hid_connect;

pub fn app_download(data: &[u8]) -> Result<Vec<u8>> {
    let request: AppDownloadReq = AppDownloadReq::decode(data).expect("imkey_illegal_param");
    device_manager::app_download(request.app_name.as_ref())?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn app_update(data: &[u8]) -> Result<Vec<u8>> {
    let request: AppUpdateReq = AppUpdateReq::decode(data).expect("imkey_illegal_prarm");
    device_manager::app_update(request.app_name.as_ref())?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn app_delete(data: &[u8]) -> Result<Vec<u8>> {
    let request: AppDeleteReq = AppDeleteReq::decode(data).expect("imkey_illegal_param");
    device_manager::app_delete(request.app_name.as_ref())?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn se_activate() -> Result<Vec<u8>> {
    device_manager::active_device()?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn check_update() -> Result<Vec<u8>> {
    let response = device_manager::check_update()?;

    let mut available_bean_list: Vec<AvailableAppBean> = Vec::new();
    for (index, value) in response
        ._ReturnData
        .available_app_bean_list
        .unwrap()
        .iter()
        .enumerate()
    {
        let version = match value.installed_version.as_ref() {
            Some(version) => version,
            None => "none",
        };

        available_bean_list.insert(
            index,
            AvailableAppBean {
                app_name: applet::get_appname_by_instid(value.instance_aid.as_ref().unwrap())
                    .unwrap()
                    .to_string(),
                app_logo: value.app_logo.as_ref().unwrap().to_string(),
                installed_version: version.to_string(),
                last_updated: value.last_updated.as_ref().unwrap().to_string(),
                latest_version: value.latest_version.as_ref().unwrap().to_string(),
                install_mode: value.install_mode.as_ref().unwrap().to_string(),
            },
        );
    }

    let return_code = response._ReturnCode;
    let mut status = constants::IMKEY_DEV_STATUS_LATEST;
    if return_code == constants::TSM_RETURNCODE_DEV_INACTIVATED.to_string() {
        status = constants::IMKEY_DEV_STATUS_INACTIVATED;
    }

    let response_msg = CheckUpdateRes {
        se_id: response._ReturnData.seid.unwrap(),
        sn: response._ReturnData.sn.unwrap(),
        status: status.to_string(),
        sdk_mode: response._ReturnData.sdk_mode.unwrap(),
        available_app_list: available_bean_list,
    };
    encode_message(response_msg)
}

pub fn se_secure_check() -> Result<Vec<u8>> {
    device_manager::check_device()?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn bind_check(data: &[u8]) -> Result<Vec<u8>> {
    let bind_check: BindCheckReq = BindCheckReq::decode(data).expect("imkey_illegal_param");
    let check_result = device_manager::bind_check(&bind_check.file_path)?;
    let response_msg = BindCheckRes {
        bind_status: check_result,
    };
    encode_message(response_msg)
}

pub fn bind_display_code() -> Result<Vec<u8>> {
    device_manager::bind_display_code()?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn bind_acquire(data: &[u8]) -> Result<Vec<u8>> {
    let bind_acquire: BindAcquireReq = BindAcquireReq::decode(data).expect("imkey_illegal_param");
    let bind_result = device_manager::bind_acquire(&bind_acquire.bind_code)?;
    let response_msg = BindAcquireRes { bind_result };
    encode_message(response_msg)
}

pub fn get_seid() -> Result<Vec<u8>> {
    let seid = device_manager::get_se_id().ok().expect("get_seid_error");
    let response_msg = GetSeidRes { seid };
    encode_message(response_msg)
}

pub fn get_sn() -> Result<Vec<u8>> {
    let sn = device_manager::get_sn().ok().expect("get_sn_error");
    let response_msg = GetSnRes { sn };
    encode_message(response_msg)
}

pub fn get_ram_size() -> Result<Vec<u8>> {
    let ram_size = device_manager::get_ram_size()
        .ok()
        .expect("get_ram_size_error");
    let response_msg = GetRamSizeRes { ram_size };
    encode_message(response_msg)
}

pub fn get_firmware_version() -> Result<Vec<u8>> {
    let firmware_version = device_manager::get_firmware_version()
        .ok()
        .expect("get_firmware_version_error");

    let response_msg = GetFirmwareVersionRes { firmware_version };
    encode_message(response_msg)
}

pub fn get_battery_power() -> Result<Vec<u8>> {
    let battery_power = device_manager::get_battery_power()
        .ok()
        .expect("get_battery_power_error");
    let response_msg = GetBatteryPowerRes { battery_power };
    encode_message(response_msg)
}

pub fn get_life_time() -> Result<Vec<u8>> {
    let life_time = device_manager::get_life_time()
        .ok()
        .expect("get_life_time_error");
    let response_msg = GetLifeTimeRes { life_time };
    encode_message(response_msg)
}

pub fn get_ble_name() -> Result<Vec<u8>> {
    let ble_name = device_manager::get_ble_name()
        .ok()
        .expect("get_ble_name_error");

    let response_msg = GetBleNameRes { ble_name };
    encode_message(response_msg)
}

pub fn set_ble_name(data: &[u8]) -> Result<Vec<u8>> {
    let request: SetBleNameReq = SetBleNameReq::decode(data).expect("ble_action");

    device_manager::set_ble_name(request.ble_name)
        .ok()
        .expect("set_ble_name_error");
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

pub fn get_ble_version() -> Result<Vec<u8>> {
    let ble_version = device_manager::get_ble_version()
        .ok()
        .expect("get_ble_version_error");
    let response_msg = GetBleVersionRes { ble_version };
    encode_message(response_msg)
}

pub fn get_sdk_info() -> Result<Vec<u8>> {
    let response_msg = GetSdkInfoRes {
        sdk_version: constants::VERSION.to_string(),
    };
    encode_message(response_msg)
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub fn cos_update() -> Result<Vec<u8>> {
    device_manager::cos_upgrade()?;
    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub fn device_connect(data: &[u8]) -> Result<Vec<u8>> {
    let device_connect_req: DeviceConnectReq =
        DeviceConnectReq::decode(data).expect("imkey_illegal_param");

    hid_connect(&device_connect_req.device_model_name)?;

    encode_message(CommonResponse {
        result: "success".to_string(),
    })
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub fn cos_check_update() -> Result<Vec<u8>> {
    let cos_check_update = device_manager::cos_check_update()?;

    encode_message(CosCheckUpdateRes {
        seid: cos_check_update._ReturnData.seid,
        is_latest: cos_check_update._ReturnData.is_latest,
        latest_cos_version: cos_check_update
            ._ReturnData
            .latest_cos_version
            .unwrap_or_default(),
        update_type: cos_check_update._ReturnData.update_type.unwrap_or_default(),
        description: cos_check_update._ReturnData.description.unwrap_or_default(),
        is_update_success: cos_check_update._ReturnData.is_update_success,
    })
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub fn is_bl_status() -> Result<Vec<u8>> {
    let check_result = device_manager::is_bl_status()?;
    encode_message(IsBlStatusRes { check_result })
}
