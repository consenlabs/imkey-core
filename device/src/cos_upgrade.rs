use common::constants::{TSM_ACTION_COS_UPGRADE, TSM_RETURN_CODE_SUCCESS, TSM_END_FLAG};
use common::{https, constants};
use serde::{Deserialize, Serialize};
use mq::message::{send_apdu};
use crate::manager::{get_se_id, get_sn, get_firmware_version, get_cert};
use common::utility::hex_to_bytes;
use crate::app_download::app_download_request;
use std::sync::Mutex;
#[cfg(target_os = "macos")]
use mq::hid_api;
#[cfg(target_os = "macos")]
use hidapi::{HidApi, HidDevice};
use lazy_static;
#[cfg(target_os = "macos")]
use mq::hid_api::{hid_connect, hid_send};
use crate::Result;
use crate::error::ImkeyError;
use common::apdu::ApduCheck;
use mq::message::DEVICE;


#[derive(Debug, Serialize, Deserialize)]
pub struct cos_upgrade_request {
    pub seid: String,
    pub sn: String,
    pub deviceCert: String,
    pub seCosVersion: String,
    pub isBLStatus: bool,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct service_response {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: cos_upgrade_response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cos_upgrade_response {
    pub seid: Option<String>,
    pub CosVersion: Option<String>,
    pub InstanceAidList: Option<Vec<String>>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl cos_upgrade_request {
    pub fn cos_upgrade(sdk_version: Option<String>) -> Result<()> {
        //read se device cert
        let mut device_cert = get_cert();
//        ApduCheck::checke_response(&device_cert)?; //TODO 在所有manager里的接口中增加check方法

        let mut is_jump = false;
        let mut seid = String::new();
        let mut sn = String::new();
        let mut se_cos_version = String::new();
        let mut is_bl_status = true;
        //read seid and sn number
        if device_cert.starts_with("bf21") || device_cert.starts_with("BF21") {
            is_bl_status = false;
            seid = get_se_id()?;
            sn = get_sn()?;
            //read se cos version
            se_cos_version = get_firmware_version()?;
            se_cos_version = format!("{}.{}.{}",
                                     se_cos_version[0..1].to_string(),
                                     se_cos_version[1..2].to_string(),
                                     se_cos_version[2..].to_string());

        } else if device_cert.starts_with("7f21") || device_cert.starts_with("7F21") {
            seid = device_cert[12..44].to_string();
            sn = "0000000000000000".to_string();
            is_jump = true;
            let mut temp_device_cert = hex_to_bytes("bf2181").unwrap();
            temp_device_cert.push(((device_cert.len()) / 2) as u8);
            temp_device_cert.extend(hex_to_bytes(&device_cert[..device_cert.len()]).unwrap().iter());
            device_cert = hex::encode_upper(temp_device_cert);
        } else {
            return Err(ImkeyError::BCOS0003.into());
        }

        let mut request_data = cos_upgrade_request {
            seid: seid.clone(),
            sn: sn,
            deviceCert: device_cert.clone(),
            seCosVersion: se_cos_version,
            isBLStatus: is_bl_status,
            stepKey: if is_jump { "03".to_string() } else {
                "01".to_string()
            },
            statusWord: None,
            commandID: String::from(TSM_ACTION_COS_UPGRADE),
            cardRetDataList: None,
        };

        loop {
            println!("请求报文：{:#?}", request_data);
            let req_data = serde_json::to_vec_pretty(&request_data).unwrap();
            let mut response_data = https::post(TSM_ACTION_COS_UPGRADE, req_data)?;
            let return_bean: service_response = serde_json::from_str(response_data.as_str())?;
            println!("反馈报文：{:#?}", return_bean);
            if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
                //判断步骤key是否已经结束
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if TSM_END_FLAG.eq(next_step_key.as_str()) {
                    return Ok(());
                }

                let mut apdu_res: Vec<String> = vec![];
                match return_bean._ReturnData.apduList {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //调用发送指令接口，并获取执行结果
                            let res = send_apdu(apdu_val.to_string());
                            apdu_res.push(res.clone());
                            if index_val == apdu_list.len() - 1 {
                                request_data.statusWord = Some(String::from(&res[res.len() -4..]));
                                if constants::APDU_RSP_SUCCESS.eq(&res[res.len() -4..]) &&
                                    ("03".eq(next_step_key.as_str()) ||
                                        "05".eq(next_step_key.as_str())) {
                                    let connect_ret = hid_api::hid_connect();
                                    let mut hid_device_obj = DEVICE.lock().unwrap();
                                    *hid_device_obj = connect_ret;
                                    std::mem::drop(hid_device_obj);
                                    let mut device_cert = get_cert();
                                    println!("aaaaa{}", device_cert);
                                }
                            }
                        }
                        request_data.cardRetDataList = Some(apdu_res);
                    }
                    None => (),
                }

                if "06".eq(next_step_key.as_str()) {//applet download
                    match &return_bean._ReturnData.InstanceAidList {
                        Some(aid_list) =>{
                            for temp_instance_aid in return_bean._ReturnData.InstanceAidList.unwrap().iter() {
                                app_download_request::build_request_data(seid.clone(),
                                                                         temp_instance_aid.clone(),
                                                                         device_cert.clone(),
                                                                         sdk_version.clone()).app_download()?;
                            }
                        },
                        None => (),
                    };
                }

                request_data.stepKey = next_step_key;
            } else {
                 return match return_bean._ReturnCode.as_str() {
                    constants::TSM_RETURNCODE_COS_INFO_NO_CONF => Err(ImkeyError::IMKEY_TSM_COS_INFO_NO_CONF.into()),
                    constants::TSM_RETURNCODE_COS_UPGRADE_FAIL => Err(ImkeyError::IMKEY_TSM_COS_UPGRADE_FAIL.into()),
                    constants::TSM_RETURNCODE_UPLOAD_COS_VERSION_IS_NULL => Err(ImkeyError::IMKEY_TSM_UPLOAD_COS_VERSION_IS_NULL.into()),
                    constants::TSM_RETURNCODE_SWITCH_BL_STATUS_FAIL => Err(ImkeyError::IMKEY_TSM_SWITCH_BL_STATUS_FAIL.into()),
                    constants::TSM_RETURNCODE_WRITE_WALLET_ADDRESS_FAIL => Err(ImkeyError::IMKEY_TSM_WRITE_WALLET_ADDRESS_FAIL.into()),
                    constants::TSM_RETURNCODE_DEVICE_CHECK_FAIL => Err(ImkeyError::BSE0009.into()),
                     constants::TSM_RETURNCODE_OCE_CERT_CHECK_FAIL => Err(ImkeyError::BSE0010.into()),
                     constants::TSM_RETURNCODE_DEVICE_ILLEGAL => Err(ImkeyError::BSE0017.into()),
                     constants::TSM_RETURNCODE_DEV_INACTIVATED => Err(ImkeyError::BSE0007.into()),
                    _ => Err(ImkeyError::IMKEY_TSM_SERVER_ERROR.into()),
                };


            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cos_upgrade::cos_upgrade_request;
    use std::collections::HashMap;

    #[test]
    fn cos_upgrade_test() {
        let upgrade_result = cos_upgrade_request::cos_upgrade(None);
        assert_eq!(true, upgrade_result.is_ok());
    }
}