use std::fmt;

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum BtcError {
    #[fail(display = "imkey_exceeded_max_utxo_number")]
    IMKEY_EXCEEDED_MAX_UTXO_NUMBER,
    #[fail(display = "imkey_address_mismatch_with_path")]
    IMKEY_ADDRESS_MISMATCH_WITH_PATH,
    #[fail(display = "imkey_signature_verify_fail")]
    IMKEY_SIGNATURE_VERIFY_FAIL,
    #[fail(display = "imkey_insufficient_funds")]
    IMKEY_INSUFFICIENT_FUNDS,
    #[fail(display = "imkey_sdk_illegal_argument")]
    IMKEY_SDK_ILLEGAL_ARGUMENT,
    #[fail(display = "imkey_amount_less_than_minimum")]
    IMKEY_AMOUNT_LESS_THAN_MINIMUM,
    #[fail(display = "imkey_path_illegal")]
    ImkeyPathIllegal,
    #[fail(display = "get_xpub_error")]
    GetXpubError,
    #[fail(display = "address_type_mismatch")]
    ADDRESS_TYPE_MISMATCH,
}
