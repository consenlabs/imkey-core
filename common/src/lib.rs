pub mod apdu;
pub mod constants;
pub mod error;
pub mod https;
pub mod path;
pub mod sign_res;
pub mod utility;
pub mod eosapi;
pub mod cosmosapi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
