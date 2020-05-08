use crate::error::ImkeyError;
use crate::ServiceResponse;
use crate::{Result, TsmService};
use common::constants::{
    TSM_ACTION_DEVICE_CERT_CHECK, TSM_RETURNCODE_DEVICE_CHECK_FAIL, TSM_RETURNCODE_DEVICE_ILLEGAL,
    TSM_RETURNCODE_DEVICE_STOP_USING, TSM_RETURNCODE_DEV_INACTIVATED, TSM_RETURN_CODE_SUCCESS,
};
use common::https;
use serde::{Deserialize, Serialize};

// SE安全检查请求bean
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCertCheckRequest {
    pub seid: String,
    pub sn: String,
    pub deviceCert: String,
    pub stepKey: String,
    pub statusWord: Option<String>,
    pub commandID: String,
    pub cardRetDataList: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceCertCheckResponse {
    pub seid: Option<String>,
    pub verifyResult: Option<bool>,
    pub nextStepKey: Option<String>,
    pub apduList: Option<Vec<String>>,
}

impl TsmService for DeviceCertCheckRequest {
    type ReturnData = ();

    fn send_message(&mut self) -> Result<()> {
        println!("send message：{:#?}", self);
        let req_data = serde_json::to_vec_pretty(&self).unwrap();
        let response_data = https::post(TSM_ACTION_DEVICE_CERT_CHECK, req_data)?;
        let return_bean: ServiceResponse<DeviceCertCheckResponse> =
            serde_json::from_str(response_data.as_str())?;
        println!("return message：{:#?}", return_bean);

        match return_bean._ReturnCode.as_str() {
            TSM_RETURN_CODE_SUCCESS => {
                if return_bean._ReturnData.verifyResult.unwrap() {
                    return Ok(());
                }
                return Err(ImkeyError::ImkeySeCertInvalid.into());
            }
            TSM_RETURNCODE_DEVICE_CHECK_FAIL => {
                Err(ImkeyError::ImkeyTsmDeviceAuthenticityCheckFail.into())
            }
            TSM_RETURNCODE_DEV_INACTIVATED => Err(ImkeyError::ImkeyTsmDeviceNotActivated.into()),
            TSM_RETURNCODE_DEVICE_ILLEGAL => Err(ImkeyError::ImkeyTsmDeviceIllegal.into()),
            TSM_RETURNCODE_DEVICE_STOP_USING => Err(ImkeyError::ImkeyTsmDeviceStopUsing.into()),
            _ => Err(ImkeyError::ImkeyTsmServerError.into()),
        }
    }
}

impl DeviceCertCheckRequest {
    pub fn build_request_data(seid: String, sn: String, device_cert: String) -> Self {
        DeviceCertCheckRequest {
            seid: seid,
            sn: sn,
            deviceCert: device_cert,
            stepKey: String::from("01"),
            statusWord: None,
            commandID: String::from(TSM_ACTION_DEVICE_CERT_CHECK),
            cardRetDataList: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::device_cert_check::DeviceCertCheckRequest;
    use crate::TsmService;

    #[test]
    pub fn device_cert_check_test() {
        let seid: String = "19060000000200860001010000000014".to_string();
        let sn: String = "imKey01191200001".to_string();
        let device_cert: String = "BF2181CA7F2181C6931019060000000200860001010000000014420200015F200401020304950200805F2504201810145F2404FFFFFFFF53007F4947B04104FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44F002DFFE5F374830460221008CB58D54BDED501236621B83B320081E6F9B6B5539AE5EC9D36B660EC445A5E8022100A203CA1F9ABEE69751EA402A2ACDFD6B4A87697D6CD721F60540959095EC9466".to_string();
        DeviceCertCheckRequest::build_request_data(seid, sn, device_cert).send_message();
    }
}
