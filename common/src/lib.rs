pub mod apdu;
pub mod constants;
pub mod error;
pub mod https;
pub mod sign_res;
pub mod utility;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
