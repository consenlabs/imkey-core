use crate::address::TronAddress;
use crate::tronapi::{TronMessageSignReq, TronMessageSignRes, TronTxReq, TronTxRes};
use crate::Result;
use common::apdu::{ApduCheck, CoinCommonApdu, Secp256k1Apdu};
use common::error::CoinError;
use common::path::check_path_validity;
use common::utility::{is_valid_hex, secp256k1_sign, sha256_hash};
use common::{constants, utility};
use device::device_binding::KEY_MANAGER;
use secp256k1::{self, Message as SecpMessage, Signature as SecpSignature};
use transport::message::{send_apdu, send_apdu_timeout};
use device::key_manager::KeyManager;

#[derive(Debug)]
pub struct TronSigner {}

impl TronSigner {
    pub fn sign_message(input: TronMessageSignReq) -> Result<TronMessageSignRes> {
        check_path_validity(&input.path).unwrap();

        let message = match input.is_hex {
            true => {
                let mut raw_hex: String = input.message.to_owned();
                if raw_hex.to_uppercase().starts_with("0X") {
                    raw_hex.replace_range(..2, "")
                }
                hex::decode(&raw_hex)?
            }
            false => input.message.into_bytes(),
        };
        let header = match input.is_tron_header {
            true => "\x19TRON Signed Message:\n32".as_bytes(),
            false => "\x19Ethereum Signed Message:\n32".as_bytes(),
        };
        let mut msg_with_header = Vec::new();
        msg_with_header.extend(header);
        msg_with_header.extend(&message);

        let mut data_pack = Vec::new();

        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let msg_sig = secp256k1_sign(&key_manager_obj.pri_key, &msg_with_header)?;
        data_pack.push(0x00);
        data_pack.push(msg_sig.len() as u8);
        data_pack.extend(msg_sig);

        let hash = tiny_keccak::keccak256(&msg_with_header);
        data_pack.push(0x01);
        data_pack.push(hash.len() as u8);
        data_pack.extend(&hash);

        let path = input.path.as_bytes();
        data_pack.push(0x02);
        data_pack.push(path.len() as u8);
        data_pack.extend(path);

        data_pack.push(0x00);

        drop(key_manager_obj);
        let signature = TronSigner::sign(&input.path, &data_pack, &hash, &input.address)?;
        Ok(TronMessageSignRes { signature })
    }

    pub fn sign_transaction(input: TronTxReq) -> Result<TronTxRes> {
        check_path_validity(&input.path).unwrap();

        let mut data_pack = Vec::new();

        let raw_data = hex::decode(input.raw_data)?;
        let hash = sha256_hash(&raw_data);
        data_pack.push(0x01);
        data_pack.push(hash.len() as u8);
        data_pack.extend(&hash);

        let path = input.path.as_bytes();
        data_pack.push(0x02);
        data_pack.push(path.len() as u8);
        data_pack.extend(path);

        let payment = input.payment.as_bytes();
        data_pack.push(0x07);
        data_pack.push(payment.len() as u8);
        data_pack.extend(payment);

        let to = input.to.as_bytes();
        data_pack.push(0x08);
        data_pack.push(to.len() as u8);
        data_pack.extend(to);

        println!("pack data");
        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let data_pack_sig = secp256k1_sign(&key_manager_obj.pri_key, &data_pack)?;
        drop(key_manager_obj);

        println!("pack data signed..");
        let mut data_pack_with_sig = Vec::new();
        data_pack_with_sig.push(0x00);
        data_pack_with_sig.push(data_pack_sig.len() as u8);
        data_pack_with_sig.extend(&data_pack_sig);
        data_pack_with_sig.extend(&data_pack);

        println!("pack data with sig");
        let signature = TronSigner::sign(&input.path, &data_pack_with_sig, &hash, &input.address)?;
        Ok(TronTxRes { signature })
    }

