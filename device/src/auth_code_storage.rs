use common::constants::{TSM_ACTION_AUTHCODE_STORAGE, TSM_RETURN_CODE_SUCCESS};
use common::{error::ImkeyError, https};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct auth_code_storage_request {
    pub seid: String,
    pub authCode: String,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct service_response {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: auth_code_storage_response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct auth_code_storage_response {
    pub seid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl auth_code_storage_request {
    pub fn build_request_data(
        seid: String,
        auth_code: String,
    ) -> auth_code_storage_request {
        auth_code_storage_request {
            seid: seid,
            authCode: auth_code,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(TSM_ACTION_AUTHCODE_STORAGE),
            cardRetDataList: None,
        }
    }

    pub fn auth_code_storage(&mut self) -> Result<service_response, ImkeyError> {
        println!("请求报文：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let mut response_data = https::post(TSM_ACTION_AUTHCODE_STORAGE, req_data);
        let return_bean: service_response =
            serde_json::from_str(response_data.ok().unwrap().as_str())
                .expect("imkey message seriailize error");
        println!("反馈报文：{:#?}", return_bean);
        if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
            //判断步骤key是否已经结束
            //            let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
            //            if "end".eq(next_step_key.as_str()) {
            println!("验证码存储功能执行完成");
            return Ok(return_bean);
        //            }
        } else {
            println!("验证码存储服务器执行失败并返回 : {}", return_bean._ReturnMsg);
            return Err(ImkeyError::BSE0010);
        }
    }
}
