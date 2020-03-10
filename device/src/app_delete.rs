extern crate reqwest;
use common::constants::{TSM_ACTION_APP_DELETE, TSM_RETURN_CODE_SUCCESS};
use common::{error::ImkeyError, https};
use mq::message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Result;
use common::error::BAPP0011;

#[derive(Debug, Serialize, Deserialize)]
pub struct app_delete_request {
    pub seid: String,
    pub instanceAid: String,
    pub deviceCert: String,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct service_response {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: app_delete_response,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct app_delete_response {
    pub seid: Option<String>,
    pub instanceAid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl app_delete_request {
    pub fn build_request_data(
        seid: String,
        instance_aid: String,
        device_cert: String,
    ) -> app_delete_request {
        app_delete_request {
            seid: seid,
            instanceAid: instance_aid,
            deviceCert: device_cert,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(TSM_ACTION_APP_DELETE),
            cardRetDataList: None,
        }
    }

    pub fn app_delete(&mut self) -> Result<()> {
        loop {
            println!("请求报文：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let mut response_data = https::post(TSM_ACTION_APP_DELETE, req_data);
            let return_bean: service_response =
                serde_json::from_str(response_data.ok().unwrap().as_str())
                    .expect("imkey message seriailize error");
            println!("反馈报文：{:#?}", return_bean);
            if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
                //判断步骤key是否已经结束
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if "end".eq(next_step_key.as_str()) {
                    println!("应用删除成功结束");
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = Vec::new();

                match return_bean._ReturnData.apduList {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //调用发送指令接口，并获取执行结果
                            println!("download apdu --> {}", apdu_val);
                            let res = message::send_apdu(apdu_val.to_string());

                            apdu_res.push(String::from(&res));
                            if index_val == apdu_list.len() - 1 {
                                let status: String =
                                    res.chars().skip(res.len() - 4).take(4).collect();
                                self.statusWord = Some(String::from(status));
                            }
                        }
                        self.cardRetDataList = Some(apdu_res);
                        self.stepKey = next_step_key;
                    }
                    None => (),
                }
            } else {
                println!("应用删除服务器执行失败并返回 : {}", return_bean._ReturnMsg);
//                return Err(ImkeyError::BAPP0011);
                return Err(format_err!("imkey_tsm_app_delete_fail"));
            }
        }
    }
}
