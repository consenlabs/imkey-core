use crate::constants::{ETH_AID, LC_MAX};
use hex;
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
        apdu.push(0x00);
    } else {
        apdu.push(0x01);
    }
    apdu.push(path.clone().bytes().len() as u8); //Lc
    let mut temp_path_byte = path.as_bytes().to_vec(); //DATA
    apdu.append(&mut temp_path_byte);
    apdu.push(0x00); //Le
    println!("get xpub apdu -->{}", apdu.to_hex().to_uppercase());
    apdu.to_hex().to_uppercase()
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
select applet apdu
*/
pub fn select(aid: &Vec<u8>) -> String {
    let mut apdu = Vec::new();
    apdu.push(0x00);
    apdu.push(0xA4);
    apdu.push(0x04);
    apdu.push(0x00);
    apdu.push(aid.len() as u8);
    apdu.extend(aid.iter());
    apdu.push(0x00);
    apdu.to_hex().to_uppercase()
}

#[cfg(test)]
mod tests {
    use crate::apdu::get_xpub;

    #[test]
    fn get_xpub_test() {
        let path = String::from("m/44/0/0");
        let verify_flag = false;
        assert_eq!(
            get_xpub(&path, verify_flag),
            String::from("80430001086D2F34342F302F3000")
        );
    }
}
