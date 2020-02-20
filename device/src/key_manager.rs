use base64::{decode, encode};
use ring::digest;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

extern crate aes_soft as aes;
extern crate block_modes;
extern crate hex_literal;

use aes_soft::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use self::aes::Aes256;
use hex::FromHex;
use secp256k1::Secp256k1;
use secp256k1::key::{SecretKey, PublicKey};
use rand::{RngCore, thread_rng};
use std::str::FromStr;

use lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref SE_PUB_KEY: Mutex<String> = Mutex::new("".to_string());
    pub static ref LOCL_PRI_KEY: Mutex<String> = Mutex::new("".to_string());
}


pub struct KeyManager {
    pub pri_key: Vec<u8>,//32 byte
    pub pub_key: Vec<u8>,//65 byte
    pub se_pub_key: Vec<u8>,//65 byte
    pub session_key: Vec<u8>,//16 byte
    pub check_sum: Vec<u8>,//4 byte
    pub encry_key: Vec<u8>,//16 byte
    pub iv: Vec<u8>,//16 byte
}

impl KeyManager {
    pub fn new() -> KeyManager {
        KeyManager {
            pri_key: vec![],
            pub_key: vec![],
            se_pub_key: vec![],
            session_key: vec![],
            check_sum: vec![],
            encry_key: vec![],
            iv: vec![],
        }
    }
    /**
    生成加密密钥
    */
    pub fn gen_encrypt_key(&mut self, seid: &str, sn: &str) {
        //calc seid and sn hash
        let seid_hash = digest::digest(&digest::SHA256, seid.as_bytes()).as_ref().to_vec();
        let sn_hash = digest::digest(&digest::SHA256, sn.as_bytes()).as_ref().to_vec();

        let mut xor_result : Vec<u8> = vec![];
        for (index, value) in seid_hash.iter().enumerate() {
            xor_result.push(value ^ sn_hash.get(index).unwrap());
        }
        self.encry_key = xor_result[..16].to_vec();
        self.iv = xor_result[16..].to_vec();
    }

    /**
    加密密钥文件数据
    */
    pub fn encrypt_data(&self) -> String {
        let mut data = vec![];
        //组织原数据
        data.extend(self.pri_key.iter());
        data.extend(self.pub_key.iter());
        data.extend(self.se_pub_key.iter());
        data.extend(self.session_key.iter());

        //计算HASH
        let hash = digest::digest(&digest::SHA256, data.as_slice());
        data.extend(hash.as_ref()[..4].iter());

        //进行AES-CBC加密
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        let cipher =
            Aes128Cbc::new_var(self.encry_key.as_ref(), self.iv.as_ref()).expect("aes cbc encrypt error");
        let ciphertext = cipher.encrypt_vec(data.as_ref());

        //base64编码
        encode(&ciphertext)
    }

    /**
    获取密钥文件数据
    */
    pub fn get_key_file_data(path: &String, seid: &String) -> String {
        let mut return_data = String::new();
        println!("{}", format!("{}key{}{}", path, seid, ".txt"));
        let file = File::open(format!("{}key{}{}", path, seid, ".txt").as_str());
        match file {
            Ok(mut f) => {
                f.read_to_string(&mut return_data).expect("read file error");
                return_data
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return_data,
                _ => panic!("open file error"),
            },
        }
    }

    /**
    解密密钥文件数据
    */
    pub fn decrypt_keys(&mut self, ciphertext: &[u8]) -> bool {
        //base64解码
        let ciphertext_bytes = decode(ciphertext).unwrap();

        //AES CBC解密
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        let cipher =
            Aes128Cbc::new_var(self.encry_key.as_ref(), self.iv.as_ref()).unwrap();
        let decrypt_result = cipher.decrypt_vec(&ciphertext_bytes);
        if decrypt_result.is_err() {
            return false;
        }
        let decrypted_data = decrypt_result.unwrap();

        //解析明文数据
        //pri_key
        self.pri_key = decrypted_data[..32].to_vec();

        let mut temp_pri_key = LOCL_PRI_KEY.lock().unwrap();
        *temp_pri_key = hex::encode_upper(decrypted_data[..32].to_vec());
        std::mem::drop(temp_pri_key);

        //pub key
        self.pub_key = decrypted_data[32..97].to_vec();

        //se pub key
        self.se_pub_key = decrypted_data[97..162].to_vec();
        let mut temp_se_pub_key = SE_PUB_KEY.lock().unwrap();
        *temp_se_pub_key = hex::encode_upper(decrypted_data[97..162].to_vec());
        std::mem::drop(temp_se_pub_key);

        //session key
        self.session_key = decrypted_data[162..178].to_vec();

        //check sum
        self.check_sum = decrypted_data[178..].to_vec();

        //校验checksum，检验成功则返回true，否则返回false
        let mut data = &decrypted_data[..178];
        let data_hash = digest::digest(&digest::SHA256, data);
        let data_hash_byte = data_hash.as_ref();
        for (index, val) in self.check_sum.iter().enumerate() {
            if val != &data_hash_byte[index] {
                return false;
            }
        }
        return true;
    }

    /**
    生成本地密钥对
    */
    pub fn gen_local_keys(&mut self) {
        let s = Secp256k1::new();
        let (sk, pk) = s.generate_keypair(&mut thread_rng());

        let mut temp_pri_key = LOCL_PRI_KEY.lock().unwrap();
        *temp_pri_key = sk.to_string();
        std::mem::drop(temp_pri_key);

        self.pri_key = Vec::from_hex(sk.to_string()).unwrap();
        self.pub_key = pk.serialize_uncompressed().to_vec();
    }
    /**
     保存密钥倒本地文件
    */
    pub fn save_keys_to_local_file(keys: &String, path: &String, seid: &String) {

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(Path::new(format!("{}key{}{}", path, seid, ".txt").as_str())).expect("open key file error");
        file.write_all(keys.as_bytes());
        
    }
}

#[cfg(test)]
mod test{
    use crate::key_manager::KeyManager;

    #[test]
    fn gen_encrypt_key_test(){
        let seid = "19060000000200860001010000000014";
        let sn = "imKey01191200001";
        let mut key_manager_obj = KeyManager::new();
        key_manager_obj.gen_encrypt_key(&seid, &sn);
        println!("encry key-->{:?}", hex::encode_upper(&key_manager_obj.encry_key));
        println!("iv-->{:?}", hex::encode_upper(&key_manager_obj.iv));
        assert_eq!(
            hex::encode_upper(key_manager_obj.encry_key),
            "A49CDEDE0370D1543033E41A413EBC4E".to_string()
        );
        assert_eq!(
            hex::encode_upper(key_manager_obj.iv),
            "92AF372F64C10BAA942478560F91F346".to_string()
        );
    }

}
