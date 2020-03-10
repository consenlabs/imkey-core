pub mod apdu;
pub mod constants;
pub mod error;
pub mod https;
pub mod path;
pub mod sign_res;
pub mod utility;
pub mod eosapi;
pub mod cosmosapi;
pub mod ethapi;
pub mod applet;

#[macro_use]
extern crate failure;
use core::result;
pub type Result<T> = result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
