use common::constants::{TSM_ACTION_SE_QUERY, TSM_RETURN_CODE_SUCCESS};
use common::{error::ImkeyError, https};
use serde::{Deserialize, Serialize};

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
    pub sn: Option<String>,
    pub sdkMode: Option<String>,
    pub availableAppBeanList: Option<Vec<available_app_bean>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct available_app_bean {
    pub appLogo: Option<String>,
    pub installMode: Option<String>,
    pub installedVersion: Option<String>,
    pub instanceAid: Option<String>,
    pub lastUpdated: Option<String>,
    pub latestVersion: Option<String>,
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
            commandID: String::from(TSM_ACTION_SE_QUERY),
            cardRetDataList: None,
        }
    }

    pub fn se_query(&mut self) -> Result<service_response, ImkeyError> {
        println!("请求报文：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let mut response_data = https::post(TSM_ACTION_SE_QUERY, req_data);
        let return_bean: service_response =
            serde_json::from_str(response_data.ok().unwrap().as_str().clone())
                .expect("imkey message seriailize error");
        println!("反馈报文：{:#?}", return_bean);

        if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
            //判断步骤key是否已经结束
            //            let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
            //            if "end".eq(next_step_key.as_str()) {
            println!("SE应用查询成功结束");
            return Ok(return_bean);
        //            }
        } else {
            println!("应用查询服务器执行失败并返回 : {}", return_bean._ReturnMsg);
            return Err(ImkeyError::BSE0008);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::se_query::se_query_request;

    #[test]
    fn se_query_test() {
        let seid: String = "19060000000200860001010000000014".to_string();
        let sn: String = "imKey01191200001".to_string();
        se_query_request::build_request_data(seid, sn, None).se_query();
    }
}
