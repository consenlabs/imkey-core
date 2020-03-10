#[macro_use()]
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Command execution error
    RpcError(String),
    AddressError,
    PrvKeyError,
    PubKeyError,
    MessageError,
    DataError,
    SignError,
    PathError,
    ChainTypeError,
    ProtoError,
    DeviceOpError,
}

macro_rules! from_err {
    ($x:ty) => {
        impl From<$x> for Error {
            fn from(err: $x) -> Self {
                Error::RpcError(format!(
                    "something wrong with rpc call: {}",
                    err.to_string()
                ))
            }
        }
    };
}

from_err!(reqwest::Error);
//from_err!(std::string::ParseError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RpcError(ref str) => write!(f, "rpc call error: {}", str),
            Error::AddressError => write!(f, "address is wrong"),
            Error::PrvKeyError => write!(f, "private key parse error"),
            Error::PubKeyError => write!(f, "public key parse error"),
            Error::MessageError => write!(f, "sigh hash got error"),
            Error::DataError => write!(f, "data field wrong format"),
            Error::SignError => write!(f, "signature error"),
            Error::PathError => write!(f, "path parameter error"),
            Error::ChainTypeError => write!(f, "unsupported chain"),
            Error::ProtoError => write!(f, "protobuf error"),
            Error::DeviceOpError => write!(f, "device operation error"),
        }
    }
}
//=======================================================================
pub const RpcError: &str = "rpc call error: {}";//TODO
pub const AddressError: &str = "address is wrong";
pub const PrvKeyError: &str = "private key parse error";
pub const PubKeyError: &str = "public key parse error";
pub const MessageError: &str = "sigh hash got error";
pub const DataError: &str = "data field wrong format";
pub const SignError: &str = "signature error";
pub const PathError: &str = "path parameter error";
pub const ChainTypeError: &str = "unsupported chain";
pub const ProtoError: &str = "protobuf error";
pub const DeviceOpError: &str = "device operation error";




pub enum ImkeyError {
    BSE0007,
    BSE0017,
    BSE0019,
    BSE0018,
    BSE0015,
    BSE0008,
    BSE0009,
    BSE0012,
    BSE0010,
    BAPP0006,
    BAPP0008,
    BAPP0011,
    NETWORK_ERROR,
    COS_UPGRADE_ERROR,
}

impl fmt::Display for ImkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ImkeyError::BSE0007 => write!(f, "imkey_tsm_device_not_activated"),
            ImkeyError::BSE0008 => write!(f, "imkey_tsm_app_update_fail"),
            ImkeyError::BSE0009 => write!(f, "imkey_tsm_device_authenticity_check_fail"),
            ImkeyError::BSE0010 => write!(f, "imkey_tsm_oce_cert_check_fail"),
            ImkeyError::BSE0012 => write!(f, "imkey_tsm_receipt_check_fail"),
            ImkeyError::BSE0015 => write!(f, "imkey_tsm_device_active_fail"),
            ImkeyError::BSE0017 => write!(f, "imkey_tsm_device_illegal"),
            ImkeyError::BSE0018 => write!(f, "imkey_tsm_device_update_check_fail"),
            ImkeyError::BSE0019 => write!(f, "imkey_tsm_device_stop_using"),
            ImkeyError::BAPP0006 => write!(f, "imkey_tsm_app_download_fail"),
            ImkeyError::BAPP0008 => write!(f, "imkey_tsm_app_update_fail"),
            ImkeyError::BAPP0011 => write!(f, "imkey_tsm_app_delete_fail"),
            ImkeyError::NETWORK_ERROR => write!(f, "imkey_tsm_network_error"),
            ImkeyError::COS_UPGRADE_ERROR => write!(f, "imkey_tsm_cos_upgrade_error"),
        }
    }
}

//device manager error message
pub const BAPP0006: &str = "imkey_tsm_app_download_fail";
pub const BAPP0008: &str = "imkey_tsm_app_update_fail";
pub const BAPP0011: &str = "imkey_tsm_app_delete_fail";
pub const BSE0015: &str = "imkey_tsm_device_active_fail";
pub const BSE0018: &str = "imkey_tsm_device_update_check_fail";
pub const BSE0009: &str = "imkey_tsm_device_authenticity_check_fail";

