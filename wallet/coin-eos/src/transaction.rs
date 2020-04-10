use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{Hash, ripemd160};
use bytes::BufMut;
use common::apdu::{EosApdu, ApduCheck};
use common::{path, utility, constants};
use mq::message;
use common::utility::{sha256_hash, secp256k1_sign, secp256k1_sign_hash, retrieve_recid};
use mq::message::{send_apdu, send_apdu_timeout};
use bitcoin::util::base58;
use bitcoin::secp256k1::Signature;
use hex::FromHex;
use crate::eosapi::{EosTxReq, EosTxRes, EosMessageSignReq, EosMessageSignRes, EosSignResult};
use crate::pubkey::EosPubkey;
use crate::Result;
use device::device_binding::KEY_MANAGER;

#[derive(Debug)]
pub struct EosTransaction {}

impl EosTransaction {

    pub fn sign_tx(tx_input:EosTxReq) -> Result<EosTxRes> {
        path::check_path_validity(&tx_input.path).unwrap();

        let select_apdu = EosApdu::select_applet();
        let select_response = message::send_apdu(select_apdu)?;
        ApduCheck::checke_response(&select_response)?;

        let mut trans_multi_signs:Vec<EosSignResult> = Vec::new();

        for sign_data in &tx_input.sign_datas {
            let mut sign_result = EosSignResult{
                hash: "".to_string(),
                signs: vec![]
            };
            //tx hash
            let tx_data_bytes = hex::decode(&sign_data.tx_data).unwrap();
            let tx_hash = sha256_hash(&tx_data_bytes).to_hex();
            sign_result.hash = tx_hash;


            //pack tx data
            let mut tx_data_pack: Vec<u8> = Vec::new();
            tx_data_pack.put_slice(hex::decode(&sign_data.chain_id).unwrap().as_slice());
            tx_data_pack.put_slice(hex::decode(&sign_data.tx_data).unwrap().as_slice());
            let context_free_actions = [0; 32];
            tx_data_pack.put_slice(&context_free_actions);

            //tx data hash
            let tx_data_hash = sha256_hash(&tx_data_pack);

            //view_info
            let mut view_info = "".to_string();
            view_info.push_str("07");
            view_info.push_str(&format!("{:02x}", &sign_data.payment.as_bytes().len()));
            view_info.push_str(&hex::encode(&sign_data.payment));
            view_info.push_str("08");
            view_info.push_str(&format!("{:02x}", &sign_data.to.as_bytes().len()));
            view_info.push_str(&hex::encode(&sign_data.to));

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

                //hash twice
                let sign_data_hash = sha256_hash(&sha256_hash(&sign_data_pack));


                //bind signature
                let key_manager_obj = KEY_MANAGER.lock().unwrap();
                let bind_signature = secp256k1_sign_hash(&key_manager_obj.pri_key, &sign_data_hash)?;

                //send prepare data
                let mut prepare_apdu_data: Vec<u8> = Vec::new();
                prepare_apdu_data.push(0x00);
                prepare_apdu_data.push(bind_signature.len() as u8);
                prepare_apdu_data.extend(bind_signature.iter());
                prepare_apdu_data.extend(sign_data_pack.iter());

                let prepare_apdus = EosApdu::prepare_sign(prepare_apdu_data);
                let mut prepare_result = "".to_string();
                for prepare_apdu in prepare_apdus {
                    prepare_result = send_apdu_timeout(prepare_apdu,constants::TIMEOUT_LONG)?;
                    ApduCheck::checke_response(&prepare_result)?;
                }

                //check pub key
                let mut signature = "".to_string();
                let uncomprs_pubkey: String = prepare_result.chars().take(prepare_result.len() - 4).collect();
                let comprs_pubkey = utility::uncompress_pubkey_2_compress(&uncomprs_pubkey);
                let mut comprs_pubkey_slice = hex::decode(comprs_pubkey)?;
                let pubkey_hash = ripemd160::Hash::hash(&comprs_pubkey_slice);
                let check_sum = &pubkey_hash[0..4];
                comprs_pubkey_slice.extend(check_sum);
                let eos_pk = "EOS".to_owned() + base58::encode_slice(&comprs_pubkey_slice).as_ref();
                if pub_key != &eos_pk{
                    return Err(format_err!("imkey_publickey_mismatch_with_path"));
                }

                //sign
                let mut nonce = 0;
                loop {
                    let sign_apdu = EosApdu::sign_tx(nonce);
                    let sign_result = send_apdu(sign_apdu)?;
                    ApduCheck::checke_response(&sign_result)?;

                    let sign_result_vec = Vec::from_hex(&sign_result[2..sign_result.len() - 6]).unwrap();
                    let mut signature_obj = Signature::from_compact(sign_result_vec.as_slice()).unwrap();
                    //generator der sign data
                    signature_obj.normalize_s();
                    let signatrue_der = signature_obj.serialize_der().to_vec();

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
                        let v = rec_id + 27 + 4;

                        signature.push_str(&format!("{:02X}", &v));
                        signature.push_str(r);
                        signature.push_str(s);
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
                sign_result.signs.push(sigature_base58);

                trans_multi_signs.push(sign_result.clone());
            }
        }

        let tx_output = EosTxRes {
            trans_multi_signs
        };
        Ok(tx_output)
    }

