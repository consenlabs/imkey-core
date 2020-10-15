use crate::api::{ErrorResponse, ImkeyAction};
use prost::Message;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
pub mod api;
pub mod btc_address;
pub mod btc_signer;
pub mod cosmos_address;
pub mod cosmos_signer;
pub mod device_manager;
pub mod eos_pubkey;
pub mod eos_signer;
pub mod error_handling;
pub mod ethereum_address;
pub mod ethereum_signer;
pub mod filecoin_address;
pub mod filecoin_signer;
pub mod message_handler;
pub mod usdt_signer;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;
use crate::error_handling::{landingpad, LAST_BACKTRACE, LAST_ERROR};
use crate::message_handler::encode_message;
use transport::message;

lazy_static! {
    pub static ref API_LOCK: Mutex<String> = Mutex::new("".to_string());
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

#[no_mangle]
pub extern "C" fn set_callback(
    callback: extern "C" fn(apdu: *const c_char, timeout: i32) -> *const c_char,
) {
    message::set_callback(callback);
}

#[no_mangle]
pub unsafe extern "C" fn free_const_string(s: *const c_char) {
    if s.is_null() {
        return;
    }
    CStr::from_ptr(s);
}

/// dispatch protobuf rpc call
#[no_mangle]
pub unsafe extern "C" fn call_imkey_api(hex_str: *const c_char) -> *const c_char {
    let mut _l = API_LOCK.lock().unwrap();
    let hex_c_str = CStr::from_ptr(hex_str);
    let hex_str = hex_c_str.to_str().expect("parse_arguments to_str");

    let data = hex::decode(hex_str).expect("imkey_illegal_prarm");
    let action: ImkeyAction = ImkeyAction::decode(data.as_slice()).expect("decode imkey api");
    let reply: Vec<u8> = match action.method.to_lowercase().as_str() {
        // imkey manager
        "app_download" => landingpad(|| device_manager::app_download(&action.param.unwrap().value)),
        "app_update" => landingpad(|| device_manager::app_update(&action.param.unwrap().value)),
        "app_delete" => landingpad(|| device_manager::app_delete(&action.param.unwrap().value)),
        "device_activate" => landingpad(|| device_manager::se_activate()),
        "check_update" => landingpad(|| device_manager::check_update()),
        "device_secure_check" => landingpad(|| device_manager::se_secure_check()),
        "bind_check" => landingpad(|| device_manager::bind_check(&action.param.unwrap().value)),
        "bind_display_code" => landingpad(|| device_manager::bind_display_code()),
        "bind_acquire" => landingpad(|| device_manager::bind_acquire(&action.param.unwrap().value)),
        "get_seid" => landingpad(|| device_manager::get_seid()),
        "get_sn" => landingpad(|| device_manager::get_sn()),
        "get_ram_size" => landingpad(|| device_manager::get_ram_size()),
        "get_firmware_version" => landingpad(|| device_manager::get_firmware_version()),
        "get_battery_power" => landingpad(|| device_manager::get_battery_power()),
        "get_life_time" => landingpad(|| device_manager::get_life_time()),
        "get_ble_name" => landingpad(|| device_manager::get_ble_name()),
        "set_ble_name" => landingpad(|| device_manager::set_ble_name(&action.param.unwrap().value)),
        "get_ble_version" => landingpad(|| device_manager::get_ble_version()),
        "get_sdk_info" => landingpad(|| device_manager::get_sdk_info()),
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        "cos_update" => landingpad(|| device_manager::cos_update()),
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        "cos_check_update" => landingpad(|| device_manager::cos_check_update()),
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        "device_connect" => {
            landingpad(|| device_manager::device_connect(&action.param.unwrap().value))
        }
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        "is_bl_status" => landingpad(|| device_manager::is_bl_status()),

        // btc
        "btc_tx_sign" => {
            landingpad(|| btc_signer::sign_btc_transaction(&action.param.unwrap().value))
        }
        "btc_segwit_tx_sign" => {
            landingpad(|| btc_signer::sign_segwit_transaction(&action.param.unwrap().value))
        }
        "btc_usdt_tx_sign" => {
            landingpad(|| usdt_signer::sign_usdt_transaction(&action.param.unwrap().value))
        }
        "btc_usdt_segwit_tx_sign" => {
            landingpad(|| usdt_signer::sign_usdt_segwit_transaction(&action.param.unwrap().value))
        }
        "btc_get_xpub" => landingpad(|| btc_address::get_btc_xpub(&action.param.unwrap().value)),
        "btc_get_address" => {
            landingpad(|| btc_address::get_btc_address(&action.param.unwrap().value))
        }
        "btc_get_setwit_address" => {
            landingpad(|| btc_address::get_segwit_address(&action.param.unwrap().value))
        }
        "btc_register_address" => {
            landingpad(|| btc_address::display_btc_address(&action.param.unwrap().value))
        }
        "btc_register_segwit_address" => {
            landingpad(|| btc_address::display_segwit_address(&action.param.unwrap().value))
        }

        // eth
        "eth_tx_sign" => {
            landingpad(|| ethereum_signer::sign_eth_transaction(&action.param.unwrap().value))
        }
        "eth_message_sign" => {
            landingpad(|| ethereum_signer::sign_eth_message(&action.param.unwrap().value))
        }
        "eth_get_address" => {
            landingpad(|| ethereum_address::get_eth_address(&action.param.unwrap().value))
        }
        "eth_register_address" => {
            landingpad(|| ethereum_address::display_eth_address(&action.param.unwrap().value))
        }

        // eos
        "eos_tx_sign" => {
            landingpad(|| eos_signer::sign_eos_transaction(&action.param.unwrap().value))
        }
        "eos_message_sign" => {
            landingpad(|| eos_signer::sign_eos_message(&action.param.unwrap().value))
        }
        "eos_get_pubkey" => landingpad(|| eos_pubkey::get_eos_pubkey(&action.param.unwrap().value)),
        "eos_register_pubkey" => {
            landingpad(|| eos_pubkey::display_eos_pubkey(&action.param.unwrap().value))
        }

        // cosmos
        "cosmos_tx_sign" => {
            landingpad(|| cosmos_signer::sign_cosmos_transaction(&action.param.unwrap().value))
        }
        "cosmos_get_address" => {
            landingpad(|| cosmos_address::get_cosmos_address(&action.param.unwrap().value))
        }
        "cosmos_register_address" => {
            landingpad(|| cosmos_address::display_cosmos_address(&action.param.unwrap().value))
        }

        // filecoin
        "filecoin_tx_sign" => {
            landingpad(|| filecoin_signer::sign_filecoin_transaction(&action.param.unwrap().value))
        }
        "filecoin_get_address" => {
            landingpad(|| filecoin_address::get_filecoin_address(&action.param.unwrap().value))
        }
        "filecoin_register_address" => {
            landingpad(|| filecoin_address::display_filecoin_address(&action.param.unwrap().value))
        }

        _ => Vec::new(),
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
            let rsp = ErrorResponse {
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
