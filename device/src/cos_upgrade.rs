use crate::app_download::AppDownloadRequest;
use crate::device_manager::{get_cert, get_firmware_version, get_se_id, get_sn};
use crate::error::ImkeyError;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::utility::hex_to_bytes;
use common::{constants, https};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use transport::hid_api::hid_connect;
use transport::message::send_apdu;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CosUpgradeRequest {
    pub seid: String,
    pub sn: String,
    pub device_cert: String,
    pub se_cos_version: String,
    pub is_bl_status: bool,
    pub step_key: String,
    pub status_word: Option<String>,
    #[serde(rename = "commandID")]
    pub command_id: String,
    pub card_ret_data_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CosUpgradeResponse {
    pub seid: Option<String>,
    pub cos_version: Option<String>,
    pub instance_aid_list: Option<Vec<String>>,
    pub next_step_key: Option<String>,
    pub apdu_list: Option<Vec<String>>,
}

impl CosUpgradeRequest {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn cos_upgrade(sdk_version: Option<String>) -> Result<()> {
        //read se device cert
        let mut device_cert = get_cert()?;
        //        ApduCheck::checke_response(&device_cert)?; //TODO 在所有manager里的接口中增加check方法

        let mut is_jump = false;
        let seid;
        let sn;
        let mut se_cos_version = String::new();
        let mut is_bl_status = true;
        //read seid and sn number
        if device_cert.starts_with("bf21") || device_cert.starts_with("BF21") {
            seid = get_se_id()?;
            sn = get_sn()?;
            is_bl_status = false;
            //read se cos version
            se_cos_version = get_firmware_version()?;
            se_cos_version = format!(
                "{}.{}.{}",
                se_cos_version[0..1].to_string(),
                se_cos_version[1..2].to_string(),
                se_cos_version[2..].to_string()
            );
        } else if device_cert.starts_with("7f21") || device_cert.starts_with("7F21") {
            seid = device_cert[12..44].to_string();
            sn = "0000000000000000".to_string();
            is_jump = true;
            let mut temp_device_cert = hex_to_bytes("bf2181").unwrap();
            temp_device_cert.push(((device_cert.len()) / 2) as u8);
            temp_device_cert.extend(
                hex_to_bytes(&device_cert[..device_cert.len()])
                    .unwrap()
                    .iter(),
            );
            device_cert = hex::encode_upper(temp_device_cert);
        } else {
            return Err(ImkeyError::ImkeyTsmCosUpgradeFail.into());
        }

        let mut request_data = CosUpgradeRequest {
            seid: seid.clone(),
            sn: sn,
            device_cert: device_cert.clone(),
            se_cos_version: se_cos_version,
            is_bl_status: is_bl_status,
            step_key: if is_jump {
                "03".to_string()
            } else {
                "01".to_string()
            },
            status_word: None,
            command_id: String::from(constants::TSM_ACTION_COS_UPGRADE),
            card_ret_data_list: None,
        };

        loop {
            println!("send message：{:#?}", request_data);
            let req_data = serde_json::to_vec_pretty(&request_data).unwrap();
            let response_data = https::post(constants::TSM_ACTION_COS_UPGRADE, req_data)?;
            let return_bean: ServiceResponse<CosUpgradeResponse> =
                serde_json::from_str(response_data.as_str())?;
            println!("return message：{:#?}", return_bean);
            if return_bean._ReturnCode == constants::TSM_RETURN_CODE_SUCCESS {
                //check if end
                let next_step_key = return_bean._ReturnData.next_step_key.unwrap();
                if constants::TSM_END_FLAG.eq(next_step_key.as_str()) {
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = vec![];
                match return_bean._ReturnData.apdu_list {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //send apdu command and get return data
                            let res = send_apdu(apdu_val.to_string())?;
                            apdu_res.push(res.clone());
                            if index_val == apdu_list.len() - 1 {
                                request_data.status_word =
                                    Some(String::from(&res[res.len() - 4..]));
                                if (constants::APDU_RSP_SUCCESS.eq(&res[res.len() - 4..])
                                    || constants::APDU_RSP_SWITCH_BL_STATUS_SUCCESS
                                        .eq(&res[res.len() - 4..]))
                                    && ("03".eq(next_step_key.as_str())
                                        || "05".eq(next_step_key.as_str()))
                                {
                                    reconnect()?;
                                }
                            }
                        }
                        request_data.card_ret_data_list = Some(apdu_res);
                    }
                    None => (),
                }

                if "06".eq(next_step_key.as_str()) {
                    //applet download
                    match &return_bean._ReturnData.instance_aid_list {
                        Some(aid_list) => {
                            for temp_instance_aid in aid_list.iter() {
                                AppDownloadRequest::build_request_data(
                                    seid.clone(),
                                    temp_instance_aid.clone(),
                                    device_cert.clone(),
                                    sdk_version.clone(),
                                )
                                .send_message()?;
                            }
                        }
                        None => (),
                    };
                }
                request_data.step_key = next_step_key;
            } else {
                return_bean.service_res_check()?;
            }
        }
    }
}

/**
reconnect device
*/
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn reconnect() -> Result<()> {
    thread::sleep(Duration::from_millis(1000));

    for _ in 0..5 {
        if hid_connect(constants::DEVICE_MODEL_NAME).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(1000));
        continue;
    }

    Err(ImkeyError::ImkeyDeviceReconnectFail.into())
}

#[cfg(test)]
mod tests {
    use crate::cos_upgrade::CosUpgradeRequest;
    use crate::TsmService;
    use transport::hid_api::hid_connect;
    use transport::message::send_apdu;
    use std::collections::HashMap;

    #[test]
    fn cos_upgrade_test() {
        hid_connect("imKey Pro");
        send_apdu("00a4040000".to_string());
        match CosUpgradeRequest::cos_upgrade(None) {
            Ok(()) => println!("COS upgrade success!"),
            Err(e) => println!("{}", e),
        };
    }
}
