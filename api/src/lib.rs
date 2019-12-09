use std::ffi::{CStr, CString};
use std::os::raw::c_char;

//#[macro_use]
//extern crate log;
//extern crate android_logger;

use mq::message;

//use android_logger::{Config, FilterBuilder};
//use log::Level;
//use android_logger::{Config,FilterBuilder};
 use device::manager;

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

//should move out
fn get_seid_internal() -> *const c_char {
    message::send_apdu(String::from("00A4040000"));
    let res = message::send_apdu(String::from("80CB800005DFFF028101"));
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_apdu() -> *const c_char {
    message::get_apdu()
}

#[no_mangle]
pub extern "C" fn set_apdu(apdu: *const c_char) {
    message::set_apdu(apdu);
}

#[no_mangle]
pub extern "C" fn get_apdu_return() -> *const c_char {
    message::get_apdu_return()
}

#[no_mangle]
pub extern "C" fn set_apdu_return(apdu_return: *const c_char) {
    message::set_apdu_return(apdu_return);
}

//#[no_mangle]
//pub extern "C" fn init() {
//    android_logger::init_once(
//        Config::default()
//            .with_min_level(Level::Trace)
//            .with_tag("imkey")
//            .with_filter(
//                FilterBuilder::new()
//                    .parse("debug,hello::crate=trace")
//                    .build(),
//            ),
//    );
//}

#[no_mangle]
pub extern "C" fn check_device(){
    manager::check_device();
}




