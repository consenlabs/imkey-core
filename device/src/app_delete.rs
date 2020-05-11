use crate::error::ImkeyError;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::constants;
use common::https;
use mq::message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDeleteRequest {
    pub seid: String,
    pub instance_aid: String,
    pub device_cert: String,
    pub step_key: String,
    pub status_word: Option<String>,
    #[serde(rename = "commandID")]
    pub command_id: String,
    pub card_ret_data_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDeleteResponse {
    pub seid: Option<String>,
    pub instance_aid: Option<String>,
    pub next_step_key: Option<String>,
    pub apdu_list: Option<Vec<String>>,
}

impl TsmService for AppDeleteRequest {
    type ReturnData = ();

    fn send_message(&mut self) -> Result<()> {
        loop {
            println!("send message：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let response_data = https::post(constants::TSM_ACTION_APP_DELETE, req_data)?;
            let return_bean: ServiceResponse<AppDeleteResponse> =
                serde_json::from_str(response_data.as_str())?;
            println!("return message：{:#?}", return_bean);
            if return_bean._ReturnCode == constants::TSM_RETURN_CODE_SUCCESS {
                //check if end
                let next_step_key = return_bean._ReturnData.next_step_key.unwrap();
                if constants::TSM_END_FLAG.eq(next_step_key.as_str()) {
                    return Ok(());
                }
                let mut apdu_res: Vec<String> = Vec::new();
                match return_bean._ReturnData.apdu_list {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //sende apdu command
                            let res = message::send_apdu(apdu_val.to_string())?;
                            apdu_res.push(res.clone());
                            if index_val == apdu_list.len() - 1 {
                                self.status_word = Some(String::from(&res[res.len() - 4..]));
                            }
                        }
                        self.card_ret_data_list = Some(apdu_res);
                        self.step_key = next_step_key;
                    }
                    None => (),
                }
            } else {
                let ret_code_check_result: Result<()> = match return_bean._ReturnCode.as_str() {
                    constants::TSM_RETURNCODE_APP_DELETE_FAIL => {
                        Err(ImkeyError::ImkeyTsmAppDeleteFail.into())
                    }
                    constants::TSM_RETURNCODE_DEVICE_ILLEGAL => {
                        Err(ImkeyError::ImkeyTsmDeviceIllegal.into())
                    }
                    constants::TSM_RETURNCODE_OCE_CERT_CHECK_FAIL => {
                        Err(ImkeyError::ImkeyTsmOceCertCheckFail.into())
                    }
                    constants::TSM_RETURNCODE_DEVICE_STOP_USING => {
                        Err(ImkeyError::ImkeyTsmDeviceStopUsing.into())
                    }
                    constants::TSM_RETURNCODE_RECEIPT_CHECK_FAIL => {
                        Err(ImkeyError::ImkeyTsmReceiptCheckFail.into())
                    }
                    constants::TSM_RETURNCODE_DEV_INACTIVATED => {
                        Err(ImkeyError::ImkeyTsmDeviceNotActivated.into())
                    }
                    _ => Err(ImkeyError::ImkeyTsmServerError.into()),
                };
                return ret_code_check_result;
            }
        }
    }
}

impl AppDeleteRequest {
    pub fn build_request_data(seid: String, instance_aid: String, device_cert: String) -> Self {
        AppDeleteRequest {
            seid: seid,
            instance_aid: instance_aid,
            device_cert: device_cert,
            step_key: String::from("01"),
            status_word: None,
            command_id: String::from(constants::TSM_ACTION_APP_DELETE),
            card_ret_data_list: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::app_delete::AppDeleteRequest;
    use crate::manager::{get_cert, get_se_id};
    use crate::TsmService;
    use mq::hid_api::hid_connect;

    #[test]
    pub fn app_delete_test() {
        hid_connect("imKey Pro");
        let seid = get_se_id().unwrap();
        let device_cert = get_cert().unwrap();
        let instance_aid = "695F627463".to_string();
        AppDeleteRequest::build_request_data(seid, instance_aid, device_cert).send_message();
    }
}
