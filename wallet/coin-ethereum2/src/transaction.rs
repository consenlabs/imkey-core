use crate::Result;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Eth2Sign {}

impl Eth2Sign {
    pub fn get_address(path: &str) -> Result<String> {
        Ok("".to_string())
    }
}
