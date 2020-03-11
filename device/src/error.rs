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
    #[fail(display = "imkey_tsm_device_authenticity_check_fail")]
    IMKEY_TSM_DEVICE_AUTHENTICITY_CHECK_FAIL,
    #[fail(display = "imkey_tsm_device_not_activated")]
    IMKEY_TSM_DEVICE_NOT_ACTIVATED,
    #[fail(display = "imkey_tsm_device_illegal")]
    IMKEY_TSM_DEVICE_ILLEGAL,
    #[fail(display = "imkey_tsm_device_stop_using")]
    IMKEY_TSM_DEVICE_STOP_USING,
    #[fail(display = "imkey_tsm_server_error")]
    IMKEY_TSM_SERVER_ERROR,
    #[fail(display = "imkey_se_cert_invalid")]
    IMKEY_SE_CERT_INVALID,
    #[fail(display = "imkey_tsm_device_update_check_fail")]
    IMKEY_TSM_DEVICE_UPDATE_CHECK_FAIL,
    #[fail(display = "imkey_tsm_device_active_fail")]
    IMKEY_TSM_DEVICE_ACTIVE_FAIL,
    #[fail(display = "imkey_tsm_receipt_check_fail")]
    IMKEY_TSM_RECEIPT_CHECK_FAIL,
    #[fail(display = "imkey_tsm_app_download_fail")]
    IMKEY_TSM_APP_DOWNLOAD_FAIL,
    #[fail(display = "imkey_tsm_app_update_fail")]
    IMKEY_TSM_APP_UPDATE_FAIL,
    #[fail(display = "imkey_tsm_app_delete_fail")]
    IMKEY_TSM_APP_DELETE_FAIL,
    #[fail(display = "imkey_tsm_oce_cert_check_fail")]
    IMKEY_TSM_OCE_CERT_CHECK_FAIL,
}

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum BindError {
    #[fail(display = "imkey_keyfile_io_error")]
    IMKEY_KEYFILE_IO_ERROR,
    #[fail(display = "imkey_sdk_illegal_argument")]
    IMKEY_SDK_ILLEGAL_ARGUMENT,
    #[fail(display = "imkey_encrypt_authcode_fail")]
    IMKEY_ENCRYPT_AUTHCODE_FAIL,
}
