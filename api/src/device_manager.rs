use crate::api::DeviceParam;
use crate::deviceapi::AppAction;
use crate::deviceapi::DeviceCert;
use crate::deviceapi::DeviceName;
use crate::deviceapi::EmptyResponse;
use crate::deviceapi::{AuthCode, AuthCodeResponse, AuthCodeServiceResponse};
use crate::deviceapi::{
    BindAcquire, BindAcquireResponse, BindCheck, BindDisplay, BindDisplayResponse,
};
use crate::deviceapi::{SeAction, SeQueryResponse, SeQueryServiceResponse};
use crate::wallet_handler::encode_message;
use common::error::Error;
use device::app_delete::app_delete_request;
use device::app_download::app_download_request;
use device::app_update::app_update_request;
use device::auth_code_storage::auth_code_storage_request;
use device::device_binding::{display_bind_code, DeviceManage};
use device::device_cert_check::device_cert_check_request;
use device::se_activate::se_activate_request;
use device::se_query::se_query_request;
use device::se_secure_check::se_secure_check_request;
use prost::Message;

pub fn device_store_authcode(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let auth_code: AuthCode =
        AuthCode::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("auth_code");
    let mut request =
        auth_code_storage_request::build_request_data(auth_code.se_id, auth_code.auth_code);
    let response = request
        .auth_code_storage()
        .map_err(|_err| Error::DeviceOpError)?;
    let response_msg = AuthCodeServiceResponse {
        return_code: response._ReturnCode,
        return_msg: response._ReturnMsg,
        return_data: Some(AuthCodeResponse {
            se_id: response._ReturnData.seid.unwrap(),
            next_stepkey: response._ReturnData.nextStepKey.unwrap(),
            apdu_list: response._ReturnData.apduList.unwrap(),
        }),
    };
    encode_message(response_msg)
}

pub fn device_app_delete(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let app_action: AppAction =
        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("app_action");
    let mut request = app_delete_request::build_request_data(
        app_action.se_id,
        app_action.instance_aid,
        app_action.device_cert,
    );
    let _response = request.app_delete().map_err(|_err| Error::DeviceOpError)?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_app_download(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let app_action: AppAction =
        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("app_action");
    let mut request = app_download_request::build_request_data(
        app_action.se_id,
        app_action.instance_aid,
        app_action.device_cert,
        Some(app_action.sdk_version),
    );
    let _response = request
        .app_download()
        .map_err(|_err| Error::DeviceOpError)?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_app_update(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let app_action: AppAction =
        AppAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("app_action");
    let mut request = app_update_request::build_request_data(
        app_action.se_id,
        app_action.instance_aid,
        app_action.device_cert,
        Some(app_action.sdk_version),
    );
    let _response = request.app_update().map_err(|_err| Error::DeviceOpError)?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_cert_check(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let device_cert: DeviceCert =
        DeviceCert::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("cert_check");
    let mut request = device_cert_check_request::build_request_data(
        device_cert.se_id,
        device_cert.sn,
        device_cert.device_cert,
    );
    let _response = request
        .device_cert_check()
        .map_err(|_err| Error::DeviceOpError)?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_activate(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let se_action: SeAction =
        SeAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("se_activate");
    let mut request = se_activate_request::build_request_data(
        se_action.se_id,
        se_action.sn,
        se_action.device_cert,
    );
    let _response = request.se_activate().map_err(|_err| Error::DeviceOpError)?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_query(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let se_action: SeAction =
        SeAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("se_query");
    let mut request = se_query_request::build_request_data(
        se_action.se_id,
        se_action.sn,
        Some(se_action.sdk_version),
    );
    let response = request.se_query().map_err(|_err| Error::DeviceOpError)?;
    let response_msg = SeQueryServiceResponse {
        return_code: response._ReturnCode,
        return_msg: response._ReturnMsg,
        return_data: Some(SeQueryResponse {
            se_id: response._ReturnData.seid.unwrap(),
            next_stepkey: response._ReturnData.nextStepKey.unwrap(),
            apdu_list: response._ReturnData.apduList.unwrap(),
        }),
    };
    encode_message(response_msg)
}

pub fn device_secure_check(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let se_action: SeAction =
        SeAction::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("se_secure_check");
    let mut request = se_secure_check_request::build_request_data(
        se_action.se_id,
        se_action.sn,
        se_action.device_cert,
    );
    let _response = request
        .se_secure_check()
        .map_err(|_err| Error::DeviceOpError)?;
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_bind_check(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let bind_check: BindCheck =
        BindCheck::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("bind_check");
    let _check_result = DeviceManage::new().bind_check(&bind_check.file_path);
    let response_msg = EmptyResponse {};
    encode_message(response_msg)
}

pub fn device_bind_acquire(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let bind_acquire: BindAcquire =
        BindAcquire::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("bind_acquire");
    let bind_result = DeviceManage::new().bind_acquire(&bind_acquire.bind_code);
    let response_msg = BindAcquireResponse {
        bind_result: bind_result,
    };
    encode_message(response_msg)
}

pub fn device_display_bind_code(param: &DeviceParam) -> Result<Vec<u8>, Error> {
    let _bind_display: BindDisplay =
        BindDisplay::decode(&param.param.as_ref().expect("device_param").value.clone())
            .expect("bind_display_code");
    let display_result = display_bind_code(); //no param
    let response_msg = BindDisplayResponse {
        bind_display_result: display_result,
    };
    encode_message(response_msg)
}
