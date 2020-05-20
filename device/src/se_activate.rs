use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::constants;
use common::https;
use mq::message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeActivateRequest {
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
pub struct SeActivateResponse {
    pub seid: Option<String>,
    pub next_step_key: Option<String>,
    pub apdu_list: Option<Vec<String>>,
}

impl TsmService for SeActivateRequest {
    type ReturnData = ();

    fn send_message(&mut self) -> Result<()> {
        loop {
            println!("send message：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let response_data = https::post(constants::TSM_ACTION_SE_ACTIVATE, req_data)?;
            let return_bean: ServiceResponse<SeActivateResponse> =
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
                            //send apdu command
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
                return_bean.service_res_check()?;
            }
        }
    }
}

impl SeActivateRequest {
    pub fn build_request_data(seid: String, sn: String, device_cert: String) -> Self {
        SeActivateRequest {
            seid: seid,
            sn: sn,
            device_cert: device_cert,
            step_key: String::from("01"),
            status_word: None,
            command_id: String::from(constants::TSM_ACTION_SE_ACTIVATE),
            card_ret_data_list: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::manager::{get_cert, get_se_id, get_sn};
    use crate::se_activate::SeActivateRequest;
    use crate::TsmService;
    use mq::hid_api::hid_connect;

    #[test]
    pub fn se_activate_test() {
        hid_connect("imKey Pro");
        let seid = get_se_id().unwrap();
        let device_cert = get_cert().unwrap();
        let sn = get_sn().unwrap();
        SeActivateRequest::build_request_data(seid, sn, device_cert).send_message();
    }
}
