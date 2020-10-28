use crate::api::{AddressParam, ErrorResponse, ImkeyAction, PubKeyParam, SignParam};
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
pub unsafe extern "C" fn imkey_free_const_string(s: *const c_char) {
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
        "init_imkey_core_x" => {
            landingpad(|| device_manager::init_imkey_core(&action.param.unwrap().value))
        }
        // imkey manager
        "app_download" => landingpad(|| device_manager::app_download(&action.param.unwrap().value)),
        "app_update" => landingpad(|| device_manager::app_update(&action.param.unwrap().value)),
        "app_delete" => landingpad(|| device_manager::app_delete(&action.param.unwrap().value)),
        "device_activate" => landingpad(|| device_manager::se_activate()),
        "check_update" => landingpad(|| device_manager::check_update()),
        "device_secure_check" => landingpad(|| device_manager::se_secure_check()),
        "bind_check" => landingpad(|| device_manager::bind_check()),
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

        "get_address" => landingpad(|| {
            let param: AddressParam = AddressParam::decode(action.param.unwrap().value.as_slice())
                .expect("imkey_illegal_param");
            match param.chain_type.as_str() {
                "BITCOIN" => btc_address::get_address(&param),
                "ETHEREUM" => ethereum_address::get_address(&param),
                "COSMOS" => cosmos_address::get_address(&param),
                "FILECOIN" => filecoin_address::get_address(&param),
                _ => Err(format_err!("get_address unsupported_chain")),
            }
        }),

        "get_pub_key" => landingpad(|| {
            let param: PubKeyParam = PubKeyParam::decode(action.param.unwrap().value.as_slice())
                .expect("imkey_illegal_param");
            match param.chain_type.as_str() {
                "EOS" => eos_pubkey::get_eos_pubkey(&param),
                _ => Err(format_err!("get_pub_key unsupported_chain")),
            }
        }),

        "register_pub_key" => landingpad(|| {
            let param: PubKeyParam = PubKeyParam::decode(action.param.unwrap().value.as_slice())
                .expect("imkey_illegal_param");
            match param.chain_type.as_str() {
                "EOS" => eos_pubkey::display_eos_pubkey(&param),
                _ => Err(format_err!("register_pub_key unsupported_chain")),
            }
        }),

        "register_address" => landingpad(|| {
            let param: AddressParam = AddressParam::decode(action.param.unwrap().value.as_slice())
                .expect("imkey_illegal_param");
            match param.chain_type.as_str() {
                "BITCOIN" => btc_address::register_btc_address(&param),
                "ETHEREUM" => ethereum_address::register_address(&param),
                "COSMOS" => cosmos_address::display_cosmos_address(&param),
                "FILECOIN" => filecoin_address::display_filecoin_address(&param),
                _ => Err(format_err!("register_address unsupported_chain")),
            }
        }),

        "sign_tx" => landingpad(|| {
            let param: SignParam = SignParam::decode(action.param.unwrap().value.as_slice())
                .expect("sign_tx unpack error");
            match param.chain_type.as_str() {
                "BITCOIN" => btc_signer::sign_btc_transaction(&param.input.unwrap().value),
                "ETHEREUM" => ethereum_signer::sign_eth_transaction(&param.input.unwrap().value),
                "EOS" => eos_signer::sign_eos_transaction(&param.input.unwrap().value),
                "COSMOS" => cosmos_signer::sign_cosmos_transaction(&param.input.unwrap().value),
                "FILECOIN" => {
                    filecoin_signer::sign_filecoin_transaction(&param.input.unwrap().value)
                }
                _ => Err(format_err!("register_address unsupported_chain")),
            }
        }),

        // btc
        // "btc_tx_sign" => {
        //     landingpad(|| btc_signer::sign_btc_transaction(&action.param.unwrap().value))
        // }
        // "btc_segwit_tx_sign" => {
        //     landingpad(|| btc_signer::sign_segwit_transaction(&action.param.unwrap().value))
        // }
        "btc_usdt_tx_sign" => {
            landingpad(|| usdt_signer::sign_usdt_transaction(&action.param.unwrap().value))
        }
        "btc_usdt_segwit_tx_sign" => {
            landingpad(|| usdt_signer::sign_usdt_segwit_transaction(&action.param.unwrap().value))
        }
        "btc_get_xpub" => landingpad(|| btc_address::get_btc_xpub(&action.param.unwrap().value)),

        // // eth
        // "eth_tx_sign" => {
        //     landingpad(|| ethereum_signer::sign_eth_transaction(&action.param.unwrap().value))
        // }
        "eth_message_sign" => {
            landingpad(|| ethereum_signer::sign_eth_message(&action.param.unwrap().value))
        }

        // // eos
        // "eos_tx_sign" => {
        //     landingpad(|| eos_signer::sign_eos_transaction(&action.param.unwrap().value))
        // }
        "eos_message_sign" => {
            landingpad(|| eos_signer::sign_eos_message(&action.param.unwrap().value))
        }

        // // cosmos
        // "cosmos_tx_sign" => {
        //     landingpad(|| cosmos_signer::sign_cosmos_transaction(&action.param.unwrap().value))
        // }
        // // filecoin
        // "filecoin_tx_sign" => {
        //     landingpad(|| filecoin_signer::sign_filecoin_transaction(&action.param.unwrap().value))
        // }
        _ => landingpad(|| Err(format_err!("unsupported_method"))),
    };

    let ret_str = hex::encode(reply);
    CString::new(ret_str).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn imkey_clear_err() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
    LAST_BACKTRACE.with(|e| {
        *e.borrow_mut() = None;
    });
}

