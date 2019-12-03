#[macro_use()]
use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Command execution error
    RpcError(String),
    AddressError,
    PrvKeyError,
    PubKeyError,
    MessageError,
    DataError,
    SignError,
}

macro_rules! from_err {
    ($x:ty) => {
        impl From<$x> for Error {
            fn from(err: $x) -> Self {
                Error::RpcError(format!(
                    "something wrong with rpc call: {}",
                    err.to_string()
                ))
            }
        }
    };
}

from_err!(reqwest::Error);
//from_err!(std::string::ParseError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RpcError(ref str) => write!(f, "rpc call error: {}", str),
            Error::AddressError => write!(f, "address is wrong"),
            Error::PrvKeyError => write!(f, "private key parse error"),
            Error::PubKeyError => write!(f, "public key parse error"),
            Error::MessageError => write!(f, "sigh hash got error"),
            Error::DataError => write!(f, "data field wrong format"),
            Error::SignError => write!(f, "signature error"),
        }
    }
}
