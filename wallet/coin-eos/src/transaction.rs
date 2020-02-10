use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{sha256, Hash, ripemd160};
use bytes::BufMut;
use common::apdu::EosApdu;
use common::error::Error;
use common::{path, utility};
use mq::message;
use std::io::Read;
use common::utility::{sha256_hash, hex_to_bytes, secp256k1_sign, secp256k1_sign_hash, retrieve_recid};
use mq::message::send_apdu;
use bitcoin::util::base58;
use bitcoin::secp256k1::Signature;
use hex::FromHex;
use common::eosapi::{EosTxInput, EosTxOutput};

#[derive(Debug)]
pub struct EosTransaction {}

impl EosTransaction {
    pub fn sign_tx(tx_input:EosTxInput) -> Result<EosTxOutput, Error> {
        path::check_path_validity(&tx_input.path);

        let select_apdu = EosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu);
        //todo: check select response

        let mut tx_output = EosTxOutput {
            hash: "".to_string(),
            signs: vec![],
        };

        for sign_data in &tx_input.sign_datas {
            //tx hash
            let tx_data_bytes = hex::decode(&sign_data.tx_data).unwrap();
            let tx_hash = sha256_hash(&tx_data_bytes).to_hex();
            println!("tx_hash:{}", &tx_hash);
            tx_output.hash = tx_hash;


            //pack tx data
            let mut tx_data_pack: Vec<u8> = Vec::new();
            tx_data_pack.put_slice(hex::decode(&sign_data.chain_id).unwrap().as_slice());
            tx_data_pack.put_slice(hex::decode(&sign_data.tx_data).unwrap().as_slice());
            let context_free_actions = [0; 32];
            tx_data_pack.put_slice(&context_free_actions);
            println!("tx_data_pack:{}", &hex::encode(&tx_data_pack));

            //tx data hash
            let tx_data_hash = sha256_hash(&tx_data_pack);
            println!("tx_data_hash:{}", &hex::encode(&tx_data_hash));

            //view_info
            let mut view_info = "".to_string();
            view_info.push_str("07");
            view_info.push_str(&format!("{:02x}", &sign_data.payment.as_bytes().len()));
            view_info.push_str(&hex::encode(&sign_data.payment));
            view_info.push_str("08");
            view_info.push_str(&format!("{:02x}", &sign_data.to.as_bytes().len()));
            view_info.push_str(&hex::encode(&sign_data.to));
            println!("view_info:{}", &view_info);

            //sign
            for pub_key in &sign_data.pub_keys {
                let mut sign_data_pack: Vec<u8> = Vec::new();
                sign_data_pack.push(0x01);
                sign_data_pack.push(tx_data_hash.len() as u8);//hash len
                sign_data_pack.extend(tx_data_hash.iter());
                sign_data_pack.push(0x02);
                sign_data_pack.push(tx_input.path.len() as u8);//hash len
                sign_data_pack.extend(tx_input.path.as_bytes());
                sign_data_pack.extend(hex::decode(&view_info).unwrap().as_slice());
                println!("sign_data_pack:{}", &hex::encode(&sign_data_pack));

                //hash twice
                let sign_data_hash = sha256_hash(&sha256_hash(&sign_data_pack));
                println!("sign_data_hash:{}", &hex::encode(&sign_data_hash));


                //bind signature
                let private_key = hex_to_bytes("7CD950180EDFF1C4A21270AD293A274580D20C84DE06666467F6386FB7DDA352").unwrap();//ios
                let mut bind_signature = secp256k1_sign_hash(&private_key, &sign_data_hash);
                println!("bind_signature:{}", &hex::encode(&bind_signature));

                //send prepare data
                let mut prepare_apdu_data: Vec<u8> = Vec::new();
                prepare_apdu_data.push(0x00);
                prepare_apdu_data.push(bind_signature.len() as u8);
                prepare_apdu_data.extend(bind_signature.iter());
                prepare_apdu_data.extend(sign_data_pack.iter());
                println!("prepare_apdu_data:{}", &hex::encode(&prepare_apdu_data));

                let prepare_apdus = EosApdu::prepare_sign(prepare_apdu_data);
                let mut prepare_result = "".to_string();
                for prepare_apdu in prepare_apdus {
                    prepare_result = send_apdu(prepare_apdu);
                    //todo checkresponse
                }
                println!("prepare_result:{}", &prepare_result);

                //check pub key
                let mut signature = "".to_string();
                let uncomprs_pubkey: String = prepare_result.chars().take(prepare_result.len() - 4).collect();
                let comprs_pubkey = utility::uncompress_pubkey_2_compress(&uncomprs_pubkey);
                let mut comprs_pubkey_slice = hex::decode(comprs_pubkey).expect("Decoding failed");
                let pubkey_hash = ripemd160::Hash::hash(&comprs_pubkey_slice);
                let check_sum = &pubkey_hash[0..4];
                comprs_pubkey_slice.extend(check_sum);
                let eos_pk = "EOS".to_owned() + base58::encode_slice(&comprs_pubkey_slice).as_ref();
                if pub_key == &eos_pk {
                    println!("eos_pk eq");

                    //sign
                    let mut nonce = 0;
                    loop {
                        let sign_apdu = EosApdu::sign_tx(nonce);
                        println!("sign_apdu:{}", &sign_apdu);
                        let sign_result = send_apdu(sign_apdu);
                        println!("sign_reuslt:{}", &sign_result);

                        let sign_result_vec = Vec::from_hex(&sign_result[2..sign_result.len() - 6]).unwrap();
                        let mut signature_obj = Signature::from_compact(sign_result_vec.as_slice()).unwrap();
                        //generator der sign data
                        signature_obj.normalize_s();
                        let mut signatrue_der = signature_obj.serialize_der().to_vec();
                        println!("sign_result_vec:{}", &hex::encode(&sign_result_vec));

                        let len_r = signatrue_der[3];
                        let len_s = signatrue_der[5 + len_r as usize];
                        if len_r == 32 && len_s ==32 {
                            let r = &sign_result[2..66];
                            let s = &sign_result[66..130];

                            //calc v
                            let pub_key_raw = hex::decode(&uncomprs_pubkey).unwrap();
                            let sign_compact = hex::decode(&sign_result[2..130]).unwrap();
                            let rec_id = retrieve_recid(&tx_data_hash, &sign_compact, &pub_key_raw)?;
                            let rec_id = rec_id.to_i32();
                            println!("rec_id:{}", &rec_id);
                            let v = rec_id + 27 + 4;

                            signature.push_str(&format!("{:02X}", &v));
                            signature.push_str(r);
                            signature.push_str(s);
                            println!("signature:{}", &signature);
                            break;
                        }
                        nonce = nonce + 1;
                    }

                    //checksum base58
                    let mut to_hash = hex::decode(&signature).unwrap();
                    to_hash.put_slice( "K1".as_bytes());
                    let signature_hash = ripemd160::Hash::hash(&to_hash);
                    let check_sum = &signature_hash[0..4];

                    let mut signature_slice = hex::decode(&signature).unwrap();
                    signature_slice.extend(check_sum);
                    let sigature_base58 = "SIG_K1_".to_owned() + base58::encode_slice(&signature_slice).as_ref();
                    println!("sigature_base58:{}", &sigature_base58);
                    tx_output.signs.push(sigature_base58);
                }

            }
        }

        Ok(tx_output)
    }
}

#[cfg(test)]
mod tests {
    use common::constants;
    use common::eosapi::{EosTxInput, EosSignData};
    use crate::transaction::EosTransaction;

    #[test]
    fn test_sgin_tx() {
        let eos_sign_data = EosSignData{
            tx_data: "c578065b93aec6a7c811000000000100a6823403ea3055000000572d3ccdcd01000000602a48b37400000000a8ed323225000000602a48b374208410425c95b1ca80969800000000000453595300000000046d656d6f00".to_string(),
            pub_keys: vec!["EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF".to_string()],
            chain_id: "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906".to_string(),
            to: "bbbb5555bbbb".to_string(),
            from: "liujianmin12".to_string(),
            payment: "undelegatebw 0.0100 EOS".to_string()
        };

        let mut eox_tx_input = EosTxInput{
            path: constants::EOS_PATH.to_string(),
            sign_datas: vec![eos_sign_data]
        };

        let result = EosTransaction::sign_tx(eox_tx_input).unwrap();
//        println!("result:{}",result);
    }
}
