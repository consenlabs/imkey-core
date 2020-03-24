use common::constants::{TSM_ACTION_AUTHCODE_STORAGE, TSM_RETURN_CODE_SUCCESS};
use common::https;
use serde::{Deserialize, Serialize};
use crate::Result;
use crate::error::ImkeyError;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCodeStorageRequest {
    pub seid: String,
    pub authCode: String,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResponse {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: AuthCodeStorageResponse,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthCodeStorageResponse {
    pub seid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl AuthCodeStorageRequest {
    pub fn build_request_data(seid: String, auth_code: String) -> AuthCodeStorageRequest {
        AuthCodeStorageRequest {
            seid: seid,
            authCode: auth_code,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(TSM_ACTION_AUTHCODE_STORAGE),
            cardRetDataList: None,
        }
    }

    pub fn auth_code_storage(&mut self) -> Result<()> {
        println!("请求报文：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let response_data = https::post(TSM_ACTION_AUTHCODE_STORAGE, req_data)?;
        let return_bean: ServiceResponse = serde_json::from_str(response_data.as_str())?;
        println!("反馈报文：{:#?}", return_bean);
        if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
            return Ok(());
        } else {
            return Err(ImkeyError::BSE0021.into());
        }
    }
}

#[cfg(test)]
mod test{
    use crate::auth_code_storage::AuthCodeStorageRequest;

    #[test]
    pub fn auth_code_storage_test(){
        let seid: String = "19060000000200860001010000000014".to_string();
        let auth_code: String = "33FE1AEAEB429C2C3798EBE67B709DBB7140FFD6FA9E8D5FF5476919B618904F88C3B52D8D05FAE7F88E9FAEC576A1EAFAF4D48B7B6670CBB368A34D6FD67F93F6BC008746BDB349A020FE88FBF566CC6ADE322E6B17325DD9AEF310495BECCE4634B61621FE776F4D217B8764CAC0FFD276FD4917CEA3EBECCF19D2EB7237DB395B9B68BA40FC6B040C257658E8D6D0DE9A67E49DF86D065BE1831B02751BBA00F8EDE4242859221C1D7DEF7547D1ADB43188D2A074AAA0C980A822B8B5363D8CCE81E829F83DA6E8D7530AD6F2785F07E417AE59AD2A414D5A203D3052F0106AC801C054374E6F4ADD024C9026E3D4DA9F947B0097F4E10B20E1B684AA6006".to_string();
        AuthCodeStorageRequest::build_request_data(seid, auth_code).auth_code_storage();
    }
}
