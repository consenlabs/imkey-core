
#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum BtcError {
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
