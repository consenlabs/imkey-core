//! Query SN registration/activation metadata. This does not replace device authenticity checks.
use crate::{Result, ServiceResponse, TsmService};
use common::{constants, https};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSnCheckRequest {
    pub sn: String,
    pub step_key: String,
    pub status_word: Option<String>,
    #[serde(rename = "commandID")]
    pub command_id: String,
    pub card_ret_data_list: Option<Vec<String>>,
    pub sdk_version: Option<String>,
    pub terminal_type: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct DeviceSnCheckResult {
    pub sn_check_result: bool,
    pub is_activate: bool,
    /// Server calendar date (yyyy-MM-dd), without an inferred time zone.
    pub activate_time: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceSnCheckResponse {
    sn_check_result: Option<bool>,
    is_activate: Option<bool>,
    activate_time: Option<String>,
}

impl DeviceSnCheckRequest {
    pub fn build_request_data(sn: String) -> Self {
        DeviceSnCheckRequest {
            sn,
            step_key: "01".to_string(),
            status_word: None,
            command_id: constants::TSM_ACTION_DEVICE_SN_CHECK.to_string(),
            card_ret_data_list: None,
            sdk_version: Some(common::SDK_VERSION.read().to_string()),
            terminal_type: Some(common::TERMINAL_TYPE.read().to_string()),
        }
    }
}

impl TsmService for DeviceSnCheckRequest {
    type ReturnData = DeviceSnCheckResult;

    fn send_message(&mut self) -> Result<Self::ReturnData> {
        let request = serde_json::to_vec(self)?;
        let response = https::post(constants::TSM_ACTION_DEVICE_SN_CHECK, request)?;
        parse_response(&response)
    }
}

fn invalid_response() -> failure::Error {
    format_err!("imkey_tsm_device_sn_check_invalid_response")
}

fn parse_response(response: &str) -> Result<DeviceSnCheckResult> {
    // Decode the envelope separately so a server error with null data retains its business error.
    let envelope: ServiceResponse<serde_json::Value> =
        serde_json::from_str(response).map_err(|_| invalid_response())?;
    match envelope._ReturnCode.as_str() {
        "BDEVICE002" => return Err(format_err!("imkey_tsm_device_sn_check_fail")),
        constants::TSM_RETURN_CODE_SUCCESS | "BDEVICE003" => (),
        _ => {
            envelope.service_res_check()?;
            return Err(invalid_response());
        }
    }
    let data: DeviceSnCheckResponse =
        serde_json::from_value(envelope._ReturnData).map_err(|_| invalid_response())?;
    if data.sn_check_result != Some(true) || data.is_activate.is_none() {
        return Err(invalid_response());
    }
    let date = data.activate_time.unwrap_or_default();
    if !date.is_empty() && !valid_date(&date) {
        return Err(invalid_response());
    }
    let activated = envelope._ReturnCode == constants::TSM_RETURN_CODE_SUCCESS;
    if !activated && (data.is_activate != Some(false) || !date.is_empty()) {
        return Err(invalid_response());
    }
    // imkey-server DeviceSnCheckServiceImpl returns 000000 only for activated cards,
    // but leaves isActivate=false when their activation timestamp is NULL. The return
    // code remains authoritative in that documented case; no timestamp is fabricated.
    Ok(DeviceSnCheckResult {
        sn_check_result: true,
        is_activate: activated,
        activate_time: date,
    })
}

fn valid_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes
            .iter()
            .enumerate()
            .any(|(i, c)| i != 4 && i != 7 && !c.is_ascii_digit())
    {
        return false;
    }
    let year: u32 = date[0..4].parse().unwrap_or(0);
    let month: u32 = date[5..7].parse().unwrap_or(0);
    let day: u32 = date[8..10].parse().unwrap_or(0);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let max_day = match month {
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0,
    };
    year > 0 && day > 0 && day <= max_day
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn response(code: &str, data: serde_json::Value) -> String {
        json!({"_ReturnCode":code,"_ReturnMsg":"test","_ReturnData":data}).to_string()
    }

    #[test]
    fn protobuf_fields_match_public_contract() {
        use crate::deviceapi::DeviceSnCheckRes;
        use prost::Message;
        let result = DeviceSnCheckRes {
            sn_check_result: true,
            is_activate: true,
            activate_time: "2024-02-29".to_string(),
        };
        let mut bytes = Vec::new();
        result.encode(&mut bytes).unwrap();
        assert_eq!(&bytes[..6], &[8, 1, 16, 1, 26, 10]);
        assert_eq!(DeviceSnCheckRes::decode(bytes.as_slice()).unwrap(), result);
    }

    #[test]
    fn request_matches_server_contract() {
        let request = DeviceSnCheckRequest::build_request_data("test-sn".to_string());
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["sn"], "test-sn");
        assert_eq!(value["stepKey"], "01");
        assert_eq!(value["commandID"], "/deviceSnCheck");
        assert!(value["sdkVersion"].is_string());
        assert!(value["terminalType"].is_string());
        assert!(value.get("seid").is_none());
    }

    #[test]
    fn activated_preserves_calendar_date() {
        let result = parse_response(&response(
            "000000",
            json!({"snCheckResult":true,"isActivate":true,"activateTime":"2024-02-29"}),
        ))
        .unwrap();
        assert_eq!(
            result,
            DeviceSnCheckResult {
                sn_check_result: true,
                is_activate: true,
                activate_time: "2024-02-29".to_string()
            }
        );
    }

    #[test]
    fn activated_without_timestamp_uses_return_code() {
        for data in [
            json!({"snCheckResult":true,"isActivate":false,"activateTime":null}),
            json!({"snCheckResult":true,"isActivate":true,"activateTime":""}),
            json!({"snCheckResult":true,"isActivate":true}),
        ]
        .iter()
        {
            let result = parse_response(&response("000000", data.clone())).unwrap();
            assert!(result.is_activate);
            assert_eq!(result.activate_time, "");
        }
    }

    #[test]
    fn unactivated_is_successful_metadata() {
        let result = parse_response(&response(
            "BDEVICE003",
            json!({"snCheckResult":true,"isActivate":false}),
        ))
        .unwrap();
        assert!(result.sn_check_result);
        assert!(!result.is_activate);
        assert!(result.activate_time.is_empty());
    }

    #[test]
    fn invalid_sn_and_server_errors_remain_distinct() {
        assert_eq!(
            parse_response(&response("BDEVICE002", serde_json::Value::Null))
                .unwrap_err()
                .to_string(),
            "imkey_tsm_device_sn_check_fail"
        );
        assert_eq!(
            parse_response(&response("UNKNOWN", serde_json::Value::Null))
                .unwrap_err()
                .to_string(),
            "imkey_tsm_server_error"
        );
        assert_eq!(
            parse_response(&response("BSE0017", serde_json::Value::Null))
                .unwrap_err()
                .to_string(),
            "imkey_tsm_device_illegal"
        );
    }

    #[test]
    fn malformed_payload_never_passes() {
        for data in [
            serde_json::Value::Null,
            json!({}),
            json!({"snCheckResult":true}),
            json!({"isActivate":true}),
            json!({"snCheckResult":false,"isActivate":true}),
            json!({"snCheckResult":"true","isActivate":true}),
        ]
        .iter()
        {
            assert!(parse_response(&response("000000", data.clone())).is_err());
        }
        for text in ["", "{}", "null", "not-json"].iter() {
            assert!(parse_response(text).is_err());
        }
        assert!(parse_response(&response(
            "BDEVICE003",
            json!({"snCheckResult":true,"isActivate":true})
        ))
        .is_err());
    }

    #[test]
    fn malformed_dates_never_pass() {
        for date in [
            "2023-02-29",
            "1900-02-29",
            "2024-04-31",
            "2024-00-01",
            "2024-01-00",
            "0000-01-01",
            "2024-1-01",
            "2024-01-01T00:00:00",
            "２０２４-01-01",
        ]
        .iter()
        {
            assert!(
                parse_response(&response(
                    "000000",
                    json!({"snCheckResult":true,"isActivate":true,"activateTime":date})
                ))
                .is_err(),
                "{}",
                date
            );
        }
        assert!(valid_date("2000-02-29"));
        assert!(valid_date("2024-12-31"));
    }
}
