pub mod address;
use core::result;
pub type Result<T> = result::Result<T, failure::Error>;
extern crate common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
