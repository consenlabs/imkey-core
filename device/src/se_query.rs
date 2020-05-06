use common::constants;
use common::https;
use serde::{Deserialize, Serialize};
use crate::{Result, TsmService};
use crate::error::ImkeyError;
use crate::ServiceResponse;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SeQueryRequest {
    pub seid: String,
    pub sn: String,
    pub sdkVersion: Option<String>,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SeQueryResponse {
    pub seid: Option<String>,
    pub nextStepKey: Option<String>,
    pub sn: Option<String>,
    pub sdkMode: Option<String>,
    pub availableAppBeanList: Option<Vec<AvailableAppBean>>,
    pub status: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AvailableAppBean {
    pub appLogo: Option<String>,
    pub installMode: Option<String>,
    pub installedVersion: Option<String>,
    pub instanceAid: Option<String>,
    pub lastUpdated: Option<String>,
    pub latestVersion: Option<String>,
}

impl TsmService for SeQueryRequest {
    type ReturnData = ServiceResponse<SeQueryResponse>;

    fn send_message(&mut self) -> Result<ServiceResponse<SeQueryResponse>> {
        println!("send message：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let response_data = https::post(constants::TSM_ACTION_SE_QUERY, req_data)?;
        let mut return_bean: ServiceResponse<SeQueryResponse> = serde_json::from_str(response_data.as_str())?;
        println!("return message：{:#?}", return_bean);

        match return_bean._ReturnCode.as_str() {
            constants::TSM_RETURN_CODE_SUCCESS => {
                return_bean._ReturnData.status = Some(constants::IMKEY_DEV_STATUS_LATEST.to_string());
                Ok(return_bean)
            }
            constants::TSM_RETURNCODE_DEVICE_ILLEGAL => Err(ImkeyError::ImkeyTsmDeviceIllegal.into()),
            constants::TSM_RETURNCODE_DEVICE_STOP_USING => Err(ImkeyError::ImkeyTsmDeviceStopUsing.into()),
            constants::TSM_RETURNCODE_SE_QUERY_FAIL => Err(ImkeyError::ImkeyTsmDeviceUpdateCheckFail.into()),
            constants::TSM_RETURNCODE_DEV_INACTIVATED => {
                return_bean._ReturnData.status = Some(constants::IMKEY_DEV_STATUS_INACTIVATED.to_string());
                Ok(return_bean)
            }
            _ => Err(ImkeyError::ImkeyTsmServerError.into()),
        }
    }
}

impl SeQueryRequest {
    pub fn build_request_data(
        seid: String,
        sn: String,
        sdk_version: Option<String>,
    ) -> Self {
        SeQueryRequest {
            seid: seid,
            sn: sn,
            sdkVersion: sdk_version,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(constants::TSM_ACTION_SE_QUERY),
            cardRetDataList: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::se_query::SeQueryRequest;
    use crate::TsmService;

    #[test]
    fn se_query_test() {
        let seid: String = "19060000000200860001010000000014".to_string();
        let sn: String = "imKey01191200001".to_string();
        SeQueryRequest::build_request_data(seid, sn, None).send_message();
    }
}
