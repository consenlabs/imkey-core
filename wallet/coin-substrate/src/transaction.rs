use crate::substrateapi::{SubstrateRawTxIn, SubstrateTxOut};
use crate::Result;
use crate::{PAYLOAD_HASH_THRESHOLD, SIGNATURE_TYPE_SR25519};
use common::SignParam;
use sp_core::blake2_256;

#[derive(Debug)]
pub struct Transaction {}

impl Transaction {
    pub fn hash_unsigned_payload(payload: &[u8]) -> Result<Vec<u8>> {
        if payload.len() > PAYLOAD_HASH_THRESHOLD {
            Ok(blake2_256(&payload).to_vec())
        } else {
            Ok(payload.to_vec())
        }
    }

    pub fn sign_transaction(
        tx: &SubstrateRawTxIn,
        sign_param: &SignParam,
    ) -> Result<SubstrateTxOut> {
        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };
        let raw_data_bytes = hex::decode(&raw_data_bytes)?;
        let hash = Transaction::hash_unsigned_payload(&raw_data_bytes)?;

        // let sig = sign_recoverable_hash(&hash, symbol, address, None)?;

        let sig_with_type = [vec![SIGNATURE_TYPE_SR25519], vec![SIGNATURE_TYPE_SR25519]].concat();

        let tx_out = SubstrateTxOut {
            signature: format!("0x{}", hex::encode(sig_with_type)),
        };
        Ok(tx_out)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_sign_transaction() {
        // let message: &[u8] = b"616263";
        // let pub_key =
        //     hex::decode("50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859")
        //         .expect("hex decode error");
        // let address = SubstrateAddress::from_public_key(&pub_key, AddressType::Polkadot)
        //     .expect("invalid public key");
        // assert_eq!("12pWV6LvG4iAfNpFNTvvkWy3H9H8wtCkjiXupAzo2BCmPViM", address);
        //
        // let pub_key =
        //     hex::decode("50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859")
        //         .expect("hex decode error");
        // let address = SubstrateAddress::from_public_key(&pub_key, AddressType::Kusama)
        //     .expect("invalid public key");
        // assert_eq!("EPq15Rj2eTcyVdBBXgyWKVta7Zj4FTo7beB3YHPwtPjxEkr", address);
    }
}
