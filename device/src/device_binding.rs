use crate::key_manager::KeyManager;
use common::apdu;
use sha1::Sha1;
pub struct DeviceManage {}

impl DeviceManage {
    pub fn bind_check() {
        //获取seid
        let seid = String::from("18090000000000860001010000000204");
        //获取SN号
        let sn = String::from("imKey01190300020");

        //计算文件加密密钥
        let mut temp_key_manager = KeyManager::new();
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
        select_imk_applet();
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
}
use common::apdu::Apdu;
use hex::FromHex;
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};

fn select_imk_applet() {
    let select_imkey = apdu::select(&Vec::from_hex("695F696D6B").unwrap());
    //发送指令到设备
}
