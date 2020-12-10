#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTxReq {
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    #[prost(string, tag = "2")]
    pub raw_data: std::string::String,
    #[prost(string, tag = "3")]
    pub address: std::string::String,
    #[prost(string, tag = "4")]
    pub payment: std::string::String,
    #[prost(string, tag = "5")]
    pub to: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTxRes {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronAddressReq {
    #[prost(string, tag = "1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronAddressRes {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronMessageSignReq {
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
    #[prost(string, tag = "3")]
    pub address: std::string::String,
    #[prost(bool, tag = "4")]
    pub is_hex: bool,
    #[prost(bool, tag = "5")]
    pub is_tron_header: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronMessageSignRes {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