    pub fn sign_message(input:EosMessageSignReq) -> Result<EosMessageSignRes>{
        let hash = if input.is_hex {
            hex::decode(input.data).unwrap()
        }else{
            sha256_hash(input.data.as_bytes())
        };

        let mut data_pack: Vec<u8>  = Vec::new();
        data_pack.push(0x01);
        data_pack.push(0x20);
        data_pack.extend(hash.as_slice());
        data_pack.push(0x02);
        data_pack.push(input.path.as_bytes().len() as u8);
        data_pack.extend(input.path.as_bytes());

        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let bind_signature = secp256k1_sign(&key_manager_obj.pri_key, &data_pack).unwrap();

        let mut prepare_pack: Vec<u8>  = Vec::new();
        prepare_pack.push(0x00);
        prepare_pack.push(bind_signature.len() as u8);
        prepare_pack.extend(bind_signature.iter());
        prepare_pack.extend(data_pack.iter());

        let select_apdu = EosApdu::select_applet();
        let select_response = send_apdu(select_apdu)?;
        ApduCheck::checke_response(&select_response)?;

        let prepare_apdus = EosApdu::prepare_message_sign(prepare_pack);

        let mut prepare_response = "".to_string();
        for apdu in prepare_apdus {
            prepare_response = send_apdu_timeout(apdu,constants::TIMEOUT_LONG)?;
            ApduCheck::checke_response(&prepare_response)?;
        }


        //todo optmize,calc from prepare response
        let pubkey = EosPubkey::pubkey_from_response(&prepare_response).unwrap();
        let mut signature = "".to_string();
        if &pubkey != &input.pubkey{
            return Err(format_err!("imkey_publickey_mismatch_with_path"));
        }
        //sign
        let mut nonce = 0;
        loop {
            let sign_apdu = EosApdu::sign_message(nonce);
            let sign_result = send_apdu(sign_apdu)?;
            ApduCheck::checke_response(&sign_result)?;

            let sign_result_vec = Vec::from_hex(&sign_result[2..sign_result.len() - 6]).unwrap();
            let mut signature_obj = Signature::from_compact(sign_result_vec.as_slice()).unwrap();
            //generator der sign data
            signature_obj.normalize_s();
            let signatrue_der = signature_obj.serialize_der().to_vec();

            let len_r = signatrue_der[3];
            let len_s = signatrue_der[5 + len_r as usize];
            if len_r == 32 && len_s ==32 {
                let r = &sign_result[2..66];
                let s = &sign_result[66..130];

                //calc v
                let uncomprs_pubkey: String = prepare_response.chars().take(prepare_response.len() - 4).collect();
                let pub_key_raw = hex::decode(&uncomprs_pubkey).unwrap();
                let sign_compact = hex::decode(&sign_result[2..130]).unwrap();
                let rec_id = utility::retrieve_recid(&hash, &sign_compact, &pub_key_raw).unwrap();
                let rec_id = rec_id.to_i32();
                let v = rec_id + 27 + 4;

                signature.push_str(&format!("{:02X}", &v));
                signature.push_str(r);
                signature.push_str(s);
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
        let signature = "SIG_K1_".to_owned() + base58::encode_slice(&signature_slice).as_ref();

        let output = EosMessageSignRes{
            signature
        };
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use common::constants;
    use crate::eosapi::{EosTxReq, EosSignData, EosMessageSignReq};
    use crate::transaction::EosTransaction;
    use device::device_binding::DeviceManage;

    #[test]
    fn test_sgin_tx() {
        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let check_result = DeviceManage::bind_check(&path).unwrap();
        println!("check_result:{}",&check_result);

        let eos_sign_data = EosSignData{
            tx_data: "c578065b93aec6a7c811000000000100a6823403ea3055000000572d3ccdcd01000000602a48b37400000000a8ed323225000000602a48b374208410425c95b1ca80969800000000000453595300000000046d656d6f00".to_string(),
            pub_keys: vec!["EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF".to_string()],
            chain_id: "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906".to_string(),
            to: "bbbb5555bbbb".to_string(),
            from: "liujianmin12".to_string(),
            payment: "undelegatebw 0.0100 EOS".to_string()
        };

        let mut eox_tx_input = EosTxReq{
            path: constants::EOS_PATH.to_string(),
            sign_datas: vec![eos_sign_data]
        };

        let result = EosTransaction::sign_tx(eox_tx_input).unwrap();
        println!("hash:{}",result.trans_multi_signs[0].hash);
    }

    #[test]
    fn test_sign_messgage(){
        let path = "/Users/joe/work/sdk_gen_key".to_string();
        let check_result = DeviceManage::bind_check(&path).unwrap();
        println!("check_result:{}",&check_result);

        let input = EosMessageSignReq{
            path: constants::EOS_PATH.to_string(),
            data: "imKey2019".to_string(),
            is_hex: false,
            pubkey: "EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF".to_string()
        };

        let output = EosTransaction::sign_message(input);
    }
}