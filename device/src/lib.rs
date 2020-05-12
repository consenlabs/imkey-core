pub mod app_delete;
pub mod app_download;
pub mod app_update;
pub mod auth_code_storage;
pub mod device_binding;
pub mod device_cert_check;
pub mod se_activate;
pub mod se_query;
pub mod se_secure_check;
extern crate common;
pub mod cos_upgrade;
pub mod device_manager;
pub mod deviceapi;
pub mod key_manager;
pub mod manager;
#[macro_use]
extern crate lazy_static;
extern crate mq;
pub mod error;
#[macro_use]
extern crate failure;
use core::result;
pub type Result<T> = result::Result<T, failure::Error>;
use serde::{Deserialize, Serialize};
pub mod cos_check_update;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceResponse<T> {
    pub _ReturnCode: String,
    pub _ReturnMsg: String,
    pub _ReturnData: T,
}

pub trait TsmService {
    type ReturnData;
    fn send_message(&mut self) -> Result<Self::ReturnData>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
