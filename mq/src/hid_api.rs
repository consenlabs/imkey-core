use hex::FromHex;
use std::thread::sleep;
use std::time::Duration;
use std::sync::Mutex;
use crate::message::DEVICE;
use hidapi::{HidApi, HidDevice};

lazy_static! {
    pub static ref HID_API: Mutex<HidApi> = Mutex::new(HidApi::new().unwrap());
}

const RETRY_SEC: u64 = 1;
const RETRY_SEC1: u64 = 30;
const DEV_VID: u16 = 0x096e;
const DEV_PID: u16 = 0x0891;

#[no_mangle]
pub enum Error {
    /// Ethereum wallet protocol error.
    Protocol(&'static str),
    /// Hidapi error.
    Usb(hidapi::HidError),
    /// Device with request key is not available.
    KeyNotFound,
    /// Signing has been cancelled by user.
    UserCancel,
    /// The Message Type given in the trezor RPC call is not something we recognize
    BadMessageType,
    /// Trying to read from a closed device at the given path
    ClosedDevice(String),
}

#[no_mangle]
pub fn hid_send(hid_device: &HidDevice, apdu: &String) -> String {
    println!("-->{}", apdu);
    send_device_message(hid_device, Vec::from_hex(apdu.as_str()).unwrap().as_slice());
    let return_data = read_device_response(hid_device).ok().unwrap();
    let apdu_response = hex::encode_upper(return_data);
    println!("<--{}", apdu_response.clone());
    return apdu_response;
}


#[no_mangle]
fn first_write_read_device_response(device: &hidapi::HidDevice) -> Result<(Vec<u8>), Error> {
    let protocol_err = Error::Protocol(&"Unexpected wire response from imkey Device");
    let firstSendcmd: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x01, 0x86, 0x00, 0x00];
    let mut send_first_data_string = String::new();
    for u in &firstSendcmd[..firstSendcmd.len()] {
        send_first_data_string.push_str((format!("{:02X}", u)).as_ref());
    }
    //    println!("{}", "send first cmd-->".to_owned()+ &send_first_data_string);
    let res = device.write(&firstSendcmd);

    let mut buf = vec![0; 64];
    let first_chunk = device.read_timeout(&mut buf, 300_000);

    let mut receive_first_data_string = String::new();
    for u in &buf[..buf.len()] {
        receive_first_data_string.push_str((format!("{:02X}", u)).as_ref());
    }
    //    println!("{}", "receive first res-->".to_owned()+&receive_first_data_string);

    Ok(buf[..64].to_vec())
}

#[no_mangle]
fn read_device_response(device: &hidapi::HidDevice) -> Result<(Vec<u8>), Error> {
    let protocol_err = Error::Protocol(&"Unexpected wire response from imkey Device");
    let mut buf = vec![0; 64];

    let first_res = device.read_timeout(&mut buf, 300_000);
    if (first_res.is_err()) {
        return Err(protocol_err);
    }
    let msg_size = (buf[5] as u8 & 0xFF) + (buf[6] as u8 & 0xFF);
    let mut data = Vec::new();
    data.extend_from_slice(&buf[7..]);
    while data.len() < (msg_size as usize) {
        //        println!("{}", data.len() as usize);
        let res = device.read_timeout(&mut buf, 10_000);
        if (res.is_err()) {
            return Err(protocol_err);
        }

        data.extend_from_slice(&buf[5..64]);
    }
    data.truncate(msg_size as usize);

    //打印收到的数据
    let mut receive_data_string = String::new();
    for u in &data[..data.len()] {
        receive_data_string.push_str((format!("{:02X}", u)).as_ref());
    }
    //    println!("{}", "receive-->".to_owned()+&receive_data_string);

    Ok(data[..msg_size as usize].to_vec())
}

#[no_mangle]
fn send_device_message(device: &hidapi::HidDevice, msg: &[u8]) -> Result<usize, Error> {
    let protocol_err = Error::Protocol(&"Unexpected wire response from imkey Device");
    let mut send_data_string = String::new();
    for u in &msg[..msg.len()] {
        send_data_string.push_str((format!("{:02X}", u)).as_ref());
    }
    //    println!("{}", "send-->".to_owned()+&send_data_string);

    let msg_size = msg.len();
    let mut headerdata = Vec::new();
    // first pack
    headerdata.push(0x00 as u8);
    headerdata.push(0x00 as u8);
    headerdata.push(0x00 as u8);
    headerdata.push(0x00 as u8);
    headerdata.push(0x01 as u8);
    headerdata.push(0x83 as u8);
    headerdata.push((msg_size & 0xFF00) as u8);
    headerdata.push((msg_size & 0x00FF) as u8);
    let mut data = Vec::new();
    if ((msg_size + 8) < 65) {
        data.extend_from_slice(&headerdata[0..8]);
        data.extend_from_slice(&msg[0..msg_size]);
    } else {
        let mut datalenflage = 0;
        let mut flg = 0;
        while (true) {
            if (msg_size - datalenflage < 65 - 8) {
                data.extend_from_slice(&headerdata[0..5]);
                data.push(flg as u8);
                data.extend_from_slice(&msg[datalenflage..msg_size]);
                datalenflage += msg_size - datalenflage;
                break;
            } else {
                if !(datalenflage == 0) {
                    data.extend_from_slice(&headerdata[0..5]);
                    data.push(flg as u8);
                    flg = 1 + flg;
                    data.extend_from_slice(&msg[datalenflage..datalenflage + 65 - 6]);
                    datalenflage += (65 - 6);
                } else {
                    data.extend_from_slice(&headerdata[0..8]);
                    data.extend_from_slice(&msg[datalenflage..65 - 8]);
                    datalenflage += (65 - 8);
                }
            }
        }
    }

    while data.len() % 65 > 0 {
        data.push(0);
    }

    let mut total_written = 0;
    for chunk in data.chunks(65) {
        let res = device.write(&chunk);
        if (res.is_err()) {
            return Err(protocol_err);
        }
    }
    Ok(total_written)
}

#[no_mangle]
pub fn hid_connect() -> HidDevice {

    let hid_api_obj = HID_API.lock().unwrap();
loop {
        match hid_api_obj.open(DEV_VID, DEV_PID) {
            Ok(dev) => {
                println!("device connected!!!");
                //first_write_read_device_response(&dev);
//                let hid_device_obj = DEVICE.lock().unwrap();
//                *hid_device_obj = dev.clone();
                return dev;
            }
            Err(err) => {
                println!("{}", err);
                sleep(Duration::from_secs(RETRY_SEC));
            }
        }
    }
}

#[cfg(test)]
mod test{
    use crate::hid_api;
    use crate::message::send_apdu;

    #[test]
    fn hid_test(){
//        let hid_device = hid_api::hid_connect();
//        hid_api::hid_send(&hid_device, &"00A4040005695F62746300".to_string());
        send_apdu("00A4040005695F62746300".to_string());

    }
}
