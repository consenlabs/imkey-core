pub mod app_delete;
pub mod app_download;
pub mod app_update;
pub mod se_activate;
pub mod se_query;
pub mod se_secure_check;
extern crate common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
