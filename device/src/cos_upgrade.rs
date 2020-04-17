use common::constants::{TSM_ACTION_COS_UPGRADE, TSM_RETURN_CODE_SUCCESS, TSM_END_FLAG, DEVICE_MODEL_NAME};
use common::{https, constants};
use serde::{Deserialize, Serialize};
use mq::message::{send_apdu};
use crate::manager::{get_se_id, get_sn, get_firmware_version, get_cert};
use common::utility::hex_to_bytes;
use crate::app_download::AppDownloadRequest;
use crate::Result;
use crate::error::ImkeyError;
use std::thread;
use std::time::Duration;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use mq::hid_api::{hid_connect};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CosUpgradeRequest {
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResponse {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: CosUpgradeResponse,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CosUpgradeResponse {
    pub seid: Option<String>,
    pub CosVersion: Option<String>,
    pub InstanceAidList: Option<Vec<String>>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
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

        let mut request_data = CosUpgradeRequest {
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
            let response_data = https::post(TSM_ACTION_COS_UPGRADE, req_data)?;
            let return_bean: ServiceResponse = serde_json::from_str(response_data.as_str())?;
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
                            let res = send_apdu(apdu_val.to_string())?;
                            apdu_res.push(res.clone());
                            if index_val == apdu_list.len() - 1 {
                                request_data.statusWord = Some(String::from(&res[res.len() -4..]));
                                if (constants::APDU_RSP_SUCCESS.eq(&res[res.len() -4..]) ||
                                    constants::APDU_RSP_SWITCH_BL_STATUS_SUCCESS.eq(&res[res.len() -4..])) &&
                                    ("03".eq(next_step_key.as_str()) ||
                                        "05".eq(next_step_key.as_str())) {
//                                    thread::sleep(Duration::from_millis(1000));
//                                    let connect_ret = hid_api::hid_connect()?;
//                                    let mut hid_device_obj = HID_DEVICE.lock().unwrap();
//                                    *hid_device_obj = connect_ret;
//                                    std::mem::drop(hid_device_obj);
                                    reconnect()?;
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
                            for temp_instance_aid in aid_list.iter() {
                                AppDownloadRequest::build_request_data(seid.clone(),
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
                    constants::TSM_RETURNCODE_COS_INFO_NO_CONF => Err(ImkeyError::ImkeyTsmCosInfoNoConf.into()),
                    constants::TSM_RETURNCODE_COS_UPGRADE_FAIL => Err(ImkeyError::ImkeyTsmCosUpgradeFail.into()),
                    constants::TSM_RETURNCODE_UPLOAD_COS_VERSION_IS_NULL => Err(ImkeyError::ImkeyTsmUploadCosVersionIsNull.into()),
                    constants::TSM_RETURNCODE_SWITCH_BL_STATUS_FAIL => Err(ImkeyError::ImkeyTsmSwitchBlStatusFail.into()),
                    constants::TSM_RETURNCODE_WRITE_WALLET_ADDRESS_FAIL => Err(ImkeyError::ImkeyTsmWriteWalletAddressFail.into()),
                    constants::TSM_RETURNCODE_DEVICE_CHECK_FAIL => Err(ImkeyError::BSE0009.into()),
                     constants::TSM_RETURNCODE_OCE_CERT_CHECK_FAIL => Err(ImkeyError::BSE0010.into()),
                     constants::TSM_RETURNCODE_DEVICE_ILLEGAL => Err(ImkeyError::BSE0017.into()),
                     constants::TSM_RETURNCODE_DEV_INACTIVATED => Err(ImkeyError::BSE0007.into()),
                    _ => Err(ImkeyError::ImkeyTsmServerError.into()),
                };


            }
        }
    }
}

//reconnect device
fn reconnect() -> Result<()>{
    thread::sleep(Duration::from_millis(1000));

    for _ in 0..5 {
        if hid_connect(DEVICE_MODEL_NAME).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(1000));
        continue;
    };

    Err(ImkeyError::ImkeyDeviceReconnectFail.into())
}

#[cfg(test)]
mod tests {
    use crate::cos_upgrade::CosUpgradeRequest;
    use std::collections::HashMap;
    use mq::hid_api::hid_connect;
    use mq::message::send_apdu;

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