#[no_mangle]
pub unsafe extern "C" fn imkey_get_last_err_message() -> *const c_char {
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

#[cfg(test)]
mod tests {
    use super::*;
    use error_handling::Result;
    use std::ffi::{CStr, CString};
    use std::fs::remove_file;
    use std::os::raw::c_char;
    use std::panic;
    use std::path::Path;

    use prost::Message;

    use crate::api::CommonResponse;
    use device::deviceapi::{AppDownloadReq, BindAcquireReq};
    use std::fs;

    fn _to_c_char(str: &str) -> *const c_char {
        CString::new(str).unwrap().into_raw()
    }

    fn _to_str(json_str: *const c_char) -> &'static str {
        let json_c_str = unsafe { CStr::from_ptr(json_str) };
        json_c_str.to_str().unwrap()
    }

    fn teardown() {
        let p = Path::new("/tmp/imtoken/wallets");
        let walk_dir = std::fs::read_dir(p).expect("read dir");
        for entry in walk_dir {
            let entry = entry.expect("DirEntry");
            let fp = entry.path();
            if !fp
                .file_name()
                .expect("file_name")
                .to_str()
                .expect("file_name str")
                .ends_with(".json")
            {
                continue;
            }

            remove_file(fp.as_path()).expect("should remove file");
        }
    }

    // fn run_test<T>(test: T) -> ()
    //     where
    //         T: FnOnce() -> () + panic::UnwindSafe,
    // {
    //     setup();
    //     let result = panic::catch_unwind(|| test());
    //     teardown();
    //     assert!(result.is_ok())
    // }

    #[test]
    fn call_api() {
        // let param = TcxAction {
        //     method: method.to_string(),
        //     param: Some(::prost_types::Any {
        //         type_url: "imtoken".to_string(),
        //         value: encode_message(msg).unwrap(),
        //     }),
        // };

        // let err_bytes = hex::decode("1233e69caae883bde5ae8ce68890e6938de4bd9ce38082efbc88746f6b656e2e4170694572726f72e99499e8afaf31e38082efbc89").unwrap();
        // let err: ErrorResponse = ErrorResponse::decode(err_bytes.as_slice()).unwrap();
        // assert_eq!("err", err.error);

        let param = hex::decode("0a0c62696e645f61637175697265121c0a186465766963656170692e42696e64416371756972655265711200").unwrap();
        let action: ImkeyAction = ImkeyAction::decode(param.as_slice()).unwrap();
        let param: BindAcquireReq =
            BindAcquireReq::decode(action.param.unwrap().value.as_slice()).unwrap();
        assert_eq!("action", param.bind_code);
        // let param: AppDownloadReq = AppDownloadReq::decode(err.param.unwrap().value.as_slice()).unwrap();
        // assert_eq!("action", param.app_name);
        // let param: AppDownloadReq = AppDownloadReq {
        //     app_name: "FILECOIN".to_string()
        // };
        // let action: ImkeyAction = ImkeyAction {
        //     method: "app_download".to_string(),
        //     param: Some(::prost_types::Any {
        //         type_url: "deviceapi.AppDownloadReq".to_string(),
        //         value: encode_message(param).unwrap(),
        //     })
        // };
        // assert_eq!("", hex::encode(encode_message(action).unwrap()));
        let _ = unsafe { imkey_clear_err() };
        // let param_bytes = encode_message(param).unwrap();
        // let param_bytes = hex::decode("0a0c636865636b5f757064617465").unwrap();
        // let param_hex = hex::encode(param_bytes);
        let ret_hex = unsafe {
            _to_str(call_imkey_api(_to_c_char(&"0a0b6765745f61646472657373123c0a106170692e41646472657373506172616d12280a0846494c45434f494e12116d2f3434272f343631272f30272f302f301a074d41494e4e45542000")))
        };
        let err = unsafe { _to_str(imkey_get_last_err_message()) };
        if !err.is_empty() {
            let err_bytes = hex::decode(err).unwrap();
            let err_ret: ErrorResponse = ErrorResponse::decode(err_bytes.as_slice()).unwrap();
            assert_eq!("err", err_ret.error)
        } else {
            let ret_bytes = hex::decode(ret_hex).unwrap();

            let ret: CommonResponse = CommonResponse::decode(ret_bytes.as_slice()).unwrap();
            assert_eq!("ret", ret.result)
        }
    }
}
