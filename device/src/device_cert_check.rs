use crate::error::ImkeyError;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::constants;
use common::https;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCertCheckRequest {
    pub seid: String,
    pub sn: String,
    pub device_cert: String,
    pub step_key: String,
    pub status_word: Option<String>,
    #[serde(rename = "commandID")]
    pub command_id: String,
    pub card_ret_data_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCertCheckResponse {
    pub seid: Option<String>,
    pub verify_result: Option<bool>,
    pub next_step_key: Option<String>,
    pub apdu_list: Option<Vec<String>>,
}

impl TsmService for DeviceCertCheckRequest {
    type ReturnData = ();

    fn send_message(&mut self) -> Result<()> {
        println!("send message：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let response_data = https::post(constants::TSM_ACTION_DEVICE_CERT_CHECK, req_data)?;
        let return_bean: ServiceResponse<DeviceCertCheckResponse> =
            serde_json::from_str(response_data.as_str())?;
        println!("return message：{:#?}", return_bean);

        match return_bean.service_res_check() {
            Ok(()) => {
                if return_bean._ReturnData.verify_result.unwrap() {
                    return Ok(());
                }
                Err(ImkeyError::ImkeySeCertInvalid.into())
            }
            Err(e) => Err(e),
        }
    }
}

impl DeviceCertCheckRequest {
    pub fn build_request_data(seid: String, sn: String, device_cert: String) -> Self {
        DeviceCertCheckRequest {
            seid: seid,
            sn: sn,
            device_cert: device_cert,
            step_key: String::from("01"),
            status_word: None,
            command_id: String::from(constants::TSM_ACTION_DEVICE_CERT_CHECK),
            card_ret_data_list: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::device_cert_check::DeviceCertCheckRequest;
    use crate::device_manager::{get_cert, get_se_id, get_sn};
    use crate::TsmService;
    use transport::hid_api::hid_connect;

    #[test]
    pub fn device_cert_check_test() {
        assert!(hid_connect("imKey Pro").is_ok());
        let seid = get_se_id().unwrap();
        let device_cert = get_cert().unwrap();
        let sn = get_sn().unwrap();
        assert!(
            DeviceCertCheckRequest::build_request_data(seid, sn, device_cert)
                .send_message()
                .is_ok()
        );
    }
}
