#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum CommonError {
    #[fail(display = "imkey_path_illegal")]
    ImkeyPathIllegal,
}

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum ApduError {
    #[fail(display = "imkey_user_not_confirmed")]
    ImkeyUserNotConfirmed,
    #[fail(display = "imkey_conditions_not_satisfied")]
    ImkeyConditionsNotSatisfied,
    #[fail(display = "imkey_command_format_error")]
    ImkeyCommandFormatError,
    #[fail(display = "imkey_command_data_error")]
    ImkeyCommandDataError,
    #[fail(display = "imkey_applet_not_exist")]
    ImkeyAppletNotExist,
    #[fail(display = "imkey_apdu_wrong_length")]
    ImkeyApduWrongLength,
    #[fail(display = "imkey_signature_verify_fail")]
    ImkeySignatureVerifyFail,
    #[fail(display = "imkey_bluetooth_channel_error")]
    ImkeyBluetoothChannelError,
    #[fail(display = "imkey_applet_function_not_supported")]
    ImkeyAppletFunctionNotSupported,
    #[fail(display = "imkey_exceeded_max_utxo_number")]
    ImkeyExceededMaxUtxoNumber,
    #[fail(display = "imkey_command_execute_fail")]
    ImkeyCommandExecuteFail,
    #[fail(display = "imkey_wallet_not_created")]
    ImkeyWalletNotCreated,
    #[fail(display = "imkey_in_menu_page")]
    ImkeyInMenuPage,
    #[fail(display = "imkey_pin_not_verified")]
    ImkeyPinNotVerified,
}

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum CoinError {
    #[fail(display = "imkey_exceeded_max_utxo_number")]
    ImkeyExceededMaxUtxoNumber,
    #[fail(display = "imkey_address_mismatch_with_path")]
    ImkeyAddressMismatchWithPath,
    #[fail(display = "imkey_signature_verify_fail")]
    ImkeySignatureVerifyFail,
    #[fail(display = "imkey_insufficient_funds")]
    ImkeyInsufficientFunds,
    #[fail(display = "imkey_sdk_illegal_argument")]
    ImkeySdkIllegalArgument,
    #[fail(display = "imkey_amount_less_than_minimum")]
    ImkeyAmountLessThanMinimum,
    #[fail(display = "imkey_path_illegal")]
    ImkeyPathIllegal,
    #[fail(display = "get_xpub_error")]
    GetXpubError,
    #[fail(display = "address_type_mismatch")]
    AddressTypeMismatch,
}
