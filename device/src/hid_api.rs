extern crate hidapi;

use hex::FromHex;
use hidapi::{HidApi, HidDevice};
use std::thread::sleep;
use std::time::Duration;

const RETRY_SEC: u64 = 1;
const RETRY_SEC1: u64 = 30;
const DEV_VID: u16 = 0x096e;
const DEV_PID: u16 = 0x0891;

fn main() {
    let hid_device = connect();
    let apdu = "00A40400".to_string();
    let response = send(&hid_device, &apdu);
    println!("{:?}", response);

    //    println!("Execution Successful, auto-exit in 30 seconds.");
    //    //等待20秒
    //    sleep(Duration::from_secs(RETRY_SEC1));
}

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
pub fn send(hid_device: &HidDevice, apdu: &String) -> String {
    println!("-->{}", apdu);
    //    let temp_apdu = Vec::from_hex(apdu.as_str()).unwrap().as_slice();

    send_device_message(hid_device, Vec::from_hex(apdu.as_str()).unwrap().as_slice());
    let return_data = read_device_response(hid_device).ok().unwrap();
    println!("<--{}", hex::encode_upper(return_data.clone()));
    let hex_str = hex::encode_upper(return_data.clone());
    hex_str.chars().take(hex_str.len() - 4).collect()
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
    if ((msg_size + 8) < 64) {
        data.extend_from_slice(&headerdata[0..8]);
        data.extend_from_slice(&msg[0..msg_size]);
    } else {
        let mut datalenflage = 0;
        while (true) {
            if (msg_size - datalenflage < 64 - 8) {
                data.extend_from_slice(&headerdata[0..5]);
                data.push(0x00 as u8);
                data.extend_from_slice(&msg[datalenflage..msg_size]);
                datalenflage += msg_size - datalenflage;
                break;
            } else {
                if (datalenflage == 0) {
                    data.extend_from_slice(&headerdata[0..8]);
                    data.extend_from_slice(&msg[datalenflage..64 - 8]);
                    datalenflage += (64 - 8);
                } else {
                    data.extend_from_slice(&headerdata[0..5]);
                    data.push(0x00 as u8);
                    data.extend_from_slice(&msg[datalenflage..64 - 6]);
                    datalenflage += (64 - 6);
                }
            }
        }
    }

    while data.len() % 64 > 0 {
        data.push(0);
    }

    let mut total_written = 0;
    for chunk in data.chunks(64) {
        let res = device.write(&chunk);
        if (res.is_err()) {
            return Err(protocol_err);
        }
    }
    Ok(total_written)
}
#[no_mangle]
pub fn connect() -> HidDevice {
    let api = HidApi::new().expect("HID API object creation failed");

    loop {
        match api.open(DEV_VID, DEV_PID) {
            Ok(dev) => {
                println!("device connected!!!");
                first_write_read_device_response(&dev);
                return dev;
            }
            Err(err) => {
                println!("{}", err);
                sleep(Duration::from_secs(RETRY_SEC));
            }
        }
    }
}
