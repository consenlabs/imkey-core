pub mod app_delete;
pub mod app_download;
pub mod app_update;
pub mod se_activate;
pub mod se_query;
pub mod se_secure_check;
pub mod device_binding;
extern crate common;
pub mod manager;
pub mod device_manager;
pub mod key_manager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
