use std::fmt;

pub enum BtcError {
    ImkeyExceededMaxUtxoNumber,
    ImkeyAddressMismatchWithPath,
    ImkeySignatureVerifyFail,
    ImkeyInsufficientFunds,
    ImkeySdkIllegalArgument,
}

impl fmt::Display for BtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BtcError::ImkeyExceededMaxUtxoNumber => write!(f, "imkey_exceeded_max_utxo_number"),
            BtcError::ImkeyAddressMismatchWithPath => write!(f, "imkey_address_mismatch_with_path"),
            BtcError::ImkeySignatureVerifyFail => write!(f, "imkey_signature_verify_fail"),
            BtcError::ImkeyInsufficientFunds => write!(f, "imkey_insufficient_funds"),
            BtcError::ImkeySdkIllegalArgument => write!(f, "imkey_sdk_illegal_argument"),
        }
    }
}
