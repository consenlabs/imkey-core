use common::constants::{TSM_ACTION_COS_UPGRADE, TSM_RETURN_CODE_SUCCESS};
use common::{error::ImkeyError, https};
//use mq::message;
use serde::{Deserialize, Serialize};
use mq::message::{send_apdu};
use crate::manager;
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


#[derive(Debug, Serialize, Deserialize)]
pub struct cos_upgrade_request {
    pub seid: String,
    pub sn: String,
    pub deviceCert: String,
    pub seCosVersion: String,
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
//    pub fn build_request_data(
//        seid: String,
//        sn: String,
//        device_cert: String,
//        se_cos_version: String,
//    ) -> cos_upgrade_request {
//        cos_upgrade_request {
//            seid: seid,
//            sn: sn,
//            deviceCert: device_cert,
//            seCosVersion: se_cos_version,
//            stepKey: String::from("01"),
//            statusWord: None,
//            commandID: String::from(TSM_ACTION_COS_UPGRADE),
//            cardRetDataList: None,
//        }
//    }

    //    pub fn cos_upgrade(&mut self) -> Result<(), ImkeyError> {
    pub fn cos_upgrade() -> Result<(), ImkeyError> {
        //read se device cert
        let mut device_cert = manager::get_cert();
//        if !"9000".eq(&device_cert[device_cert.len() - 4..]) {
//            return Err(ImkeyError::COS_UPGRADE_ERROR);
//        }
        let mut is_jump = false;
        let mut seid = "".to_string();
        let mut sn = "".to_string();
        let mut se_cos_version = "".to_string();

        //read seid and sn number
        if device_cert.starts_with("bf21") || device_cert.starts_with("BF21") {
            seid = manager::get_se_id().ok().unwrap();
            sn = String::from_utf8(hex_to_bytes(&manager::get_sn().ok().unwrap()).unwrap()).expect("conver sn number error");
            //read se cos version
            send_apdu("00A4040000".to_string());
            let apdu_response = send_apdu("80CB800005DFFF02800300".to_string());
            if !"9000".eq(&apdu_response[apdu_response.len() - 4..]) {
                return Err(ImkeyError::COS_UPGRADE_ERROR);
            }
            se_cos_version = format!("{}.{}.{}",
                                     apdu_response[0..1].to_string(),
                                     apdu_response[1..2].to_string(),
                                     apdu_response[2..apdu_response.len() - 4].to_string());

        } else if device_cert.starts_with("7f21") || device_cert.starts_with("7F21") {
            seid = device_cert[12..44].to_string();
//            seid = "19060000000200860001010000000014".to_string();
            sn = "0000000000000000".to_string();
            is_jump = true;
            let mut temp_device_cert = hex_to_bytes("bf2181").unwrap();
            temp_device_cert.push(((device_cert.len()) / 2) as u8);
//            temp_device_cert.extend(hex_to_bytes(&device_cert[..device_cert.len() - 4]).unwrap().iter()); TODO
            temp_device_cert.extend(hex_to_bytes(&device_cert[..device_cert.len()]).unwrap().iter());
            device_cert = hex::encode_upper(temp_device_cert);
//            device_cert = "BF2181CA7F2181C6931019060000000200860001010000000014420200015F200401020304950200805F2504201810145F2404FFFFFFFF53007F4947B04104FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44F002DFFE5F374830460221008CB58D54BDED501236621B83B320081E6F9B6B5539AE5EC9D36B660EC445A5E8022100A203CA1F9ABEE69751EA402A2ACDFD6B4A87697D6CD721F60540959095EC9466".to_string();
        } else {
            return Err(ImkeyError::COS_UPGRADE_ERROR);
        }

        let mut request_data = cos_upgrade_request {
            seid: seid.clone(),
            sn: sn,
            deviceCert: device_cert.clone(),
            seCosVersion: se_cos_version,
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
            let mut response_data = https::post(TSM_ACTION_COS_UPGRADE, req_data);
            let return_bean: service_response =
                serde_json::from_str(response_data.ok().unwrap().as_str())
                    .expect("imkey message seriailize error");

            println!("反馈报文：{:#?}", return_bean);
            if return_bean._ReturnCode == TSM_RETURN_CODE_SUCCESS {
                //判断步骤key是否已经结束
                let next_step_key = return_bean._ReturnData.nextStepKey.unwrap();
                if "end".eq(next_step_key.as_str()) {
                    println!("COS升级成功结束");
                    return Ok(());
                }

                if "06".eq(next_step_key.as_str()) {//applet download
                    for temp_instance_aid in return_bean._ReturnData.InstanceAidList.unwrap().iter() {
                        let app_dwonlaod_result = app_download_request::build_request_data(seid.clone(),
                                                                                           temp_instance_aid.clone(),
                                                                                           device_cert.clone(),
                                                                                           None)
                            .app_download();
                        if app_dwonlaod_result.is_err() {
                            return Err(ImkeyError::BAPP0006);
                        }
                    }
                }

                let mut apdu_res: Vec<String> = Vec::new();
                match return_bean._ReturnData.apduList {
                    Some(apdu_list) => {
                        for (index_val, apdu_val) in apdu_list.iter().enumerate() {
                            //调用发送指令接口，并获取执行结果
                            let res = send_apdu(apdu_val.to_string());

                            apdu_res.push(String::from(&res));
                            if index_val == apdu_list.len() - 1 {
                                let status: String =
                                    res.chars().skip(res.len() - 4).take(4).collect();
                                request_data.statusWord = Some(String::from(status));
                            }
                        }
                        request_data.cardRetDataList = Some(apdu_res);
                    }
                    None => (),
                }

//                if "03".eq(next_step_key.as_str()) || "06".eq(next_step_key.as_str()) {
//                    let mut device = DEVICE.lock().unwrap();
//                    std::mem::drop(device);
////                    *device = hid_api::hid_connect();
//                    hid_api::hid_connect();
//
//                    let mut device_cert = manager::get_cert();
//                    println!("aaaaa{}", device_cert);
//                }
                request_data.stepKey = next_step_key;
            } else {
                println!("应用服务器执行失败并返回 : {}", return_bean._ReturnMsg);
                return Err(ImkeyError::BAPP0006);
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
        cos_upgrade_request::cos_upgrade();
    }
}