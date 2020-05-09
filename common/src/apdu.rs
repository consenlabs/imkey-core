use crate::constants::{BTC_AID, COSMOS_AID, EOS_AID, ETH_AID, LC_MAX};
use crate::error::ApduError;
use crate::Result;
use hex;
use rustc_serialize::hex::ToHex;

pub trait CoinCommonApdu: Default{
    fn select_applet() -> String;
    fn get_xpub(path: &str, verify_flag: bool) -> String;
    fn register_address(address: &[u8]) -> String;
}

pub struct BtcApdu();

impl Default for BtcApdu{
    fn default() -> Self {
        BtcApdu{}
    }
}

impl CoinCommonApdu for BtcApdu{

    fn select_applet() -> String {
        Apdu::select_applet(BTC_AID)
    }

    fn get_xpub(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x43, path, verify_flag)
    }

    fn register_address(address: &[u8]) -> String {
        Apdu::register_address(0x36, address)
    }
}

impl BtcApdu{
    pub fn btc_prepare(ins: u8, p1: u8, data: &Vec<u8>) -> Vec<String> {
        let mut apdu_vec = Vec::new();
        let apdu_number = (data.len() - 1) / LC_MAX as usize + 1;
        for index in 0..apdu_number {
            if index == apdu_number - 1 {
                let length = if data.len() % LC_MAX as usize == 0 {
                    LC_MAX
                } else {
                    (data.len() % LC_MAX as usize) as u32
                };
                let mut temp_apdu_vec =ApduHeader::new(0x80, ins, p1, 0x80, length as u8).to_array();
                temp_apdu_vec.extend_from_slice(&data[index * LC_MAX as usize..]);
                apdu_vec.push(hex::encode_upper(temp_apdu_vec));
            } else {
                let mut temp_apdu_vec =ApduHeader::new(0x80, ins, p1, 0x00, LC_MAX as u8).to_array();
                temp_apdu_vec.extend_from_slice(
                    &data[index * LC_MAX as usize..((index + 1) * LC_MAX as usize) as usize],
                );
                apdu_vec.push(hex::encode_upper(temp_apdu_vec));
            }
        }
        return apdu_vec;
    }

    pub fn btc_perpare_input(p1: u8, data: &Vec<u8>) -> String {
        if data.len() as u32 > LC_MAX {
            panic!("data to long");
        }
        let mut apdu =ApduHeader::new(0x80, 0x41, p1, 0x00, data.len() as u8).to_array();
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn btc_sign(index: u8, hash_type: u8, path: &str) -> String {
        let path_bytes = path.as_bytes();
        let mut apdu =ApduHeader::new(0x80, 0x42, index, hash_type, path_bytes.len() as u8).to_array();
        apdu.extend(path_bytes.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn btc_segwit_sign(last_one: bool, hash_type: u8, data: Vec<u8>) -> String {
        if data.len() as u32 > LC_MAX {
            panic!("data to long");
        }

        let mut apdu = match last_one{
            true => ApduHeader::new(0x80, 0x32, 0x80, hash_type, data.len() as u8).to_array(),
            _ => ApduHeader::new(0x80, 0x32, 0x00, hash_type, data.len() as u8).to_array(),
        };

        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn omni_prepare_data(p1: u8, data: Vec<u8>) -> String {
        if data.len() as u32 > LC_MAX {
            panic!("data to long");
        }
        let mut apdu = ApduHeader::new(0x80, 0x44, p1, 0x00, data.len() as u8).to_array();
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }
}

pub struct EthApdu();

impl Default for EthApdu{
    fn default() -> Self {
        EthApdu()
    }
}

impl CoinCommonApdu for EthApdu{

    fn select_applet() -> String {
        Apdu::select_applet(ETH_AID)
    }

    fn get_xpub(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x53, path, verify_flag)
    }

    fn register_address(address: &[u8]) -> String {
        Apdu::register_address(0x56, address)
    }
}

impl EthApdu{
    pub fn prepare_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x51, data)
    }

    pub fn sign_digest(path: &str) -> String {
        Apdu::sign_digest(0x52, 0x00, 0x00, path)
    }

    pub fn prepare_personal_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x54, data)
    }

    pub fn personal_sign(path: &str) -> String {
        Apdu::sign_digest(0x55, 0x00, 0x00, path)
    }
}

pub struct EosApdu();

impl Default for EosApdu{
    fn default() -> Self {
        EosApdu()
    }
}

impl CoinCommonApdu for EosApdu{

    fn select_applet() -> String {
        Apdu::select_applet(EOS_AID)
    }

    fn get_xpub(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x63, path, verify_flag)
    }

    fn register_address(address: &[u8]) -> String {
        Apdu::register_address(0x66, address)
    }
}

impl EosApdu{

    pub fn prepare_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x61, data)
    }

    pub fn sign_digest(path: &str) -> String {
        Apdu::sign_digest(0x52, 0x00, 0x00, path)
    }

    pub fn sign_tx(nonce: usize) -> String {
        let mut apdu = ApduHeader::new(0x80, 0x62, 0x00, 0x00, 0x02).to_array();
        apdu.push(((nonce & 0xFF00) >> 8) as u8);
        apdu.push((nonce & 0x00FF) as u8);
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn prepare_message_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x64, data)
    }

    pub fn sign_message(nonce: usize) -> String {
        let mut apdu = ApduHeader::new(0x80, 0x65, 0x00, 0x00, 0x02).to_array();
        apdu.push(((nonce & 0xFF00) >> 8) as u8);
        apdu.push((nonce & 0x00FF) as u8);
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }
}

