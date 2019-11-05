#![cfg(target_os="android")]
#![allow(non_snake_case)]


// use std::ffi::{CString, CStr};
// // use jni::JNIEnv;
// // use jni::objects::{JObject, JString};
// // use jni::sys::{jstring};

// use jni::{
//     descriptors::Desc,
//     objects::{JClass, JMethodID, JObject, JStaticMethodID, JValue},
//     signature::{JavaType, Primitive},
//     sys::jint,
//     InitArgsBuilder, JNIEnv, JNIVersion, JavaVM,
// };

// // This is the interface to the JVM that we'll call the majority of our
// // methods on.
// use jni::JNIEnv;

// // These objects are what you should use as arguments to your native
// // function. They carry extra lifetime information to prevent them escaping
// // this context and getting used after being GC'd.
// use jni::objects::{JClass, JMethodID, JObject, JString, JValue};

// // This is just a pointer. We'll be returning it from our function. We
// // can't return one of the objects with lifetime information because the
// // lifetime checker won't let us.
// use jni::signature::JavaType;
// use jni::sys::jstring;

// extern crate jni;

// // use jni::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder, MavenArtifact};

// // use lazy_static::LazyStatic;

// // lazy_static! {
// //     static ref VM: JavaVM = {
// //         let args = InitArgsBuilder::new()
// //             .version(JNIVersion::V8)
// //             .build()
// //             .unwrap();
// //         JavaVM::new(args).unwrap()
// //     };
// // }

// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern fn Java_com_mk_imkeydemo_MainActivity_hello(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
//     let recipient = CString::from(
//         CStr::from_ptr(
//             env.get_string(j_recipient).unwrap().as_ptr()
//         )
//     );

//     let output = env.new_string("Hello ".to_owned() + recipient.to_str().unwrap()).unwrap();
//     output.into_inner()
// }

// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "system" fn Java_com_mk_imkeydemo_MainActivity_getXPub(env: JNIEnv, object: JObject) -> jstring {
//     // https://stackoverflow.com/questions/38079081/l-z-and-v-in-java-method-signature/38079146
//     let send_apdu_mid = get_method_id(&env, "com/mk/imkeydemo/MainActivity", "sendApdu", "(Ljava/lang/String;)Ljava/lang/String;").unwrap();
    
//       let rsp = env.call_method_unchecked(
//             object,
//             send_apdu_mid,
//             JavaType::Object("java/lang/String".into()),
//             &[JValue::Object(JObject::from(env.new_string("miao miao miao").unwrap()))]
//         ).unwrap();
//         // let ss = env.get_string(JString::from(rsp.l().unwrap())).expect("Couldn't get java string!").into();
    
//     // // Then we have to create a new Java string to return. Again, more info
//     // in the `strings` module.
//     let output = env.new_string(format!("{}", "hihi"))
//         .expect("Couldn't create java string!");

//     // Finally, extract the raw pointer to return.
//     output.into_inner()
// }

// // static CLASS_MATH: &str = "java/lang/Math";
// // static CLASS_OBJECT: &str = "java/lang/Object";
// // static METHOD_MATH_ABS: &str = "abs";
// // static METHOD_OBJECT_HASH_CODE: &str = "hashCode";
// // static METHOD_CTOR: &str = "<init>";
// // static SIG_OBJECT_CTOR: &str = "()V";
// // static SIG_MATH_ABS: &str = "(I)I";
// // static SIG_OBJECT_HASH_CODE: &str = "()I";

#[no_mangle]
pub extern "C" fn treble(value: i32) -> i32 {
    value * 4
}

// #[no_mangle]
// pub extern "C" fn double_input(input: i32) -> i32 {
//     input * 4
// }

// /// Produces `JMethodID` for a particular class dealing with its lifetime.
// fn get_method_id(env: &JNIEnv, class: &str, name: &str, sig: &str) -> Option<JMethodID<'static>> {
//     env.get_method_id(class, name, sig)
//         // we need this line to erase lifetime in order to save underlying raw pointer in static
//         .map(|mid| mid.into_inner().into())
//         .ok()
// }

// // #[no_mangle]
// // fn jni_abs_safe(env: &JNIEnv, x: jint) -> jint {
// //     let x = JValue::from(x);
// //     let v = env
// //         .call_static_method(CLASS_MATH, METHOD_MATH_ABS, SIG_MATH_ABS, &[x])
// //         .unwrap();
// //     v.i().unwrap()
// // }
