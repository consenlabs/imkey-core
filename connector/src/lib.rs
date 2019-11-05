use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod libAndroid;
// use j4rs::{ClasspathEntry, Jvm, JvmBuilder};

#[no_mangle]
pub extern "C" fn rust_hello(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
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

// #[no_mangle]
// pub extern fn treble(value: i32) -> i32 {
//     value * 10
// }

// #[no_mangle]
// pub extern fn treble(value: i32) -> i32 {
//     let entry = ClasspathEntry::new("/home/myuser/dev/myjar-1.0.0.jar");
//     let jvm: Jvm = JvmBuilder::new()
//     .classpath_entry(entry)
//     .build()?;
// }
