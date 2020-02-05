extern crate aes_soft as aes;
use super::key_manager::KeyManager;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use common::apdu::Apdu;
use hex::FromHex;
use mq::message::send_apdu;
use rand::rngs::OsRng;
use ring::digest;
use rsa::{BigUint, PaddingScheme, PublicKey as RSAPublic, RSAPrivateKey, RSAPublicKey};
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};
use sha1::Sha1;
use crate::manager;
use crate::key_manager::SE_PUB_KEY;

pub struct DeviceManage {
    key_manager: KeyManager,
}

impl DeviceManage {
    pub fn new() -> DeviceManage {
        DeviceManage {
            key_manager: KeyManager::new(),
        }
    }

pub fn bind_check(&mut self, file_path: &String) -> String {
    //获取seid
//    let seid = String::from("19060000000200860001010000000014");
    let seid = manager::get_se_id();
    //获取SN号
//    let sn = String::from("imKey01191200001");
    let sn = manager::get_sn();
    let sn = String::from_utf8(hex::decode(sn).unwrap()).unwrap();
    //计算文件加密密钥
    //        let mut temp_key_manager = KeyManager::new();
    let mut key_manager_obj = &mut self.key_manager;
    key_manager_obj.gen_encrypt_key(&seid, &sn);

    //获取本地密钥文件内容
    let ciphertext = KeyManager::get_key_file_data(file_path, &seid);
    let mut key_flag = false;
    if !ciphertext.is_empty() {
        key_flag = !key_manager_obj.decrypt_keys(ciphertext.as_bytes());
    }

    //如果密钥文件不存在或者密钥文件里没有数据则重新生成
    if ciphertext.is_empty() || key_flag {
        //生成公私钥
        key_manager_obj.gen_local_keys();
        key_flag = true;
    }

    //生成bindcheck指令
    let bind_check_apdu =
        Apdu::bind_check(&key_manager_obj.pub_key);

    //发送bindcheck指令，并获取返回数据 TODO
    let apdu_response = send_apdu(Apdu::select_applet("695F696D6B"));
    if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
        panic!("selcet imk error");
    }
    let bind_check_apdu_resp_data = send_apdu(bind_check_apdu);
    if !"9000".eq(&bind_check_apdu_resp_data[bind_check_apdu_resp_data.len() - 4 ..]) {
        panic!("bind check apdu error");
    }

    //获取状态值 //状态 0x00: 未绑定  0x55: 与传入appPK绑定  0xAA：与其他appPK绑定
    let status: String = bind_check_apdu_resp_data.chars().take(2).collect();
    let se_pub_key_cert: String = bind_check_apdu_resp_data.chars().skip(2).collect();
    if status.eq("00") || status.eq("AA") {
        //验证SE证书 TODO

        //解析SE公钥证书，获取SE公钥
        //82cc68ac4bd131d84d4dcfeab1bb606cae40b9be267892326f3ccfa1a0f862a1
        //040258df4552fbe4f1eb2b4d2ed30978511e2e3bc1f9eed02502b5ef766a98900124f8ea6965322edb80f45de48058c2b5c2cc5df57e755f9437d5d47d0d14c7d1
//        let temp_se_pub_key = Vec::from_hex("04FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44").unwrap();
        let se_cert_str = manager::get_cert();
        println!("{:?}", se_cert_str);
        let mut index = 0;
        if se_cert_str.contains("7F4947B041") {
            index = se_cert_str.find("7F4947B041").expect("get tager index error");
        }else if se_cert_str.contains("7F4946B041") {
            index = se_cert_str.find("7F4946B041").expect("get tager index error");
        }else {
            panic!("se cert error");
        }

        let temp_se_pub_key = &se_cert_str[index + 10..index + 130 + 10];
        let mut se_pub_key_ = SE_PUB_KEY.lock().unwrap();
        *se_pub_key_ = temp_se_pub_key.to_string();
        std::mem::drop(se_pub_key_);

        key_manager_obj.se_pub_key = hex::decode(temp_se_pub_key).unwrap();

        //协商会话密钥
//        let se_pub_key_obj = PublicKey::from_slice(temp_se_pub_key.as_ref()).unwrap();
//        println!(
//            "pri_key : {:?}",
//            hex::encode_upper(key_manager_obj.pri_key.unwrap().as_ref())
//        );
//        let locl_pri_key_obj =
//            SecretKey::from_slice(key_manager_obj.pri_key.unwrap().as_ref()).unwrap();
//        let sec = SharedSecret::new(&se_pub_key_obj, &locl_pri_key_obj);
//        //SHA1
//        let sha1_data = Sha1::from(&sec[..]).digest().bytes();

        let pk2 = PublicKey::from_slice(key_manager_obj.se_pub_key.as_slice()).expect("generator public key error");
        let sk1 = SecretKey::from_slice(key_manager_obj.pri_key.as_slice()).expect("generator private key error");
        let expect_result: [u8; 64] = [123; 64];
        let mut x_out = [0u8; 32];
        let mut y_out = [0u8; 32];
//        SharedSecret
        let result = SharedSecret::new_with_hash(&pk2, &sk1, | x, y | {
            x_out = x;
            y_out = y;
            expect_result.into()
        }).unwrap();
        let sha1_result = Sha1::from(&x_out[..]).digest().bytes();

        //设置session key
        key_manager_obj.session_key = sha1_result[..16].to_vec();

        //保存密钥到本地文件
        if key_flag {
            let ciphertext = key_manager_obj.encrypt_data();
            KeyManager::save_keys_to_local_file(&ciphertext, file_path, &seid);
        }
    }
    if status.eq("00") {
        return "unbound".to_string();
    }else if status.eq("55") {
        return "bound_this".to_string();
    }else if status.eq("AA") {
        return "bound_other".to_string()
    }else {
        panic!("bind check status error");
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
        let apdu_response = send_apdu(Apdu::select_applet("695F696D6B"));
        if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
            panic!("selcet imk error");
        }
        //计算HASH
        let mut data: Vec<u8> = Vec::new();
        data.extend(binding_code_bytes);
        data.extend(&self.key_manager.pub_key);

