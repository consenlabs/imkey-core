use crate::error::ImkeyError;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::constants;
use common::https;
use mq::message;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AppDownloadRequest {
    pub seid: String,
    pub instanceAid: String,
    pub deviceCert: String,
    pub sdkVersion: Option<String>,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AppDownloadResponse {
    pub seid: Option<String>,
    pub instanceAid: Option<String>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl TsmService for AppDownloadRequest {
    type ReturnData = ();

    fn send_message(&mut self) -> Result<()> {
        loop {
            println!("send message：{:#?}", self);
            let req_data = serde_json::to_vec_pretty(&self).unwrap();
            let response_data = https::post(constants::TSM_ACTION_APP_DOWNLOAD, req_data)?;
            let return_bean: ServiceResponse<AppDownloadResponse> =
                serde_json::from_str(response_data.as_str())?;
            println!("return message：{:#?}", return_bean);
            if return_bean._ReturnCode == constants::TSM_RETURN_CODE_SUCCESS {
                //check step key is end
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if constants::TSM_END_FLAG.eq(next_step_key.as_str()) {
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = Vec::new();
                match return_bean._ReturnData.apduList {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //send apdu
                            let res = message::send_apdu(apdu_val.to_string())?;
                            apdu_res.push(res.clone());
                            if index_val == apdu_list.len() - 1 {
                                self.statusWord = Some(String::from(&res[res.len() - 4..]));
                            }
                        }
                        self.cardRetDataList = Some(apdu_res);
                        self.stepKey = next_step_key;
                    }
                    None => (),
                }
            } else {
                let ret_code_check_result: Result<()> = match return_bean._ReturnCode.as_str() {
                    constants::TSM_RETURNCODE_APP_DOWNLOAD_FAIL => {
                        Err(ImkeyError::ImkeyTsmAppDownloadFail.into())
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

impl AppDownloadRequest {
    pub fn build_request_data(
        seid: String,
        instance_aid: String,
        device_cert: String,
        sdk_version: Option<String>,
    ) -> Self {
        AppDownloadRequest {
            seid: seid,
            instanceAid: instance_aid,
            deviceCert: device_cert,
            sdkVersion: sdk_version,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(constants::TSM_ACTION_APP_DOWNLOAD),
            cardRetDataList: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::app_download::AppDownloadRequest;
    use crate::manager::{get_cert, get_se_id};
    use crate::TsmService;
    use mq::hid_api::hid_connect;

    #[test]
    pub fn app_download_test() {
        hid_connect("imKey Pro");
        let seid = get_se_id().unwrap();
        let device_cert = get_cert().unwrap();
        let instance_aid = "695F627463".to_string();
        AppDownloadRequest::build_request_data(seid, instance_aid, device_cert, None)
            .send_message();
    }
}
