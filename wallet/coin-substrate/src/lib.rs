pub mod address;
pub mod signer;
pub mod transaction;
use core::result;
pub type Result<T> = result::Result<T, failure::Error>;
extern crate common;

pub use address::SubstrateAddress;
pub use transaction::{
    ExportSubstrateKeystoreResult, SubstrateKeystoreParam, SubstrateRawTxIn, SubstrateTxOut,
};
pub(crate) const SIGNATURE_TYPE_SR25519: u8 = 0x01;
pub(crate) const PAYLOAD_HASH_THRESHOLD: usize = 256;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
