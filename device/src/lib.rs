pub mod app_delete;
pub mod app_download;
pub mod app_update;
pub mod device_binding;
pub mod se_activate;
pub mod se_query;
pub mod se_secure_check;
pub mod auth_code_storage;
pub mod device_cert_check;
extern crate common;
pub mod device_manager;
pub mod key_manager;
pub mod manager;
pub mod hid_api;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
