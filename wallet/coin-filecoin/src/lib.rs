pub mod address;
pub mod filecoinapi;
pub mod transaction;
pub mod utils;
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