pub struct CosmosApdu();

impl Default for CosmosApdu{
    fn default() -> Self {
        CosmosApdu()
    }
}

impl CoinCommonApdu for CosmosApdu{

    fn select_applet() -> String {
        Apdu::select_applet(COSMOS_AID)
    }

    fn get_xpub(path: &str, verify_flag: bool) -> String {
        Apdu::get_pubkey(0x73, path, verify_flag)
    }

    fn register_address(address: &[u8]) -> String {
        Apdu::register_address(0x76, address)
    }
}

impl CosmosApdu{
    pub fn prepare_sign(data: Vec<u8>) -> Vec<String> {
        Apdu::prepare_sign(0x71, data)
    }

    pub fn sign_digest(path: &str) -> String {
        Apdu::sign_digest(0x72, 0x00, 0x00, path)
    }
}


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
    pub fn select_applet(aid: &str) -> String {
        let aid_array = hex::decode(aid).unwrap();
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
        if path_bytes.len() as u32 > LC_MAX {
            panic!("data to long");
        }
        let p1 = if verify_flag { 0x01 } else { 0x00 };
        let mut apdu = ApduHeader::new(0x80, ins, p1, 0x00, path_bytes.len() as u8).to_array();
        apdu.extend(path_bytes.iter());
        apdu.push(0x00); //Le
        hex::encode(apdu)
    }

    pub fn register_address(ins: u8, data: &[u8]) -> String {
        if data.len() as u32 > LC_MAX {
            panic!("data to long");
        }
        let mut apdu = ApduHeader::new(0x80, ins, 0x00, 0x00, data.len() as u8).to_array();
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    pub fn sign_digest(ins: u8, index: u8, hashtype: u8, path: &str) -> String {
        let path_bytes = path.as_bytes();
        if path_bytes.len() as u32 > LC_MAX {
            panic!("data to long");
        }

        let mut apdu = Vec::new();

        let apdu_header = ApduHeader::new(0x80, ins, index, hashtype, path_bytes.len() as u8);
        apdu.extend(apdu_header.to_array().iter());
        apdu.extend(path_bytes.iter()); //payload
        apdu.push(0x00); //le
        hex::encode(apdu)
    }

    pub fn set_ble_name(ble_name: &str) -> String {
        let ble_name_array = ble_name.as_bytes();
        let mut apdu = Vec::new();
        let apdu_header = ApduHeader::new(0xFF, 0xDA, 0x46, 0x54, ble_name_array.len() as u8);
        apdu.extend(apdu_header.to_array().iter());
        apdu.extend(ble_name_array.iter());
        apdu.push(0x00); //Le
        hex::encode(apdu)
    }
}

pub struct ImkApdu {}

impl ImkApdu {
    /**
    binding check apdu build
    */
    pub fn bind_check(data: &Vec<u8>) -> String {
        if data.len() as u32 > LC_MAX {
            panic!("data to long");
        }
        let mut apdu = ApduHeader::new(0x80, 0x71, 0x00, 0x00, data.len() as u8).to_array();
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }

    /**
    binding check apdu build
    */
    pub fn generate_auth_code() -> String {
        let apdu = ApduHeader::new(0x80, 0x72, 0x00, 0x00, 0x00).to_array();
        apdu.to_hex().to_uppercase()
    }

    /**
    bind code verify
    */
    pub fn identity_verify(data: &Vec<u8>) -> String {
        if data.len() as u32 > LC_MAX {
            panic!("data to long");
        }
        let mut apdu = ApduHeader::new(0x80, 0x73, 0x80, 0x00, data.len() as u8).to_array();
        apdu.extend(data.iter());
        apdu.push(0x00);
        apdu.to_hex().to_uppercase()
    }
}

pub struct ApduCheck {}

impl ApduCheck {
    pub fn checke_response(response_data: &str) -> Result<()> {
        let response_data: &str = &response_data[response_data.len() - 4..];
        match response_data {
            "9000" => Ok(()),
            "6940" => Err(ApduError::ImkeyUserNotConfirmed.into()),
            "6985" => Err(ApduError::ImkeyConditionsNotSatisfied.into()),
            "6A82" => Err(ApduError::ImkeyAppletNotExist.into()),
            "6A86" => Err(ApduError::ImkeyCommandFormatError.into()),
            "6E00" => Err(ApduError::ImkeyCommandFormatError.into()),
            "6A80" => Err(ApduError::ImkeyCommandDataError.into()),
            "6700" => Err(ApduError::ImkeyApduWrongLength.into()),
            "6942" => Err(ApduError::ImkeySignatureVerifyFail.into()),
            "6D00" => Err(ApduError::ImkeyAppletFunctionNotSupported.into()),
            "6941" => Err(ApduError::ImkeyExceededMaxUtxoNumber.into()),
            "F000" => Err(ApduError::ImkeyWalletNotCreated.into()),
            "F080" => Err(ApduError::ImkeyInMenuPage.into()),
            "F081" => Err(ApduError::ImkeyPinNotVerified.into()),
            "6F01" => Err(ApduError::ImkeyBluetoothChannelError.into()),
            _ => Err(format_err!("imkey_command_execute_fail_{}", response_data)), //Err(ApduError::ImkeyCommandExecuteFail.into())
        }
    }
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
