///for app download, update, delete and so on
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAction {
    #[prost(string, tag="1")]
    pub app_name: std::string::String,
}
///for device cert related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceCert {
    #[prost(string, tag="1")]
    pub se_id: std::string::String,
    #[prost(string, tag="2")]
    pub sn: std::string::String,
    #[prost(string, tag="3")]
    pub device_cert: std::string::String,
}
///auth code related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthCode {
    #[prost(string, tag="1")]
    pub se_id: std::string::String,
    #[prost(string, tag="2")]
    pub auth_code: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthCodeResponse {
    #[prost(string, tag="1")]
    pub se_id: std::string::String,
    #[prost(string, tag="2")]
    pub next_stepkey: std::string::String,
    #[prost(string, repeated, tag="3")]
    pub apdu_list: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthCodeServiceResponse {
    #[prost(string, tag="1")]
    pub return_code: std::string::String,
    #[prost(string, tag="2")]
    pub return_msg: std::string::String,
    #[prost(message, optional, tag="3")]
    pub return_data: ::std::option::Option<AuthCodeResponse>,
}
///binding code related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindCode {
    #[prost(string, tag="1")]
    pub bind_code: std::string::String,
}
///name related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceName {
    #[prost(string, tag="1")]
    pub ble_name: std::string::String,
}
