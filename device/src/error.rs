use std::error;
use std::fmt;
use std::fmt::Display;
use serde::export::Formatter;


pub enum ImkeyError{
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
}

impl Display for ImkeyError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        }
    }
}
