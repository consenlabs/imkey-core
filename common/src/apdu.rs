use rustc_serialize::hex::ToHex;

/**
获取xpub
*/
pub fn get_xpub(path : & String, verify_flag : bool) -> String{

    if path.as_bytes().len() > 256 {
        panic!("data to long");
    }

    let mut apdu = Vec::new();
    apdu.push(0x80);//CLA
    apdu.push(0x43);//INS
    apdu.push(0x00);//P1
    //p2
    if verify_flag{
        apdu.push(0x00);
    }else {
        apdu.push(0x01);
    }
    apdu.push(path.clone().into_bytes().len() as u8);//Lc
    let mut temp_path_byte = path.as_bytes().to_vec();//DATA
    apdu.append(&mut temp_path_byte);
    apdu.push( 0x00);//Le
    println!("get xpub apdu -->{}", apdu.to_hex().to_uppercase());
    apdu.to_hex().to_uppercase()
}


#[cfg(test)]
mod tests {
    use crate::apdu::get_xpub;

    #[test]
    fn get_xpub_test() {
        let path = String::from("m/44/0/0");
        let verify_flag = false;
        assert_eq!(get_xpub(&path, verify_flag), String::from("80430001086D2F34342F302F3000"));
    }

}


