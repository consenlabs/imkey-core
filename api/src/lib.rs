use crate::api::{TcxAction, Response};
use crate::wallet_handler::{device_manage, get_address, register_coin, sign_tx, encode_message};
use common::error::Error;
use prost::Message;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
pub mod api;
pub mod btc_signer;
pub mod btcapi;
pub mod cosmos_address;
pub mod cosmos_signer;
//pub mod cosmosapi;
pub mod device_manager;
pub mod deviceapi;
pub mod eos_pubkey;
pub mod eos_signer;
pub mod error_handling;
pub mod ethapi;
pub mod ethereum;
pub mod ethereum_address;
pub mod ethereum_signer;
pub mod wallet_handler;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate log;


use crate::error_handling::{landingpad, Result, LAST_BACKTRACE, LAST_ERROR};

//#[macro_use]
//extern crate log;
//extern crate android_logger;

use mq::message;

//use android_logger::{Config, FilterBuilder};
//use log::Level;
//use android_logger::{Config,FilterBuilder};
use device::manager;

#[no_mangle]
/*
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
*/

#[no_mangle]
/*pub extern "C" fn rust_hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}*/

#[no_mangle]
/*pub extern "C" fn get_se_id(
    callback: extern "C" fn(apdu: *const c_char) -> *const c_char,
) -> *const c_char {
    callback(CString::new("00A4040000".to_owned()).unwrap().into_raw());
    callback(
        CString::new("80CB800005DFFF028101".to_owned())
            .unwrap()
            .into_raw(),
    )
}*/

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
pub extern "C" fn check_device() {
    manager::check_device();
}

#[no_mangle]
pub extern "C" fn active_device() {
    manager::active_device();
}

#[no_mangle]
pub extern "C" fn check_update() {
    manager::check_update();
}

#[no_mangle]
pub extern "C" fn app_download() {
    manager::app_download();
}

#[no_mangle]
pub extern "C" fn app_update() {
    manager::app_update();
}

#[no_mangle]
pub extern "C" fn app_delete() {
    manager::app_delete();
}


/// dispatch protobuf rpc call
/// //@@XM TODO: add in error handling
#[no_mangle]
pub unsafe extern "C" fn call_tcx_api(hex_str: *const c_char) -> *const c_char {
    let hex_c_str = CStr::from_ptr(hex_str);
    let hex_str = hex_c_str.to_str().expect("parse_arguments to_str");

    let data = hex::decode(hex_str).expect("parse_arguments hex decode");
    let action: TcxAction = TcxAction::decode(data).expect("decode tcx api");
    let reply: Vec<u8> = match action.method.to_lowercase().as_str() {
        "sign_tx" => sign_tx(&action.param.unwrap().value).unwrap(),
        "get_address" => get_address(&action.param.unwrap().value).unwrap(),
        "device_manage" => device_manage(&action.param.unwrap().value).unwrap(),
        "register_coin" => register_coin(&action.param.unwrap().value).unwrap(),
        _ => Vec::new(), //@@XM TODO: change to error message
                         /*
                         "sign_tx" => landingpad(|| sign_tx(&action.param.unwrap().value)),

                         "get_address" => landingpad(|| get_address(&action.param.unwrap().value)),

                         _ => landingpad(|| Err(format_err!("unsupported_method"))),
                         */
    };

    let ret_str = hex::encode(reply);
    CString::new(ret_str).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn clear_err() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
    LAST_BACKTRACE.with(|e| {
        *e.borrow_mut() = None;
    });
}

#[no_mangle]
pub unsafe extern "C" fn get_last_err_message() -> *const c_char {
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            let rsp = Response {
                is_success: false,
                error: err.to_string(),
            };
            // eprintln!("{:#?}", rsp);
            let rsp_bytes = encode_message(rsp).expect("encode error");
            let ret_str = hex::encode(rsp_bytes);
            CString::new(ret_str).unwrap().into_raw()
        } else {
            CString::new("").unwrap().into_raw()
        }
    })
}
