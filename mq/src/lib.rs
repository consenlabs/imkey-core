#[cfg(any(target_os = "macos", target_os = "windows"))]
pub mod hid_api;
pub mod message;

//#[macro_use]
////extern crate log;
#[macro_use]
extern crate lazy_static;
