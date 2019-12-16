extern crate aes_soft as aes;
use super::key_manager::KeyManager;
use common::apdu::Apdu;
use hex::FromHex;
use rand::rngs::OsRng;
use ring::digest;
use rsa::{PaddingScheme, PublicKey as RSAPublic, RSAPrivateKey, BigUint, RSAPublicKey};
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};
use sha1::Sha1;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use crate::hid_api;

pub struct DeviceManage {
    key_manager: KeyManager,
}

impl DeviceManage {
    pub fn new() -> DeviceManage {
        DeviceManage {
            key_manager: KeyManager::new(),
        }
    }
    pub fn bind_check(&mut self, file_path : &String) {
        //获取seid
        let seid = String::from("18080000000000860001010000000106");
        //获取SN号
        let sn = String::from("imKey01191200010");

        //计算文件加密密钥
        //        let mut temp_key_manager = KeyManager::new();
        let mut temp_key_manager = &mut self.key_manager;
        temp_key_manager.gen_encrypt_key(&seid, &sn);

        //获取本地密钥文件内容
        let ciphertext = KeyManager::get_key_file_data(file_path, &seid);
        let mut key_flag = false;
        if !ciphertext.is_empty() {
            key_flag = !temp_key_manager.decrypt_keys(ciphertext.as_bytes());
        }

        //如果密钥文件不存在或者密钥文件里没有数据则重新生成
        if ciphertext.is_empty() || key_flag {
            //生成公私钥
            temp_key_manager.gen_local_keys();
            key_flag = true;
        }

        //生成bindcheck指令
        let bind_check_apdu = Apdu::bind_check(&temp_key_manager.pub_key.unwrap().as_ref().to_vec());

        //发送bindcheck指令，并获取返回数据 TODO
        let hid_device = hid_api::connect();
        let select_imk_applet = Apdu::select_applet("695F696D6B");
        let response = hid_api::send(&hid_device, &select_imk_applet);
        let bind_check_apdu_resp_data = hid_api::send(&hid_device, &bind_check_apdu);

        //获取状态值 //状态 0x00: 未绑定  0x55: 与传入appPK绑定  0xAA：与其他appPK绑定
        let status : String = bind_check_apdu_resp_data.chars().take(2).collect();
        let se_pub_key_cert : String = bind_check_apdu_resp_data.chars().skip(2).collect();
        if status.eq("00") || status.eq("AA") {
            //验证SE证书 TODO

            //解析SE公钥证书，获取SE公钥
            //82cc68ac4bd131d84d4dcfeab1bb606cae40b9be267892326f3ccfa1a0f862a1
            //040258df4552fbe4f1eb2b4d2ed30978511e2e3bc1f9eed02502b5ef766a98900124f8ea6965322edb80f45de48058c2b5c2cc5df57e755f9437d5d47d0d14c7d1
            let temp_se_pub_key = Vec::from_hex("0403089D8A83A87F24D906303A49D39669D17B0F7AB76EB098A65AFEF31154E75DEE5B87B69CBF78F11E831A4961C8A8F031C2869EA0716C798F76F5E91338DC35").unwrap();
            let mut se_pub_key = [0u8; 65];
            se_pub_key.copy_from_slice(temp_se_pub_key.as_slice());
            temp_key_manager.se_pub_key = Some(se_pub_key);

            //协商会话密钥
            let se_pub_key_obj = PublicKey::from_slice(temp_se_pub_key.as_ref()).unwrap();
            println!("pri_key : {:?}", hex::encode_upper(temp_key_manager.pri_key.unwrap().as_ref()));
            let locl_pri_key_obj =
                SecretKey::from_slice(temp_key_manager.pri_key.unwrap().as_ref()).unwrap();
            let sec = SharedSecret::new(&se_pub_key_obj, &locl_pri_key_obj);
            //SHA1
            let sha1_data = Sha1::from(&sec[..]).digest().bytes();
            //设置session key
            let mut temp_session_key = [0u8; 16];
            temp_session_key.copy_from_slice(&sha1_data[..16]);
            println!("sessionkey : {:?}", hex::encode_upper(temp_session_key.to_vec()));
            temp_key_manager.session_key = Some(temp_session_key);

            //保存密钥到本地文件
            if key_flag {
                let ciphertext = temp_key_manager.encrypt_data();
                KeyManager::save_keys_to_local_file(&ciphertext, file_path, &seid);
            }
        }
    }