        data.extend(&self.key_manager.se_pub_key);
        let data_hash = digest::digest(&digest::SHA256, data.as_slice());

        //用sessionKey加密HASH值
        type Aes128Cbc = Cbc<Aes128, Pkcs7>;
        let cipher = Aes128Cbc::new_var(
            &self.key_manager.session_key,
            &gen_iv(&temp_binding_code).as_ref(),
        ).unwrap();

        let ciphertext = cipher.encrypt_vec(data_hash.as_ref());
        //生成identityVerify指令数据
        let mut apdu_data = Vec::new();
        apdu_data.extend(&self.key_manager.pub_key);
        apdu_data.extend(ciphertext);
        let identity_verify_apdu = Apdu::identity_verify(&apdu_data);
        //发送指令到设备
        //        let response = hid_api::send(&hid_device, &identity_verify_apdu);
        let response = send_apdu(identity_verify_apdu);
        if !"9000".eq(&response[response.len() - 4 ..]) {
            panic!("identity verify apdu error");
        }
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
    let mut result = [0u8; 32];
    for (index, value) in auth_code_hash.as_ref().iter().enumerate() {
        result[index] = value ^ salt_bytes.as_ref().get(index).unwrap();
    }
    let mut return_data = [0u8; 16];
    return_data.copy_from_slice(&result[..16]);
    return_data
}

/**
encrypt auth code
*/
fn auth_code_encrypt(auth_code: &String) -> String {
    let n = hex::decode("C6627A6F0485B33DDC1CA7E062C64E8841133B9246A41F40D0767BAE44EAB2EF453D008FFB07B8D9FDFCD21882487ECC4DA933C97E494242ADA3CE02C5A05189AA49410E771A66E8100E43CB1AF6CC610B59EE4EBB236FF38C62AD7B1D11DFBD4E054D19E3349391A31F5E89CA721292B7380295745D8968CC5C2D223AC6750BB0ACA27773687E9CD76065E47F42F4AE005459BCE5746BD760646A5BD119BA3469A935F48EB898CBAB72CB394C3FEC9E41635EAE954107A17AC7B8C6321D8F1755AD3915A9D2398DB268A3F642CEE9CBE9F82ECD5AD64EBEDDDE66601DC2B891E2FEDDF72DAF627FA8FA16F7C640DB661BE15DCB4274D9576D98DBEB20C25309");
    let e = hex::decode("010001");
    let u32_vec_n = BigUint::from_bytes_be(&n.unwrap());
    let u32_vec_e = BigUint::from_bytes_be(&e.unwrap());
    let rsa_pub_key = RSAPublicKey::new(u32_vec_n, u32_vec_e);
    let mut rng = OsRng::new().expect("no secure randomness available");
    let enc_data =
        rsa_pub_key
            .unwrap()
            .encrypt(&mut rng, PaddingScheme::PKCS1v15, auth_code.as_bytes());
    hex::encode_upper(enc_data.unwrap())
}

pub fn display_bind_code() -> String {
    let apdu_response = send_apdu(Apdu::select_applet("695F696D6B"));
    if !"9000".eq(&apdu_response[apdu_response.len() - 4 ..]) {
        panic!("selcet imk error");
    }
    let gen_auth_code_ret_data = send_apdu(Apdu::generate_auth_code());
    if !"9000".eq(&gen_auth_code_ret_data[gen_auth_code_ret_data.len() - 4 ..]) {
        panic!("gen auth code apdu error");
    }
    gen_auth_code_ret_data[..gen_auth_code_ret_data.len() - 4].to_string()
}

#[cfg(test)]
mod test{
    use crate::key_manager::KeyManager;
    use crate::device_binding::DeviceManage;

    #[test]
    fn device_bind_test(){

        let path = "/Users/caixiaoguang/workspace/myproject/imkey-core/".to_string();
        let bind_code = "E4APZZRT".to_string();
        let mut device_manage = DeviceManage::new();
        device_manage.bind_check(&path);
        device_manage.bind_acquire(&bind_code);

//        let sn = String::from("imKey01191200001");
//        println!("{:?}", hex::encode_upper(sn.as_bytes()));
//        println!("{:?}", String::from_utf8(sn.as_bytes().to_vec()));

    }

    #[test]
    fn cert_parsing(){
        //证书解析
        let cert = "BF2181CA7F2181C6931019060000000200860001010000000014420200015F200401020304950200805F2504201810145F2404FFFFFFFF53007F4947B04104FAF45816AB9B5364B5C4C376E9E63F716CEB3CD63E7A195D780D2ECA1DD50F04C9230A8A72FDEE02A9306B1951C00EB452131243091961B191470AB3EED33F44F002DFFE5F374830460221008CB58D54BDED501236621B83B320081E6F9B6B5539AE5EC9D36B660EC445A5E8022100A203CA1F9ABEE69751EA402A2ACDFD6B4A87697D6CD721F60540959095EC";
        if cert.contains("7F4947B041") || cert.contains("7F4946B041"){
            println!("success");
            let index = cert.find("7F4947B041").expect("get tager index error");
            let se_pub_key = &cert[index + 10..index + 140];
            println!("{:?}", se_pub_key);
        }else {
            println!("{:?}", "cert error");
        }
    }

}
