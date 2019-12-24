use crate::api::DeviceParam;
use crate::deviceapi::AppAction;
use crate::deviceapi::BindCode;
use crate::deviceapi::DeviceCert;
use crate::deviceapi::DeviceName;
use crate::deviceapi::{AuthCode, AuthCodeResponse, AuthCodeServiceResponse};
use crate::wallet_handler::encode_message;
use common::error::Error;
use device::auth_code_storage::auth_code_storage_request;
use device::manager;
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