    pub fn bind_acquire(&self, binding_code: &String) -> String {

        let temp_binding_code = binding_code.to_uppercase();
        let binding_code_bytes = temp_binding_code.as_bytes();
        //绑定码校验 TODO
        let reg_ex = "^[A-HJ-NP-Z2-9]{8}$";

        //RSA加密绑定码
        let auth_code_ciphertext = auth_code_encrypt(&temp_binding_code);

        //保存绑定码 TODO
//        let seid = String::from("18090000000000860001010000000204");
//        let auth_code_storage_result = auth_code_storage_request::build_request_data(seid, auth_code_ciphertext).auth_code_storage();
//        if auth_code_storage_result.is_err() {
//            //TODO
//        }

        //选择IMK applet TODO
//        select_imk_applet();
        let hid_device = hid_api::connect();
        let select_imk_applet = Apdu::select_applet("695F696D6B");
        let response = hid_api::send(&hid_device, &select_imk_applet);


        //计算HASH
        let mut data: Vec<u8> = Vec::new();
        data.extend(binding_code_bytes);
        println!("{}", hex::encode_upper(binding_code_bytes));
        data.extend(self.key_manager.pub_key.unwrap().as_ref().iter());
        println!("pub_key:{:?}", hex::encode_upper(self.key_manager.pub_key.unwrap().as_ref()));

        data.extend(self.key_manager.se_pub_key.unwrap().iter());
        println!("se_pub_key:{:?}", hex::encode_upper(self.key_manager.se_pub_key.unwrap().as_ref()));
        println!("data : {:?}", hex::encode_upper(data.as_slice()));
        let data_hash = digest::digest(&digest::SHA256, data.as_slice());
        println!("hash value:{:?}", data_hash.as_ref());
        println!("data : {:?}", hex::encode_upper(data_hash.as_ref()));

        //用sessionKey加密HASH值
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        println!("session_key:{:?}", hex::encode_upper(self.key_manager.session_key.unwrap().as_ref()));
        println!("iv : {:?}", hex::encode_upper(gen_iv(&temp_binding_code)));

        let cipher = Aes128Cbc::new_var(
            self.key_manager.session_key.unwrap().as_ref(),
            &gen_iv(&temp_binding_code).as_ref(),
        ).unwrap();
        let ciphertext = cipher.encrypt_vec(data_hash.as_ref());
        println!("ciphertext:{:?}", hex::encode_upper(ciphertext.as_slice()));
        //生成identityVerify指令数据
        let mut apdu_data = Vec::new();
        apdu_data.extend(self.key_manager.pub_key.unwrap().as_ref());
        apdu_data.extend(ciphertext);
        println!("{:?}", apdu_data.as_slice());
        let identity_verify_apdu = Apdu::identity_verify(&apdu_data);
        println!("{:?}", identity_verify_apdu);
        //发送指令到设备
        let response = hid_api::send(&hid_device, &identity_verify_apdu);
        response.chars().take(2).collect()
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

/**
encrypt auth code
*/
fn auth_code_encrypt(auth_code : &String) -> String{

    let n = hex::decode("C6627A6F0485B33DDC1CA7E062C64E8841133B9246A41F40D0767BAE44EAB2EF453D008FFB07B8D9FDFCD21882487ECC4DA933C97E494242ADA3CE02C5A05189AA49410E771A66E8100E43CB1AF6CC610B59EE4EBB236FF38C62AD7B1D11DFBD4E054D19E3349391A31F5E89CA721292B7380295745D8968CC5C2D223AC6750BB0ACA27773687E9CD76065E47F42F4AE005459BCE5746BD760646A5BD119BA3469A935F48EB898CBAB72CB394C3FEC9E41635EAE954107A17AC7B8C6321D8F1755AD3915A9D2398DB268A3F642CEE9CBE9F82ECD5AD64EBEDDDE66601DC2B891E2FEDDF72DAF627FA8FA16F7C640DB661BE15DCB4274D9576D98DBEB20C25309");
    let e = hex::decode("010001");
    let u32_vec_n = BigUint::from_bytes_be(&n.unwrap());
    let u32_vec_e = BigUint::from_bytes_be(&e.unwrap());
    let rsa_pub_key = RSAPublicKey::new(u32_vec_n, u32_vec_e);
    let mut rng = OsRng::new().expect("no secure randomness available");
    let enc_data = rsa_pub_key.unwrap().encrypt(&mut rng, PaddingScheme::PKCS1v15, auth_code.as_bytes());
    hex::encode_upper(enc_data.unwrap())

}

pub fn display_bind_code() -> String{

    let hid_device = hid_api::connect();
    let select_imk_applet = Apdu::select_applet("695F696D6B");
    let response = hid_api::send(&hid_device, &select_imk_applet);
    let gen_auth_code_apdu = Apdu::generate_auth_code();
    let bind_code = hid_api::send(&hid_device, &gen_auth_code_apdu);
    bind_code
}