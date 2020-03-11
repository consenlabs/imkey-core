use common::constants::{TSM_ACTION_DEVICE_CERT_CHECK, TSM_RETURN_CODE_SUCCESS};
use common::https;
use mq::message;
use mq::message::send_apdu;
use serde::{Deserialize, Serialize};
use crate::Result;
use crate::error::ImkeyError;

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
    pub verifyResult: Option<bool>,
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

    pub fn device_cert_check(&mut self) -> Result<service_response> {
        println!("请求报文：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let mut response_data = https::post(TSM_ACTION_DEVICE_CERT_CHECK, req_data)?;
        let return_bean: service_response = serde_json::from_str(response_data.as_str())?;
        println!("返回报文：{:#?}", return_bean);
        if return_bean._ReturnCode.is_empty(){
            return Err(ImkeyError::BSE0009.into());
        }
        Ok(return_bean)
    }
}

#[cfg(test)]
mod test{
    use crate::device_cert_check::device_cert_check_request;

    #[test]
    pub fn device_cert_check_test(){
        let seid: String = "19060000000200860001010000000014".to_string();
        let sn:String = "imKey01191200001".to_string();
        let device_cert: String = "BF2181CA7F2181C6931019060000000200860001010000000014420200015F200401020304950200805F2504201810145F2404FFFFFFFF53007F4947B04104FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44F002DFFE5F374830460221008CB58D54BDED501236621B83B320081E6F9B6B5539AE5EC9D36B660EC445A5E8022100A203CA1F9ABEE69751EA402A2ACDFD6B4A87697D6CD721F60540959095EC9466".to_string();
        device_cert_check_request::build_request_data(seid, sn, device_cert).device_cert_check();
    }
}
