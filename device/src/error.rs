
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
    ImkeyTsmDeviceAuthenticityCheckFail,
    #[fail(display = "imkey_tsm_device_not_activated")]
    ImkeyTsmDeviceNotActivated,
    #[fail(display = "imkey_tsm_device_illegal")]
    ImkeyTsmDeviceIllegal,
    #[fail(display = "imkey_tsm_device_stop_using")]
    ImkeyTsmDeviceStopUsing,
    #[fail(display = "imkey_tsm_server_error")]
    ImkeyTsmServerError,
    #[fail(display = "imkey_se_cert_invalid")]
    ImkeySeCertInvalid,
    #[fail(display = "imkey_tsm_device_update_check_fail")]
    ImkeyTsmDeviceUpdateCheckFail,
    #[fail(display = "imkey_tsm_device_active_fail")]
    ImkeyTsmDeviceActiveFail,
    #[fail(display = "imkey_tsm_receipt_check_fail")]
    ImkeyTsmReceiptCheckFail,
    #[fail(display = "imkey_tsm_app_download_fail")]
    ImkeyTsmAppDownloadFail,
    #[fail(display = "imkey_tsm_app_update_fail")]
    ImkeyTsmAppUpdateFail,
    #[fail(display = "imkey_tsm_app_delete_fail")]
    ImkeyTsmAppDeleteFail,
    #[fail(display = "imkey_tsm_oce_cert_check_fail")]
    ImkeyTsmOceCertCheckFail,
    #[fail(display = "imkey_tsm_cos_info_no_conf")]
    ImkeyTsmCosInfoNoConf,
    #[fail(display = "imkey_tsm_cos_upgrade_fail")]
    ImkeyTsmCosUpgradeFail,
    #[fail(display = "imkey_tsm_upload_cos_version_is_null")]
    ImkeyTsmUploadCosVersionIsNull,
    #[fail(display = "imkey_tsm_switch_bl_status_fail")]
    ImkeyTsmSwitchBlStatusFail,
    #[fail(display = "imkey_tsm_write_wallet_address_fail")]
    ImkeyTsmWriteWalletAddressFail,
}

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum BindError {
    #[fail(display = "imkey_keyfile_io_error")]
    ImkeyKeyfileIoError,
    #[fail(display = "imkey_sdk_illegal_argument")]
    ImkeySdkIllegalArgument,
    #[fail(display = "imkey_encrypt_authcode_fail")]
    ImkeyEncryptAuthcodeFail,
    #[fail(display = "imkey_save_key_file_fail")]
    ImkeySaveKeyFileFail,
}
