#[cfg(any(target_os = "macos", target_os = "windows"))]
pub mod hid_api;
pub mod message;
pub mod error;
#[macro_use]
extern crate failure;
use core::result;
pub type Result<T> = result::Result<T, failure::Error>;

//#[macro_use]
////extern crate log;
#[macro_use]
extern crate lazy_static;
