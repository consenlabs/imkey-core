use common::constants::{TSM_ACTION_DEVICE_CERT_CHECK, TSM_RETURN_CODE_SUCCESS};
use common::{error::ImkeyError, https};
use mq::message;
use mq::message::send_apdu;
use serde::{Deserialize, Serialize};

// SE安全检查请求bean
#[derive(Debug, Serialize, Deserialize)]
pub struct device_cert_check_request {
    pub seid: String,
    pub sn: String,
    pub deviceCert: String,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

//SE安全检查接口
#[derive(Serialize, Deserialize, Debug)]
pub struct service_response {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: device_cert_check_response,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct device_cert_check_response {
    pub seid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl device_cert_check_request {
    pub fn build_request_data(
        seid: String,
        sn: String,
        device_cert: String,
    ) -> device_cert_check_request {
        device_cert_check_request {
            seid: seid,
            sn: sn,
            deviceCert: device_cert,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(TSM_ACTION_DEVICE_CERT_CHECK),
            cardRetDataList: None,
        }
    }

    pub fn device_cert_check(&mut self) -> Result<(), ImkeyError> {
        println!("请求报文：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let mut response_data = https::post(TSM_ACTION_DEVICE_CERT_CHECK, req_data);
        let return_bean: service_response =
            serde_json::from_str(response_data.ok().unwrap().as_str())
                .expect("imkey message seriailize error");
        println!("返回报文：{:#?}", return_bean);
        if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
            return Ok(());
        } else {
            println!(
                "SE安全检查服务器执行失败并返回 : {}",
                return_bean._ReturnMsg
            );
            return Err(ImkeyError::BSE0009);
        }
    }
}
