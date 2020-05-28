pub mod apdu;
pub mod applet;
pub mod constants;
pub mod error;
pub mod https;
pub mod path;
pub mod utility;

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
