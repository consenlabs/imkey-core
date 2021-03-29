use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::constants;
use common::https;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateRequest {
    pub seid: String,
    pub instance_aid: String,
    pub device_cert: String,
    pub sdk_version: Option<String>,
    pub step_key: String,
    pub status_word: Option<String>,
    #[serde(rename = "commandID")]
    pub command_id: String,
    pub card_ret_data_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateResponse {
    pub seid: Option<String>,
    pub instance_aid: Option<String>,
    pub next_step_key: Option<String>,
    pub apdu_list: Option<Vec<String>>,
    pub address_register_list: Option<Vec<String>>,
}

impl TsmService for AppUpdateRequest {
    type ReturnData = ServiceResponse<AppUpdateResponse>;

    fn send_message(&mut self) -> Result<ServiceResponse<AppUpdateResponse>> {
        loop {
            // println!("send message：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let response_data = https::post(constants::TSM_ACTION_APP_UPDATE, req_data)?;
            let return_bean: ServiceResponse<AppUpdateResponse> =
                serde_json::from_str(response_data.as_str())?;
            // println!("return message：{:#?}", return_bean);
            if return_bean._ReturnCode == constants::TSM_RETURN_CODE_SUCCESS {
                //check if end
                let next_step_key = return_bean.clone()._ReturnData.next_step_key.unwrap();
                if constants::TSM_END_FLAG.eq(next_step_key.as_str()) {
                    return Ok(return_bean);
                }

                match return_bean._ReturnData.apdu_list {
                    Some(apdu_list) => {
                        let handle_result =
                            ServiceResponse::<AppUpdateResponse>::apdu_handle(apdu_list)?;
                        self.card_ret_data_list = Some(handle_result.0);
                        self.status_word = Some(handle_result.1);
                        self.step_key = next_step_key;
                    }
                    None => (),
                }
            } else {
                return_bean.service_res_check()?;
            }
        }
    }
}

impl AppUpdateRequest {
    pub fn build_request_data(
        seid: String,
        instance_aid: String,
        device_cert: String,
        sdk_version: Option<String>,
    ) -> Self {
        AppUpdateRequest {
            seid: seid,
            instance_aid: instance_aid,
            device_cert: device_cert,
            sdk_version: sdk_version,
            step_key: String::from("01"),
            status_word: None,
            command_id: String::from(constants::TSM_ACTION_APP_UPDATE),
            card_ret_data_list: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::app_update::AppUpdateRequest;
    use crate::device_manager::{get_cert, get_se_id};
    use crate::TsmService;
    use transport::hid_api::hid_connect;

    #[test]
    pub fn app_update_test() {
        assert!(hid_connect("imKey Pro").is_ok());
        let seid = get_se_id().unwrap();
        let device_cert = get_cert().unwrap();
        let instance_aid = "695F626368".to_string();
        let exe_result =
            AppUpdateRequest::build_request_data(seid, instance_aid, device_cert, None)
                .send_message();
        assert!(exe_result.is_ok());
    }

    #[test]
    pub fn app_update_error_test() {
        let seid = "00000000000000000000000000000000".to_string();
        let device_cert = "00000000000000000000000000".to_string();
        let instance_aid = "695F627463".to_string();
        assert!(
            AppUpdateRequest::build_request_data(seid, instance_aid, device_cert, None)
                .send_message()
                .is_err()
        );
    }
}
