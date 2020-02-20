#[cfg(target_os = "macos")]
use super::hid_api;
#[cfg(target_os = "macos")]
use hidapi::{HidApi, HidDevice};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
//use secp256k1::Secp256k1;
//extern crate android_logger;

lazy_static! {
    pub static ref APDU: Mutex<String> = Mutex::new("".to_string());
    pub static ref APDU_RETURN: Mutex<String> = Mutex::new("".to_string());
    pub static ref STRING: Mutex<String> = Mutex::new("".to_string());
    pub static ref CALLBACK: Mutex<extern "C" fn(*const i8) -> *const i8> =
        Mutex::new(default_callback);
}

#[cfg(target_os = "macos")]
lazy_static! {
    pub static ref DEVICE: Mutex<HidDevice> = Mutex::new(hid_api::hid_connect());
}

#[no_mangle]
pub extern "C" fn default_callback(apdu: *const c_char) -> *const c_char {
    CString::new("need set callback!".to_owned())
        .unwrap()
        .into_raw()
}

pub fn set_callback(callback: extern "C" fn(apdu: *const c_char) -> *const c_char) {
    let mut _callback = CALLBACK.lock().unwrap();
    *_callback = callback;
}

#[no_mangle]
pub extern "C" fn rust_hello(
    to: *const c_char,
    callback: extern "C" fn(apdu: *const c_char) -> *const c_char,
) -> *mut c_char {
    let cb = callback;
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    callback(
        CString::new("Hello bit 2223333 ".to_owned() + recipient)
            .unwrap()
            .into_raw(),
    );
    CString::new("Hello bit 2223333 ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn get_se_id(
    callback: extern "C" fn(apdu: *const c_char) -> *const c_char,
) -> *const c_char {
    callback(CString::new("00A4040000".to_owned()).unwrap().into_raw());
    callback(
        CString::new("80CB800005DFFF028101".to_owned())
            .unwrap()
            .into_raw(),
    )
}

// #[no_mangle]
// pub extern "C" fn get_se_id(callback: extern "C" fn(apdu:*const c_char)->*const c_char) -> *const c_char {
//     let functions = vec![callback];
//     functions[0](CString::new("00A4040000".to_owned())
//         .unwrap()
//         .into_raw());
//     callback(CString::new("80CB800005DFFF028101".to_owned())
//         .unwrap()
//         .into_raw())
// }

#[no_mangle]
pub extern "C" fn get_seid() -> *const c_char {
    get_seid_internal()
}

fn get_seid_internal() -> *const c_char {
    //debug!("get_seid_internal...");
    // set_apdu_r(String::from("00A4040000"));
    // get_apdu_return_r();
    // set_apdu_r(String::from("80CB800005DFFF028101"));
    // CString::new(get_apdu_return_r()).unwrap().into_raw()
    send_apdu(String::from("00A4040000"));
    let res = send_apdu(String::from("80CB800005DFFF028101"));
    CString::new(res).unwrap().into_raw()
}

// static mut VAR: i32 = 5;
// static mut STR:String = "22".to_string();

pub fn get_apdu() -> *const c_char {
    // loop{
    //     let mut apdu = APDU.lock().unwrap();
    //     if *apdu != ""{
    //         let temp = apdu.clone();
    //         *apdu = String::from("");
    //         return CString::new(temp.to_owned()).unwrap().into_raw();
    //     }
    // }
    // //debug!("set_apdu_r...{}",apdu);

    let apdu = APDU.lock().unwrap();
    return CString::new(apdu.to_owned()).unwrap().into_raw();
}

fn set_apdu_r(apdu: String) {
    //debug!("set_apdu_r...{}", apdu);
    println!("set_apdu_r...");
    loop {
        let mut _apdu = APDU.lock().unwrap();
        if *_apdu == "" {
            //debug!("is null set");
            println!("is null set");
            *_apdu = String::from(apdu.clone());
            break;
        } else {
            //debug!("not null...{}", _apdu);
            println!("not null...{}", _apdu);
        }
        // thread::sleep(Duration::from_millis(1000));
    }
}

pub fn set_apdu(apdu: *const c_char) {
    let mut _apdu = APDU.lock().unwrap();
    let c_str: &CStr = unsafe { CStr::from_ptr(apdu) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    //debug!("set_apdu...{}", str_buf);
    *_apdu = str_buf;
}

fn get_apdu_return_r() -> String {
    //debug!("get_apdu_return_r");
    loop {
        let mut apdu_return = APDU_RETURN.lock().unwrap();
        if *apdu_return != "" {
            //debug!("get_apdu_return_r not null {}", apdu_return.clone());
            let temp = apdu_return.clone();
            *apdu_return = String::from("");
            return String::from(temp.to_owned());
        } else {
            //debug!("get_apdu_return_r is null {}", apdu_return.clone());
        }
        // thread::sleep(Duration::from_millis(1000));
    }
}

pub fn get_apdu_return() -> *const c_char {
    let apdu = APDU_RETURN.lock().unwrap();
    //debug!("get_apdu_return...{}", apdu.clone());
    return CString::new(apdu.to_owned()).unwrap().into_raw();
}

pub fn set_apdu_return(apdu_return: *const c_char) {
    let mut _apdu_return = APDU_RETURN.lock().unwrap();
    let c_str: &CStr = unsafe { CStr::from_ptr(apdu_return) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    //debug!("set_apdu_return...{}", str_buf);
    *_apdu_return = str_buf;
}

// #[no_mangle]
// pub extern "C" fn set_apdu_return(apdu_return:*const c_char){
//     loop{
//         thread::sleep(Duration::from_millis(10));
//         //debug!("set_apdu_return...");
//         let mut _apdu_return = APDU_RETURN.lock().unwrap();
//         if *_apdu_return == ""{
//             //debug!("is null set...");
//             thread::sleep(Duration::from_millis(200));
//             //debug!("start set...");
//             let c_str: &CStr = unsafe { CStr::from_ptr(apdu_return) };
//             let str_slice: &str = c_str.to_str().unwrap();
//             let str_buf: String = str_slice.to_owned();
//             *_apdu_return = str_buf;
//             break;
//         }else{
//             //debug!("not null {}",_apdu_return);
//         }
//     }
// }

#[cfg(target_os = "macos")]
pub fn send_apdu(apdu: String) -> String {
    hid_api::send(&DEVICE.lock().unwrap(), &apdu)
}

#[cfg(target_os = "ios")]
pub fn send_apdu(apdu: String) -> String {
    set_apdu_r(apdu);
    get_apdu_return_r()
}

#[test]
fn test_str() {
    let mut string = STRING.lock().unwrap();

    // string.push_str("hello");
    *string = String::from("dereferenced22");
//    std::mem::drop(string);


    let mut str2 = STRING.lock().unwrap();
    // str2.push_str("hah");
    // println!("{}", str2);

    if *str2 == "" {
        println!("isnull {}", str2);
    } else {
        println!("{}", str2);
    }

    set_apdu_r("test".to_string());

    let hello = "hello123456".to_string();
    let s: String = hello.chars().skip(hello.len() - 4).take(4).collect();
    println!("s:{}", s);
}
