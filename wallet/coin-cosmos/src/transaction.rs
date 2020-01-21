use mq::message;

#[derive(Debug)]
pub struct CosmosTransaction {
    pub path: String,
    pub sign_datas: String,//json string
}

impl CosmosTransaction{
//    pub fn sign(&mut self) -> Result<EosSignResult, Error> {
//        path::check_path_validity(&self.path);
//
//        let select_apdu = EosApdu::select_applet();
//        let select_response = message::send_apdu(select_apdu);
//        //todo: check select response
//
//        let eos_tx = EosSignResult {
//            hash: "".to_string(),
//            signs: vec![],
//        };
//
//        for signdata in &self.sign_datas {
//            let mut tx_hash_pack: Vec<u8> = Vec::new();
//            //            let hash_data = signdata.chain_id.to_owned() + &signdata.tx_hash;
//            //            tx_hash_pack.put_slice(hash_data.as_bytes());
//            tx_hash_pack.put_slice(signdata.chain_id.as_bytes());
//            tx_hash_pack.put_slice(signdata.tx_hash.as_bytes());
//            let context_free_actions = [0; 32];
//            tx_hash_pack.put_slice(&context_free_actions);
//            let tx_hash_sha256 = sha256::Hash::from_slice(&tx_hash_pack).unwrap();
//            let tx_hash = "0120".to_owned() + &hex::encode(&tx_hash_sha256);
//
//            let mut tx_pack: Vec<u8> = Vec::new();
//
//            //sha256(chainid + txhash + contextFreeActions)
//            tx_pack.put_slice(tx_hash.as_bytes());
//
//            //path
//            let path_hex = "0211".to_owned() + &hex::encode(&self.path);
//            tx_pack.put_slice(path_hex.as_bytes());
//
//            //payment todo: check payment is null
//            let pay_bytes = signdata.payment.as_bytes();
//            let pay_headr = "07".to_owned() + &pay_bytes.to_hex();
//            tx_pack.put_slice(pay_headr.as_bytes());
//            tx_pack.put_slice(pay_bytes);
//
//            //to
//            let to_bytes = signdata.to.as_bytes();
//            let to_headr = "08".to_owned() + &to_bytes.to_hex();
//            tx_pack.put_slice(to_headr.as_bytes());
//            tx_pack.put_slice(to_bytes);
//
//            let tx_sha256 = sha256::Hash::from_slice(&tx_pack).unwrap();
//            let tx_sh256 = sha256::Hash::from(tx_sha256);
//
//            // todo: impl ec sign hash
//        }
//
//        let ss = &self.path;
//
//        Ok(eos_tx)
//    }
}