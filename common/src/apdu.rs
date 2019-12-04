use crate::constants::{ETH_AID, LC_MAX};
use rustc_serialize::hex::ToHex;

pub struct Apdu {}

/**
获取xpub
*/
pub fn get_xpub(path: &String, verify_flag: bool) -> String {
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
    apdu.push(path.clone().into_bytes().len() as u8); //Lc
    let mut temp_path_byte = path.as_bytes().to_vec(); //DATA
    apdu.append(&mut temp_path_byte);
    apdu.push(0x00); //Le
    println!("get xpub apdu -->{}", apdu.to_hex().to_uppercase());
    apdu.to_hex().to_uppercase()
}

impl Apdu {
    pub fn eth_select() -> String {
        let mut apdu = Vec::new();
        apdu.push(0x00); //CLA
        apdu.push(0xA4); //INS
        apdu.push(0x04); //P1
        apdu.push(0x00); //P2
        apdu.push(ETH_AID.as_bytes().len() as u8);
        apdu.extend(ETH_AID.as_bytes().iter());
        apdu.push(0x00); //Le
        apdu.to_hex().to_uppercase()
    }

    pub fn eth_prepare(data: Vec<u8>) -> Vec<String> {
        let mut apdu_list = Vec::new();
        let size = data.len() as u32 / LC_MAX as u32
            + if data.len() as u32 % LC_MAX as u32 != 0 {
                1
            } else {
                0
            };

        for i in 0..size {
            let mut apdu = Vec::new();
            apdu.extend([0x80, 0x51].iter()); //CLA and ins
            let p1 = if i == 0 { 0x00 } else { 0x80 };
            let (p2, lc) = if i == size - 1 {
                (
                    0x80,
                    (data.len() as u32 - LC_MAX as u32 * (size - 1 as u32)) as u8,
                )
            } else {
                (0x00, 0xF5)
            };
            apdu.extend([p1, p2, lc].iter());
            //payload
            let payload =
                &data[(i as u32 * LC_MAX) as usize..(i as u32 * LC_MAX + lc as u32) as usize];
            apdu.extend(payload.iter());
            //le
            apdu.push(0x00);
            apdu_list.push(apdu.to_hex())
        }
        apdu_list
    }

    pub fn eth_pub(path: &String, verify_flag: bool) -> String {
        if path.as_bytes().len() > 256 {
            panic!("data to long");
        }

        let mut apdu = Vec::new();
        apdu.push(0x80); //CLA
        apdu.push(0x53); //INS
        apdu.push(0x00); //P1
        if verify_flag {
            apdu.push(0x00);
        } else {
            apdu.push(0x01);
        }
        apdu.push(path.clone().into_bytes().len() as u8); //Lc
        let mut temp_path_byte = path.as_bytes().to_vec(); //DATA
        apdu.append(&mut temp_path_byte);
        apdu.push(0x00); //Le
        apdu.to_hex().to_uppercase()
    }

    pub fn eth_sign(path: &String) -> String {
        if path.as_bytes().len() > 256 {
            panic!("data to long");
        }

        let mut apdu = Vec::new();
        apdu.push(0x80); //CLA
        apdu.push(0x52); //INS
        apdu.push(0x00); //P1-index
        apdu.push(0x00); //P2-hashtype
        apdu.push(path.as_bytes().len() as u8); //lc
        apdu.extend(path.as_bytes().iter()); //payload
        apdu.push(0x00); //le
        apdu.to_hex().to_uppercase()
    }
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
