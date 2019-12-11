use crate::key_manager::KeyManager;
use common::apdu;
use common::apdu::Apdu;
use hex::FromHex;
use rand::rngs::OsRng;
use ring::digest;
use rsa::{PaddingScheme, PublicKey as RSAPublic, RSAPrivateKey};
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};
use sha1::Sha1;
extern crate aes_soft as aes;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

//use crate::auth_code_storage::auth_code_storage_request;

pub struct DeviceManage {
    key_manager: KeyManager,
}

impl DeviceManage {
    pub fn new() -> DeviceManage {
        DeviceManage {
            key_manager: KeyManager::new(),
        }
    }
    pub fn bind_check(&mut self) {
        //获取seid
        let seid = String::from("18090000000000860001010000000204");
        //获取SN号
        let sn = String::from("imKey01190300020");

        //计算文件加密密钥
        //        let mut temp_key_manager = KeyManager::new();
        let mut temp_key_manager = &mut self.key_manager;
        temp_key_manager.gen_encrypt_key(&seid, &sn);

        //获取本地密钥文件内容
        let file_path = String::from("/Users/caixiaoguang/workspace/GIT/imkey-core/");
        let ciphertext = KeyManager::get_key_file_data(&file_path.to_string(), &seid);
        let mut key_flag = false;
        if !ciphertext.is_empty() {
            key_flag = !temp_key_manager.decrypt_keys(ciphertext.as_bytes());
        }

        if ciphertext.is_empty() || key_flag {
            //生成公私钥
            temp_key_manager.gen_local_keys();
            key_flag = true;
        }

        //发送指令
        //        select_imk_applet();
        //生成bindcheck指令
        let bind_check_apdu = apdu::bind_check(&temp_key_manager.pub_key.unwrap().to_vec());
        //发送bindcheck指令，并获取返回数据
        let bind_check_apdu_resp_data = String::from("xxxxxxxxxxxx"); // TODO
                                                                      //获取状态值 //状态 0x00: 未绑定  0x55: 与传入appPK绑定  0xAA：与其他appPK绑定
        let status = "00";
        let se_pub_key_cert = "";
        if "00".eq(status) || "AA".eq(status) {
            //发送服务器安全验证请求验证SE证书 TODO

            //解析SE公钥证书，获取SE公钥
            //82cc68ac4bd131d84d4dcfeab1bb606cae40b9be267892326f3ccfa1a0f862a1
            //040258df4552fbe4f1eb2b4d2ed30978511e2e3bc1f9eed02502b5ef766a98900124f8ea6965322edb80f45de48058c2b5c2cc5df57e755f9437d5d47d0d14c7d1
            let temp_se_pub_key = Vec::from_hex("040258df4552fbe4f1eb2b4d2ed30978511e2e3bc1f9eed02502b5ef766a98900124f8ea6965322edb80f45de48058c2b5c2cc5df57e755f9437d5d47d0d14c7d1").unwrap();
            //协商会话密钥
            let se_pub_key_obj = PublicKey::from_slice(temp_se_pub_key.as_ref()).unwrap();
            let locl_pri_key_obj =
                SecretKey::from_slice(temp_key_manager.pri_key.unwrap().as_ref()).unwrap();
            let sec = SharedSecret::new(&se_pub_key_obj, &locl_pri_key_obj);
            //SHA1
            //            let sha1_data = Sha1::from(sec).digest().bytes();
            //
            //            let mut temp_session_key = [0u8; 16];
            //            temp_session_key.copy_from_slice(&sha1_data[..16]);
            //            temp_key_manager.session_key = Some(temp_data);

            //保存密钥到本地文件
            //            if key_flag {
            if true {
                let ciphertext = temp_key_manager.encrypt_data();
                KeyManager::save_keys_to_local_file(&ciphertext, &file_path, &seid);
            }
        }
    }

    pub fn bind_acquire(&self, binding_code: &String) {
        let temp_binding_code = binding_code.to_uppercase();
        //绑定码校验 TODO
        let reg_ex = "^[A-HJ-NP-Z2-9]{8}$";

        //RSA加密绑定码
        let mut rng = OsRng::new().expect("no secure randomness available");
        let bits = 2048;
        let key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

        let binding_code_bytes = temp_binding_code.as_bytes();
        println!("绑定码bytes{:?}", binding_code_bytes);
        let enc_data = key
            .encrypt(&mut rng, PaddingScheme::PKCS1v15, &binding_code_bytes)
            .expect("failed to encrypt");
        let temp_enc_data = hex::encode(enc_data);
        println!("绑定码密文值：{:?}", temp_enc_data);

        //保存验证码 TODO
        //        let seid = String::from("18090000000000860001010000000204");
        //        let auth_code_storage_result = auth_code_storage_request::build_request_data(seid, temp_enc_data).auth_code_storage();
        //        if auth_code_storage_result.is_err() {
        //            //TODO
        //        }

        //选择IMK applet
        select_imk_applet();

        //计算HASH
        let mut data: Vec<u8> = Vec::new();
        data.extend(binding_code_bytes);
        data.extend(self.key_manager.pub_key.unwrap().iter());
        println!("pub_key:{:?}", self.key_manager.pub_key.unwrap().iter());
        data.extend(self.key_manager.se_pub_key.unwrap().iter());
        println!(
            "se_pub_key:{:?}",
            self.key_manager.se_pub_key.unwrap().iter()
        );
        let data_hash = digest::digest(&digest::SHA256, data.as_slice());
        println!("hash value:{:?}", data_hash.as_ref());
        //用sessionKey加密HASH值
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        println!(
            "session_key:{:?}",
            self.key_manager.session_key.unwrap().iter()
        );
        let cipher = Aes128Cbc::new_var(
            self.key_manager.session_key.unwrap().as_ref(),
            &gen_iv(&temp_binding_code).as_ref(),
        )
        .unwrap();
        let ciphertext = cipher.encrypt_vec(data.as_ref());
        println!("ciphertext:{:?}", ciphertext.as_slice());
        //生成identityVerify指令数据
        let mut apdu_data = Vec::new();
        apdu_data.extend(self.key_manager.pub_key.unwrap().as_ref());
        apdu_data.extend(ciphertext);
        println!("{:?}", apdu_data.as_slice());
        let identity_verify_apdu = Apdu::identity_verify(&apdu_data);
        //发送指令到设备
    }
}

fn select_imk_applet() {
    let select_imkey = Apdu::select_applet("695F696D6B");
    //把指令把指令发送到设备
}

fn gen_iv(auth_code: &String) -> [u8; 16] {
    let salt_bytes = digest::digest(&digest::SHA256, "bindingCode".as_bytes());
    let auth_code_hash = digest::digest(&digest::SHA256, auth_code.as_bytes());
    println!("{:?}", salt_bytes.as_ref());
    println!("{:?}", auth_code_hash.as_ref());
    let mut result = [0u8; 32];
    for (index, value) in auth_code_hash.as_ref().iter().enumerate() {
        result[index] = value ^ salt_bytes.as_ref().get(index).unwrap();
    }
    println!("{:?}", result);
    let mut return_data = [0u8; 16];
    return_data.copy_from_slice(&result[..16]);
    println!("{:?}", return_data);
    return_data
}
