use crate::Result;
use common::path::check_path_validity;

pub struct TezosAddress();

impl TezosAddress {
    pub fn get_xpub(path: &str) -> Result<String> {
        //path check
        check_path_validity(path)?;
    }
}
