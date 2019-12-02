#[macro_use]
extern crate lazy_static;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

lazy_static! {
    // static ref CALLBACK: extern "C" fn(apdu:*const c_char)->*const c_char;
}

#[no_mangle]
pub extern "C" fn rust_hello(
    to: *const c_char,
    callback: extern "C" fn(apdu: *const c_char) -> *const c_char,
) -> *mut c_char {
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
