use common::constants;
use common::https;
use serde::{Deserialize, Serialize};
use crate::Result;
use crate::error::ImkeyError;

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
            commandID: String::from(constants::TSM_ACTION_SE_QUERY),
            cardRetDataList: None,
        }
    }

    pub fn se_query(&mut self) -> Result<service_response> {
        println!("请求报文：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let mut response_data = https::post(constants::TSM_ACTION_SE_QUERY, req_data)?;
        let return_bean: service_response = serde_json::from_str(response_data.as_str())?;
        println!("反馈报文：{:#?}", return_bean);

        match return_bean._ReturnCode.as_str() {
            constants::TSM_RETURN_CODE_SUCCESS => Ok(return_bean),
            constants::TSM_RETURNCODE_DEVICE_ILLEGAL => Err(ImkeyError::IMKEY_TSM_DEVICE_ILLEGAL.into()),
            constants::TSM_RETURNCODE_DEVICE_STOP_USING => Err(ImkeyError::IMKEY_TSM_DEVICE_STOP_USING.into()),
            constants::TSM_RETURNCODE_SE_QUERY_FAIL => Err(ImkeyError::IMKEY_TSM_DEVICE_UPDATE_CHECK_FAIL.into()),
            _ => Err(ImkeyError::IMKEY_TSM_SERVER_ERROR.into()),
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
