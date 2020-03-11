use serde::export::Formatter;
use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum ImkeyError {
    #[fail(display = "imkey_tsm_device_not_activated")]
    BSE0007,
    #[fail(display = "imkey_tsm_device_illegal")]
    BSE0017,
    #[fail(display = "imkey_tsm_device_stop_using")]
    BSE0019,
    #[fail(display = "imkey_tsm_device_update_check_fail")]
    BSE0018,
    #[fail(display = "imkey_tsm_device_active_fail")]
    BSE0015,
    #[fail(display = "imkey_tsm_device_illegal")]
    BSE0008,
    #[fail(display = "imkey_tsm_device_authenticity_check_fail")]
    BSE0009,
    #[fail(display = "imkey_tsm_receipt_check_fail")]
    BSE0012,
    #[fail(display = "imkey_tsm_oce_cert_check_fail")]
    BSE0010,
    #[fail(display = "imkey_auth_code_ciphertext_storage_fail")]
    BSE0021,
    #[fail(display = "imkey_tsm_app_download_fail")]
    BAPP0006,
    #[fail(display = "imkey_tsm_app_update_fail")]
    BAPP0008,
    #[fail(display = "imkey_tsm_app_delete_fail")]
    BAPP0011,
    #[fail(display = "imkey_tsm_cos_upgrade_fail")]
    BCOS0003,
}

//impl Display for ImkeyError {
//    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//        match &self {
//            ImkeyError::BSE0007 => write!(f, "imkey_tsm_device_not_activated"),
//            ImkeyError::BSE0008 => write!(f, "imkey_tsm_app_update_fail"),
//            ImkeyError::BSE0009 => write!(f, "imkey_tsm_device_authenticity_check_fail"),
//            ImkeyError::BSE0010 => write!(f, "imkey_tsm_oce_cert_check_fail"),
//            ImkeyError::BSE0012 => write!(f, "imkey_tsm_receipt_check_fail"),
//            ImkeyError::BSE0015 => write!(f, "imkey_tsm_device_active_fail"),
//            ImkeyError::BSE0017 => write!(f, "imkey_tsm_device_illegal"),
//            ImkeyError::BSE0018 => write!(f, "imkey_tsm_device_update_check_fail"),
//            ImkeyError::BSE0019 => write!(f, "imkey_tsm_device_stop_using"),
//            ImkeyError::BAPP0006 => write!(f, "imkey_tsm_app_download_fail"),
//            ImkeyError::BAPP0008 => write!(f, "imkey_tsm_app_update_fail"),
//            ImkeyError::BAPP0011 => write!(f, "imkey_tsm_app_delete_fail"),
//        }
//    }
//}
pub const MESSAGE_CONVER_ERROR: &str = "";