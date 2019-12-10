use common::constants::{TSM_ACTION_APP_UPDATE, TSM_RETURN_CODE_SUCCESS};
use common::{error::ImkeyError, https};
use serde::{Deserialize, Serialize};
use mq::message;

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

    pub fn app_update(&mut self) -> Result<(), ImkeyError> {
        loop {
            println!("请求报文：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let mut response_data = https::post(TSM_ACTION_APP_UPDATE, req_data);
            let return_bean: service_response =
                serde_json::from_str(response_data.ok().unwrap().as_str())
                    .expect("imkey message seriailize error");
            println!("反馈报文：{:#?}", return_bean);
            if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
                //判断步骤key是否已经结束
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if "end".eq(next_step_key.as_str()) {
                    println!("应用更新成功结束");
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = Vec::new();

                match return_bean._ReturnData.apduList {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //调用发送指令接口，并获取执行结果
                            println!("download apdu --> {}", apdu_val);
                            let res = message::send_apdu(apdu_val.to_string());

                            let status_word = "9000";

                            if "02".eq(next_step_key.as_str()) {
//                                apdu_res.push(String::from("9000"));
//                                apdu_res.push(String::from("5F49410465330B2F12ADEC9D6C61CA1768704261D02E5F39177762D5C457F0FDA4ABC87882ADD11C951941C003269874103F5C83269C3CF7A61231D2C746F4AE543D382F86100C1402F7FC4E1C3C1BD35674431261289000"));
                                apdu_res.push(String::from(&res));
                            } else {
                                apdu_res.push(String::from(status_word));
                            }

                            //如果指令执行失败，则停止执行并返回
                            if "03".eq(next_step_key.as_str())
                                && index_val > 0
                                && !"9000".eq(status_word)
                            {
                                println!("更新指令执行失败");
                                break;
                            } else if index_val == apdu_list.len() - 1 {
                                let status:String = res.chars().skip(res.len()-4).take(4).collect();
                                self.statusWord = Some(String::from(status));
                            }
                        }
                        self.cardRetDataList = Some(apdu_res);
                        self.stepKey = next_step_key;
                    }
                    None => (),
                }
            } else {
                println!("应用更新服务器执行失败并返回 : {}", return_bean._ReturnMsg);
                return Err(ImkeyError::BAPP0008);
            }
        }
    }
}
