use common::constants::{TSM_ACTION_APP_UPDATE, TSM_RETURN_CODE_SUCCESS, TSM_END_FLAG};
use common::https;
use mq::message;
use serde::{Deserialize, Serialize};
use crate::Result;
use crate::error::ImkeyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct app_update_request {
    pub seid: String,
    pub instanceAid: String,
    pub deviceCert: String,
    pub sdkVersion: Option<String>,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct service_response {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: app_update_response,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct app_update_response {
    pub seid: Option<String>,
    pub instanceAid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl app_update_request {
    pub fn build_request_data(
        seid: String,
        instance_aid: String,
        device_cert: String,
        sdk_version: Option<String>,
    ) -> app_update_request {
        app_update_request {
            seid: seid,
            instanceAid: instance_aid,
            deviceCert: device_cert,
            sdkVersion: sdk_version,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(TSM_ACTION_APP_UPDATE),
            cardRetDataList: None,
        }
    }

    pub fn app_update(&mut self) -> Result<()> {
        loop {
            println!("请求报文：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let mut response_data = https::post(TSM_ACTION_APP_UPDATE, req_data)?;
            let return_bean: service_response = serde_json::from_str(response_data.as_str())?;
            println!("反馈报文：{:#?}", return_bean);
            if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
                //判断步骤key是否已经结束
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if TSM_END_FLAG.eq(next_step_key.as_str()) {
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = vec![];
                match return_bean._ReturnData.apduList {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //调用发送指令接口，并获取执行结果
                            let res = message::send_apdu(apdu_val.to_string());
                            apdu_res.push(res.clone());
                            if index_val == apdu_list.len() - 1 {
                                self.statusWord = Some(String::from(&res[res.len() -4..]));
                            }
                        }
                        self.cardRetDataList = Some(apdu_res);
                        self.stepKey = next_step_key;
                    }
                    None => (),
                }
            } else {
                return Err(ImkeyError::BAPP0008.into());
            }
        }
    }
}

#[cfg(test)]
mod test{
    use crate::app_update::app_update_request;

    #[test]
    pub fn app_update_test(){
        let seid: String = "19060000000200860001010000000014".to_string();
        let instance_aid: String = "695F657468".to_string();
        let device_cert: String = "BF2181CA7F2181C6931019060000000200860001010000000014420200015F200401020304950200805F2504201810145F2404FFFFFFFF53007F4947B04104FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44F002DFFE5F374830460221008CB58D54BDED501236621B83B320081E6F9B6B5539AE5EC9D36B660EC445A5E8022100A203CA1F9ABEE69751EA402A2ACDFD6B4A87697D6CD721F60540959095EC9466".to_string();
        app_update_request::build_request_data(seid, instance_aid, device_cert, None).app_update();
    }
}