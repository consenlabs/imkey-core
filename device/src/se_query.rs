extern crate reqwest;

use common::http;
use reqwest::{Client, Response, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct se_query_request {
    pub seid: String,
    pub sn: String,
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
    pub _ReturnData: se_query_response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct se_query_response {
    pub seid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl se_query_request {
    pub fn build_request_data(
        seid: String,
        sn: String,
        sdk_version: Option<String>,
    ) -> se_query_request {
        se_query_request {
            seid: seid,
            sn: sn,
            sdkVersion: sdk_version,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from("seInfoQuery"),
            cardRetDataList: None,
        }
    }

    pub fn se_query(&mut self) {
        let mut response_data: Response = http::post("seInfoQuery", &self);
        let return_bean: service_response = response_data.json().unwrap();
        if return_bean._ReturnCode == "000000" {
            //判断步骤key是否已经结束
            let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
            if "end".eq(next_step_key.as_str()) {
                println!("SE应用查询成功结束");
            }
        } else {
            println!("应用查询服务器执行失败并返回 : {}", return_bean._ReturnMsg);
        }
    }
}

////http请求
//mod constants;
//fn post<T : Serialize>(action: &str, req_data: &T) -> Response{
//   let url: String = constants::URL.to_string() + action;
//    // let mut url = String::from("http://localhost:8080/imkey/");
//    url.push_str(action);
//    let client = reqwest::Client::new();
//    let response: Response = client.post(&*url)
//        .json(&req_data)
//        .send().unwrap();
//    response
//}