    pub fn sign(path: &str, data_pack: &[u8], hash: &[u8], sender: &str) -> Result<String> {
        println!("slect tron applet....");
        let select_apdu = Secp256k1Apdu::select_applet();
        let select_result = send_apdu(select_apdu)?;
        ApduCheck::checke_response(&select_result)?;


        let key_manager_obj = KEY_MANAGER.lock().unwrap();
        let path_signature = secp256k1_sign(&key_manager_obj.pri_key, &path.as_bytes())?;
        let mut path_pack: Vec<u8> = vec![];
        path_pack.push(0x00);
        path_pack.push(path_signature.len() as u8);
        path_pack.extend(path_signature.as_slice());
        path_pack.push(0x01);
        path_pack.push(path.as_bytes().len() as u8);
        path_pack.extend(path.as_bytes());

        let msg_pubkey = Secp256k1Apdu::get_xpub(&path_pack);
        println!("msg_pubkey:{}",&msg_pubkey);
        let res_msg_pubkey = send_apdu(msg_pubkey)?;
        println!("res_msg_pubkey:{}",&res_msg_pubkey);
        let pubkey_raw = hex::decode(&res_msg_pubkey[..130]).unwrap();
        let address = TronAddress::address_from_pubkey(pubkey_raw.as_slice()).unwrap();
        println!("address y...");
        if &address != sender {
            println!("mismatch...");
            return Err(CoinError::ImkeyAddressMismatchWithPath.into());
        }

        println!("address checked...");
        let mut sign_response = "".to_string();
        let sign_apdus = Secp256k1Apdu::sign(data_pack);
        for apdu in sign_apdus {
            println!("apdu aaaaaaa:{}",apdu);
            sign_response = send_apdu_timeout(apdu, constants::TIMEOUT_LONG)?;
            ApduCheck::checke_response(&sign_response)?;
        }
        println!("signed...");

        let sign_compact = hex::decode(&sign_response[2..130]).unwrap();
        let mut signnture_obj = SecpSignature::from_compact(sign_compact.as_slice()).unwrap();
        signnture_obj.normalize_s();
        let normalizes_sig_vec = signnture_obj.serialize_compact();

        let rec_id = utility::retrieve_recid(&hash, &normalizes_sig_vec, &pubkey_raw).unwrap();
        let rec_id = rec_id.to_i32();
        let v = rec_id + 27;

        let mut signature = hex::encode(&normalizes_sig_vec.as_ref());
        signature.push_str(&format!("{:02x}", &v));

        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use crate::signer::TronSigner;
    use crate::tronapi::{TronMessageSignReq, TronTxReq};
    use bitcoin::util::misc::hex_bytes;
    use common::constants;
    use device::device_binding::bind_test;

    #[test]
    fn sign_message() {
        bind_test();

        let input = TronMessageSignReq {
            path: constants::TRON_PATH.to_string(),
            message: "645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76".to_string(),
            address: "TY2uroBeZ5trA9QT96aEWj32XLkAAhQ9R2".to_string(),
            is_hex: true,
            is_tron_header: true,
        };
        let res = TronSigner::sign_message(input).unwrap();
        assert_eq!("7209610445e867cf2a36ea301bb5d1fbc3da597fd2ce4bb7fa64796fbf0620a4175e9f841cbf60d12c26737797217c0082fdb3caa8e44079e04ec3f93e86bbea1c", hex::encode(&res.signature))
    }

    #[test]
    fn sign_transaction() {
        bind_test();

        let input = TronTxReq{
            path: constants::TRON_PATH.to_string(),
            raw_data: "0a0208312208b02efdc02638b61e40f083c3a7c92d5a65080112610a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412300a1541a1e81654258bf14f63feb2e8d1380075d45b0dac1215410b3e84ec677b3e63c99affcadb91a6b4e086798f186470a0bfbfa7c92d".to_string(),
            address: "TY2uroBeZ5trA9QT96aEWj32XLkAAhQ9R2".to_string(),
            payment: "100 TRX".to_string(),
            to: "TDQqJsFsStSy5fjG52KuiWW7HhJGAKGJLb".to_string()
        };
        let res = TronSigner::sign_transaction(input).unwrap();
        assert_eq!("beac4045c3ea5136b541a3d5ec2a3e5836d94f28a1371440a01258808612bc161b5417e6f5a342451303cda840f7e21bfaba1011fad5f63538cb8cc132a9768800", hex::encode(&res.signature))
    }
}
