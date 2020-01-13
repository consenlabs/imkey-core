use crate::constants::{BTC_AID, ETH_AID, LC_MAX, COSMOS_AID};
use hex;
use regex::internal::Input;
use rustc_serialize::hex::ToHex;

pub struct Apdu {}

struct ApduHeader {
    cla: u8,
    ins: u8,
    p1: u8,
    p2: u8,
    lc: u8,
}

impl ApduHeader {
    fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8) -> ApduHeader {
        ApduHeader {
            cla: cla,
            ins: ins,
            p1: p1,
            p2: p2,
            lc: lc,
        }
    }

    fn to_array(&self) -> Vec<u8> {
        vec![self.cla, self.ins, self.p1, self.p2, self.lc]
    }
}

impl Apdu {
    //pub fn eth_select() -> String {
    pub fn select_applet(aid: &str) -> String {
        let aid_array = hex::decode(aid).unwrap(); //@@XM TOOD: take care of this unwrap
        let mut apdu = Vec::new();
        let apdu_header = ApduHeader::new(0x00, 0xA4, 0x04, 0x00, aid_array.len() as u8);
        apdu.extend(apdu_header.to_array().iter());
        apdu.extend(aid_array.iter());
        apdu.push(0x00); //Le
        hex::encode(apdu)
    }

    pub fn prepare_sign(ins: u8, data: Vec<u8>) -> Vec<String> {
        let mut apdu_list = Vec::new();
        let size = data.len() as u32 / LC_MAX as u32
            + if data.len() as u32 % LC_MAX as u32 != 0 {
                1
            } else {
                0
            };

        for i in 0..size {
            let mut apdu = Vec::new();
            let p1 = if i == 0 { 0x00 } else { 0x80 };
            let (p2, lc) = if i == size - 1 {
                (
                    0x80,
                    (data.len() as u32 - LC_MAX as u32 * (size - 1 as u32)) as u8,
                )
            } else {
                (0x00, 0xF5)
            };
            let apdu_header = ApduHeader::new(0x80, ins, p1, p2, lc);
            apdu.extend(apdu_header.to_array().iter());
            let payload =
                &data[(i as u32 * LC_MAX) as usize..(i as u32 * LC_MAX + lc as u32) as usize];
            apdu.extend(payload.iter()); //payload
            apdu.push(0x00); //le
            apdu_list.push(hex::encode(apdu))
        }
        apdu_list
    }

    pub fn get_pubkey(ins: u8, path: &str, verify_flag: bool) -> String {
        let path_bytes = path.as_bytes();
        if path_bytes.len() > 256 {
            panic!("data to long");
        }

        let mut apdu = Vec::new();

        let p1 = if verify_flag { 0x01 } else { 0x00 };

        let apdu_header = ApduHeader::new(0x80, ins, p1, 0x00, path_bytes.len() as u8);
        apdu.extend(apdu_header.to_array().iter());
        apdu.extend(path_bytes.iter());
        apdu.push(0x00); //Le
        hex::encode(apdu)
    }

    pub fn register_address(ins: u8, data: &[u8]) -> String {
        if data.len() > 256 {
            panic!("data to long");
        }
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(ins);
        apdu.push(0x00);
        apdu.push(0x00);
        apdu.push(data.len() as u8);
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn sign_digest(ins: u8, index: u8, hashtype: u8, path: &str) -> String {
        let path_bytes = path.as_bytes();
        if path_bytes.len() > 256 {
            panic!("data to long");
        }

        let mut apdu = Vec::new();

        let apdu_header = ApduHeader::new(0x80, ins, index, hashtype, path_bytes.len() as u8);
        apdu.extend(apdu_header.to_array().iter());
        apdu.extend(path_bytes.iter()); //payload
        apdu.push(0x00); //le
        hex::encode(apdu)
    }

    /**
    binding check apdu build
    */
    pub fn bind_check(data: &Vec<u8>) -> String {
        if data.len() > 256 {
            panic!("data to long");
        }
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(0x71);
        apdu.push(0x00);
        apdu.push(0x00);
        apdu.push(data.len() as u8);
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }
    /**
    binding check apdu build
    */
    pub fn generate_auth_code() -> String {
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(0x72);
        apdu.push(0x00);
        apdu.push(0x00);
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }
    pub fn identity_verify(data: &Vec<u8>) -> String {
        if data.len() > 256 {
            panic!("data to long");
        }
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(0x73);
        apdu.push(0x80);
        apdu.push(0x00);
        apdu.push(data.len() as u8);
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }
}

pub struct EthApdu {}

impl EthApdu {
    pub fn select_applet() -> String {
        Apdu::select_applet(ETH_AID)
    }

