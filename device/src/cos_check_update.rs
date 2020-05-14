use crate::error::ImkeyError;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::{constants, https};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CosCheckUpdateRequest {
    pub seid: String,
    pub cos_version: String,
    #[serde(rename = "commandID")]
    pub command_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CosCheckUpdateResponse {
    pub seid: String,
    pub is_latest: bool,
    pub latest_cos_version: Option<String>,
    pub update_type: Option<String>,
    pub description: Option<String>,
}

impl TsmService for CosCheckUpdateRequest {
    type ReturnData = ServiceResponse<CosCheckUpdateResponse>;

    fn send_message(&mut self) -> Result<ServiceResponse<CosCheckUpdateResponse>> {
        println!("send message：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let response_data = https::post(constants::TSM_ACTION_COS_CHECK_UPDATE, req_data)?;
        let return_bean: ServiceResponse<CosCheckUpdateResponse> =
            serde_json::from_str(response_data.as_str())?;
        println!("return message：{:#?}", return_bean);
        match return_bean._ReturnCode.as_str() {
            constants::TSM_RETURN_CODE_SUCCESS => Ok(return_bean),
            constants::TSM_RETURNCODE_DEVICE_ILLEGAL => {
                Err(ImkeyError::ImkeyTsmDeviceIllegal.into())
            }
            constants::TSM_RETURNCODE_DEVICE_STOP_USING => {
                Err(ImkeyError::ImkeyTsmDeviceStopUsing.into())
            }
            constants::TSM_RETURNCODE_COS_CHECK_UPDATE_FAIL => {
                Err(ImkeyError::ImkeyTsmCosCheckUpdateFail.into())
            }
            _ => Err(ImkeyError::ImkeyTsmServerError.into()),
        }
    }
}

impl CosCheckUpdateRequest {
    pub fn build_request_data(seid: String, cos_version: String) -> Self {
        CosCheckUpdateRequest {
            seid: seid,
            cos_version: cos_version,
            command_id: String::from(constants::TSM_ACTION_COS_CHECK_UPDATE),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cos_check_update::CosCheckUpdateRequest;
    use crate::TsmService;

    #[test]
    pub fn cos_check_update_test() {
        let seid: String = "18080000000000860001010000000106".to_string();
        let sn: String = "imKey01200200010".to_string();
        let cos_version: String = "1.0.10".to_string();
        CosCheckUpdateRequest::build_request_data(seid, cos_version).send_message();
    }
}
