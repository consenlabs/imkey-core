extern crate reqwest;
use std::collections::HashMap;
use reqwest::{Client, Response};
use serde::{Serialize, Deserialize};
use common::http;
use crate::error::ImkeyError;
use common::constants::{TSM_ACTION_SE_ACTIVATE, TSM_RETURN_CODE_SUCCESS};

#[derive(Debug, Serialize, Deserialize)]
pub struct se_activate_request{
    pub seid : String,
    pub sn : String,
    pub deviceCert : String,
    pub stepKey : String,
    pub statusWord : Option<String>,
    pub commandID : String,
    pub cardRetDataList : Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct service_response {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: se_activate_response,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct se_activate_response{
    pub seid : Option<String>,
    pub nextStepKey : Option<String>,
    pub apduList : Option<Vec<String>>,
}

impl se_activate_request{
    pub fn build_request_data(seid : String, sn : String, device_cert : String) -> se_activate_request{
        se_activate_request{
            seid : seid,
            sn : sn,
            deviceCert : device_cert,
            stepKey : String::from("01"),
            statusWord : None,
            commandID : String::from(TSM_ACTION_SE_ACTIVATE),
            cardRetDataList : None,
        }
    }

    pub fn se_activate(&mut self) -> Result<(), ImkeyError>{

        loop {
            println!("请求报文：{:#?}", self);
            let mut response_data : Response = http::post(TSM_ACTION_SE_ACTIVATE, &self);
            let return_bean: service_response = response_data.json().expect("imkey message seriailize error");
            println!("反馈报文：{:#?}", return_bean);
            if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS{
                //判断步骤key是否已经结束
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if "end".eq(next_step_key.as_str()) {
                    println!("SE激活成功结束");
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = Vec::new();

                match return_bean._ReturnData.apduList{
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate(){
                            //调用发送指令接口，并获取执行结果
                            println!("apdu --> {}", apdu_val);
                            let status_word = "9000";


                            if "02".eq(next_step_key.as_str()) {
                                apdu_res.push(String::from("9000"));
                                apdu_res.push(String::from("5F49410465330B2F12ADEC9D6C61CA1768704261D02E5F39177762D5C457F0FDA4ABC87882ADD11C951941C003269874103F5C83269C3CF7A61231D2C746F4AE543D382F86100C1402F7FC4E1C3C1BD35674431261289000"));
                            }else{
                                apdu_res.push(String::from(status_word));
                            }

                            //把最后一条指令结果上送给服务器
                            if index_val == apdu_list.len() - 1 {
                                self.statusWord = Some(String::from(status_word));
                            }
                        }
                        self.cardRetDataList = Some(apdu_res);
                        self.stepKey = next_step_key;
                    },
                    None => (),
                }
            }else{
                println!("SE激活服务器执行失败并返回 : {}", return_bean._ReturnMsg);
                return Err(ImkeyError::BSE0015);
            }
        }
    }
}