    pub fn prepare_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x51, data)
    }

    pub fn get_pubkey(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x53, path, verify_flag)
    }

    pub fn sign_digest(path: &str) -> String {
        Apdu::sign_digest(0x52, 0x00, 0x00, path)
    }
}

pub struct EosApdu {}

impl EosApdu {
    pub fn select_applet() -> String {
        Apdu::select_applet(ETH_AID)
    }

    pub fn prepare_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x51, data)
    }

    pub fn get_pubkey(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x63, path, verify_flag)
    }

    pub fn register_pubkey(data: &[u8]) -> String {
        Apdu::register_address(0x66, data)
    }

    pub fn sign_digest(path: &str) -> String {
        Apdu::sign_digest(0x52, 0x00, 0x00, path)
    }
}

pub struct CosmosApdu {}

impl CosmosApdu {
    pub fn select_applet() -> String {
        Apdu::select_applet(COSMOS_AID)
    }

    pub fn prepare_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x71, data)
    }

    pub fn get_pubkey(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x73, path, verify_flag)
    }

    pub fn register_pubkey(data: &[u8]) -> String {
        Apdu::register_address(0x76, data)
    }

    pub fn sign_digest(path: &str) -> String {
        Apdu::sign_digest(0x72, 0x00, 0x00, path)
    }
}

pub struct BtcApdu {}

impl BtcApdu {
    pub fn select_applet() -> String {
        Apdu::select_applet(BTC_AID)
    }

    /**
    获取xpub
    */
    pub fn get_xpub(path: &str, verify_flag: bool) -> String {
        if path.as_bytes().len() > 256 {
            panic!("data to long");
        }
        let mut apdu = Vec::new();
        apdu.push(0x80); //CLA
        apdu.push(0x43); //INS
        apdu.push(0x00); //P1
                         //p2
        if verify_flag {
            apdu.push(0x01);
        } else {
            apdu.push(0x00);
        }
        let path_bytes = path.as_bytes();
        apdu.push(path_bytes.len() as u8); //Lc
        apdu.extend(path_bytes.iter()); //data
        apdu.push(0x00); //Le
        apdu.to_hex().to_uppercase()
    }

    pub fn btc_prepare(ins: u8, p1: u8, data: &Vec<u8>) -> Vec<String> {
        let mut apdu_vec = Vec::new();
        let apdu_number = (data.len() - 1) / LC_MAX as usize + 1;
        for index in (0..apdu_number) {
            if index == apdu_number - 1 {
                let length = if data.len() % LC_MAX as usize == 0 {
                    LC_MAX
                } else {
                    (data.len() % LC_MAX as usize) as u32
                };
                let mut temp_apdu_vec = Vec::new();
                temp_apdu_vec.push(0x80);
                temp_apdu_vec.push(ins);
                temp_apdu_vec.push(p1);
                temp_apdu_vec.push(0x80);
                temp_apdu_vec.push(length as u8);
                println!("left:{}", index * LC_MAX as usize);
                //                println!("right:{}", length as usize);
                temp_apdu_vec.extend_from_slice(&data[index * LC_MAX as usize..]);
                apdu_vec.push(hex::encode_upper(temp_apdu_vec));
            } else {
                let mut temp_apdu_vec = Vec::new();
                temp_apdu_vec.push(0x80);
                temp_apdu_vec.push(ins);
                temp_apdu_vec.push(p1);
                temp_apdu_vec.push(0x80);
                temp_apdu_vec.push(LC_MAX as u8);
                temp_apdu_vec.extend_from_slice(
                    &data[index * LC_MAX as usize..((index + 1) * LC_MAX as usize) as usize],
                );
                apdu_vec.push(hex::encode_upper(temp_apdu_vec));
            }
        }
        return apdu_vec;
    }

