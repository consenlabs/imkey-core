use std::fmt;

pub enum BtcError {
    ImkeyExceededMaxUtxoNumber,
    ImkeyAddressMismatchWithPath,
    ImkeySignatureVerifyFail,
    ImkeyInsufficientFunds,
    ImkeySdkIllegalArgument,
    ImkeyAmountLessThanMinimum,
    ImkeyPathIllegal,
    InvalidPublicKey,
    GetXpubError,
}

impl fmt::Display for BtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BtcError::ImkeyExceededMaxUtxoNumber => write!(f, "imkey_exceeded_max_utxo_number"),
            BtcError::ImkeyAddressMismatchWithPath => write!(f, "imkey_address_mismatch_with_path"),
            BtcError::ImkeySignatureVerifyFail => write!(f, "imkey_signature_verify_fail"),
            BtcError::ImkeyInsufficientFunds => write!(f, "imkey_insufficient_funds"),
            BtcError::ImkeySdkIllegalArgument => write!(f, "imkey_sdk_illegal_argument"),
            BtcError::ImkeyAmountLessThanMinimum => write!(f, "imkey_amount_less_than_minimum"),
            BtcError::ImkeyPathIllegal => write!(f, "imkey_path_illegal"),
            BtcError::InvalidPublicKey => write!(f, "secp: malformed public key"),
            BtcError::GetXpubError => write!(f, "get_xpub_error"),
        }
    }
}
