pub mod transaction;
pub mod usdt_transaction;
pub mod common;
pub mod address;
pub mod btcapi;
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