    pub fn btc_perpare_input(p1: u8, data: &Vec<u8>) -> String {
        if data.len() > 256 {
            panic!("data to long");
        }
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(0x41);
        apdu.push(p1);
        apdu.push(0x00);
        apdu.push(data.len() as u8);
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn btc_sign(index: u8, hash_type: u8, path: &str) -> String {
        let path_bytes = path.as_bytes();
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(0x42);
        apdu.push(index);
        apdu.push(hash_type);
        apdu.push(path_bytes.len() as u8);
        apdu.extend(path_bytes.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn btc_segwit_sign(last_one : bool, hash_type : u8, data : Vec<u8>) -> String{
        let mut apdu = Vec::new();
        apdu.push(0x80);
        apdu.push(0x32);
        if last_one {
            apdu.push(0x80);
        }else{
            apdu.push(0x00);
        }
        apdu.push(hash_type);
        apdu.push(data.len() as u8);
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

}

/**
binding check apdu build
*/
pub fn bind_check(data: &Vec<u8>) -> String {
    if data.len() > 256 {
        panic!("data to long");
    }
    let mut apdu = Vec::new();
    apdu.push(0x80);
    apdu.push(0x71);
    apdu.push(0x00);
    apdu.push(0x00);
    apdu.push(data.len() as u8);
    apdu.extend(data.iter());
    apdu.push(0x00);
    apdu.to_hex().to_uppercase()
}

#[cfg(test)]
mod tests {
    use crate::apdu::BtcApdu;
    use hex::FromHex;

    #[test]
    fn get_xpub_test() {
        let path = String::from("m/44/0/0");
        let verify_flag = false;
        assert_eq!(
            BtcApdu::get_xpub(&path, verify_flag),
            String::from("80430001086D2F34342F302F3000")
        );
    }
    #[test]
    fn btc_prepare() {
        let data = Vec::from_hex("004630440220041038231F1C5E8D98EF941347BD9B6C220578128677BD561D258EC9B4CDFA3502203D18C43F7D06EE32D9527C8322F4D675F58856EC227ABF7085CCE65D5E5100C1019501000000040220D9AE2F000000001976A91455BDC1B42E3BED851959846DDF600E96125423E088AC0000000000000000536A4C500200000080A10BC28928F4C17A287318125115C3F098ED20A8237D1E8E4125BC25D1BE99752ADAD0A7B9CECA853768AEBB6965ECA126A62965F698A0C1BC43D83DB632AD7F717276057E6012AFA99385000000000100000000000000000027106F004630440220041038231F1C5E8D98EF941347BD9B6C220578128677BD561D258EC9B4CDFA3502203D18C43F7D06EE32D9527C8322F4D675F58856EC227ABF7085CCE65D5E5100C1019501000000040220D9AE2F000000001976A91455BDC1B42E3BED851959846DDF600E96125423E088AC0000000000000000536A4C500200000080A10BC28928F4C17A287318125115C3F098ED20A8237D1E8E4125BC25D1BE99752ADAD0A7B9CECA853768AEBB6965ECA126A62965F698A0C1BC43D83DB632AD7F717276057E6012AFA99385000000000100000000000000000027106F004630440220041038231F1C5E8D98EF941347BD9B6C220578128677BD561D258EC9B4CDFA3502203D18C43F7D06EE32D9527C8322F4D675F58856EC227ABF7085CCE65D5E5100C1019501000000040220D9AE2F000000001976A91455BDC1B42E3BED851959846DDF600E96125423E088AC0000000000000000536A4C500200000080A10BC28928F4C17A287318125115C3F098ED20A8237D1E8E4125BC25D1BE99752ADAD0A7B9CECA853768AEBB6965ECA126A62965F698A0C1BC43D83DB632AD7F717276057E6012AFA99385000000000100000000000000000027106F004630440220041038231F1C5E8D98EF941347BD9B6C220578128677BD561D258EC9B4CDFA3502203D18C43F7D06EE32D9527C8322F4D675F58856EC227ABF7085CCE65D5E5100C1019501000000040220D9AE2F000000001976A91455BDC1B42E3BED851959846DDF600E96125423E088AC0000000000000000536A4C500200000080A10BC28928F4C17A287318125115C3F098ED20A8237D1E8E4125BC25D1BE99752ADAD0A7B9CECA853768AEBB6965ECA126A62965F698A0C1BC43D83DB632AD7F717276057E6012AFA99385000000000100000000000000000027106F").unwrap();
        let apdu_vec = BtcApdu::btc_prepare(0x41, 0x00, &data);
        for apdu in apdu_vec {
            println!("apdu-->{:?}", apdu);
        }
    }
}
