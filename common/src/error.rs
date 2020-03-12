#[macro_use()]
use std::fmt;

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum CommonError {
    #[fail(display = "imkey_path_illegal")]
    IMKEY_PATH_ILLEGAL,
}

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum ApduError {
    #[fail(display = "imkey_user_not_confirmed")]
    IMKEY_USER_NOT_CONFIRMED,
    #[fail(display = "imkey_conditions_not_satisfied")]
    IMKEY_CONDITIONS_NOT_SATISFIED,
    #[fail(display = "imkey_command_format_error")]
    IMKEY_COMMAND_FORMAT_ERROR,
    #[fail(display = "imkey_command_data_error")]
    IMKEY_COMMAND_DATA_ERROR,
    #[fail(display = "imkey_applet_not_exist")]
    IMKEY_APPLET_NOT_EXIST,
    #[fail(display = "imkey_apdu_wrong_length")]
    IMKEY_APDU_WRONG_LENGTH,
    #[fail(display = "imkey_signature_verify_fail")]
    IMKEY_SIGNATURE_VERIFY_FAIL,
    #[fail(display = "imkey_bluetooth_channel_error")]
    IMKEY_BLUETOOTH_CHANNEL_ERROR,
    #[fail(display = "imkey_applet_function_not_supported")]
    IMKEY_APPLET_FUNCTION_NOT_SUPPORTED,
    #[fail(display = "imkey_exceeded_max_utxo_number")]
    IMKEY_EXCEEDED_MAX_UTXO_NUMBER,
    #[fail(display = "imkey_command_execute_fail")]
    IMKEY_COMMAND_EXECUTE_FAIL,
    #[fail(display = "imkey_wallet_not_created")]
    IMKEY_WALLET_NOT_CREATED,
    #[fail(display = "imkey_in_menu_page")]
    IMKEY_IN_MENU_PAGE,
    #[fail(display = "imkey_pin_not_verified")]
    IMKEY_PIN_NOT_VERIFIED,

}

