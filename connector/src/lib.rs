use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod libAndroid;
// use j4rs::{ClasspathEntry, Jvm, JvmBuilder};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jstring};

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

#[no_mangle]
pub extern "system" fn Java_com_mk_imkeydemo_MainActivity_factAndCallMeBack(
    env: JNIEnv,
    _class: JClass,
    n: jint,
    callback: JObject,
) {
    let i = n as i32;
    let res: jint = (2..i + 1).product();

    env.call_method(callback, "factCallback", "(I)V", &[res.into()])
        .unwrap();
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_mk_imkeylibrary_utils_LogUtil_installApplet(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jstring {
    //do something ...

    let s = String::from("00A4040000");
    let jj:JString = env.new_string(s).unwrap();
    let jjvalue = JValue::Object(jj.into());
    let result:JValue = env.call_method(callback, "sendApdu", "(Ljava/lang/String;)Ljava/lang/String;", &[jjvalue])
        .unwrap();
    
    let jv = result.to_jni();
    jv.l

    //do something ...
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
