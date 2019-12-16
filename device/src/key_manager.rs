use base64::{decode, encode};
use ring::digest;
use std::convert::TryInto;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

extern crate aes_soft as aes;
extern crate block_modes;
extern crate hex_literal;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use self::aes::Aes256;
use hex::FromHex;
use rand::OsRng;
use secp256k1::ecdh::SharedSecret;
use secp256k1::rand::thread_rng;
use secp256k1::{Message, Secp256k1};

pub struct KeyManager {
    pub pri_key: Option<[u8; 32]>,
    pub pub_key: Option<[u8; 65]>,
    pub se_pub_key: Option<[u8; 65]>,
    pub session_key: Option<[u8; 16]>,
    pub check_sum: Option<[u8; 4]>,
    pub encry_key: Option<[u8; 16]>,
    pub iv: Option<[u8; 16]>,
}

impl KeyManager {
    pub fn new() -> KeyManager {
        KeyManager {
            pri_key: None,
            pub_key: None,
            se_pub_key: None,
            session_key: None,
            check_sum: None,
            encry_key: None,
            iv: None,
        }
    }
    /**
    生成加密密钥
    */
    pub fn gen_encrypt_key(&mut self, seid: &String, sn: &String) {
        let seid_hash = digest::digest(&digest::SHA256, seid.as_bytes());
        let sn_hash = digest::digest(&digest::SHA256, sn.as_bytes());

        let seid_hash = seid_hash.as_ref();
        let sn_hash = sn_hash.as_ref();

        let mut result: [u8; 32] = [0x00; 32];
        for (index, value) in seid_hash.iter().enumerate() {
            result[index] = value ^ sn_hash.get(index).unwrap();
        }

        let mut temp_encry_key = [0u8; 16];
        temp_encry_key.copy_from_slice(&result[..16]);
        let mut temp_iv = [0u8; 16];
        temp_iv.copy_from_slice(&result[16..]);
        self.encry_key = Some(temp_encry_key);
        self.iv = Some(temp_iv);
    }
    /**
    加密密钥文件数据
    */
    pub fn encrypt_data(&self) -> String {
        let mut data = Vec::new();
        //组织原数据
        data.extend(self.pri_key.unwrap().iter());
        data.extend(self.pub_key.unwrap().iter());
        data.extend(self.se_pub_key.unwrap().iter());
        data.extend(self.session_key.unwrap().iter());

        //计算HASH
        let hash = digest::digest(&digest::SHA256, data.as_slice());
        data.extend(hash.as_ref()[..4].iter());

        //进行AES-CBC加密
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        let cipher =
            Aes128Cbc::new_var(self.encry_key.unwrap().as_ref(), self.iv.unwrap().as_ref())
                .unwrap();
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
            Aes128Cbc::new_var(self.encry_key.unwrap().as_ref(), self.iv.unwrap().as_ref())
                .unwrap();
        let decrypted_data = cipher.decrypt_vec(&ciphertext_bytes).unwrap();

        //解析明文数据
        //pri_key
        let mut temp_pri_key = [0u8; 32];
        temp_pri_key.copy_from_slice(&decrypted_data[..32]);
        self.pri_key = Some(temp_pri_key);
        println!("{:?}", hex::encode_upper(temp_pri_key.to_vec()));
        //pub key
        let mut temp_pub_key = [0u8; 65];
        temp_pub_key.copy_from_slice(&decrypted_data[32..97]);
        println!("{:?}", hex::encode_upper(temp_pub_key.to_vec()));
        self.pub_key = Some(temp_pub_key);

        //se pub key
        let mut temp_se_pub_key = [0u8; 65];
        temp_se_pub_key.copy_from_slice(&decrypted_data[97..162]);
        println!("{:?}", hex::encode_upper(temp_se_pub_key.to_vec()));
        self.se_pub_key = Some(temp_se_pub_key);
        //session key
        let mut temp_session_key = [0u8; 16];
        temp_session_key.copy_from_slice(&decrypted_data[162..178]);
        self.session_key = Some(temp_session_key);
        //check sum
        let mut temp_check_sum = [0u8; 4];
        temp_check_sum.copy_from_slice(&decrypted_data[178..]);
        self.check_sum = Some(temp_check_sum);

        //校验checksum，检验成功则返回true，否则返回false
        let mut data = &decrypted_data[..178];
        let data_hash = digest::digest(&digest::SHA256, data);
        let data_hash_byte = data_hash.as_ref();
        for (index, val) in temp_check_sum.iter().enumerate() {
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
        let s = Secp256k1::signing_only();
        let (sk1, pk1) = s.generate_keypair(&mut thread_rng());
        let mut temp_pri_key = [0u8; 32];
        temp_pri_key.copy_from_slice(&Vec::from_hex(sk1.to_string()).unwrap().as_slice()[..]);
        self.pri_key = Some(temp_pri_key);
        let mut temp_pub_key = [0u8; 65];
        temp_pub_key.copy_from_slice(&pk1.serialize_uncompressed()[..]);
        self.pub_key = Some(temp_pub_key);
    }

    pub fn save_keys_to_local_file(keys: &String, path: &String, seid: &String) {
        let file = File::open(Path::new(format!("{}key{}{}", path, seid, ".txt").as_str()));
        let mut file = match file {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    match File::create(Path::new(format!("{}key{}{}", path, seid, ".txt").as_str()))
                    {
                        Ok(fc) => fc,
                        Err(e) => panic!("create file error"),
                    }
                }
                _ => panic!("open file error"),
            },
        };
        file.write_all(keys.as_bytes());
    }
}
