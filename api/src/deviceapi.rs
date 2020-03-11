#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyResponse {
}
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
///se related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeAction {
    #[prost(string, tag="1")]
    pub se_id: std::string::String,
    #[prost(string, tag="2")]
    pub sn: std::string::String,
    #[prost(string, tag="3")]
    pub device_cert: std::string::String,
    #[prost(string, tag="4")]
    pub sdk_version: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeQueryResponse {
    #[prost(string, tag="1")]
    pub se_id: std::string::String,
    #[prost(string, tag="2")]
    pub next_stepkey: std::string::String,
    #[prost(string, tag="3")]
    pub sn: std::string::String,
    #[prost(string, tag="4")]
    pub sdk_mode: std::string::String,
    #[prost(message, repeated, tag="5")]
    pub available_app_bean_list: ::std::vec::Vec<AvailableAppBean>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeQueryServiceResponse {
    #[prost(string, tag="1")]
    pub return_code: std::string::String,
    #[prost(string, tag="2")]
    pub return_msg: std::string::String,
    #[prost(message, optional, tag="3")]
    pub return_data: ::std::option::Option<SeQueryResponse>,
}
///binding related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindCheck {
    #[prost(string, tag="1")]
    pub file_path: std::string::String,
}
///binding related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindCheckResponse {
    #[prost(string, tag="1")]
    pub bind_status: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindAcquire {
    #[prost(string, tag="1")]
    pub bind_code: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindAcquireResponse {
    #[prost(string, tag="1")]
    pub bind_result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindDisplay {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindDisplayResponse {
    #[prost(string, tag="1")]
    pub bind_display_result: std::string::String,
}
///name related
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceName {
    #[prost(string, tag="1")]
    pub ble_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnResponse {
    #[prost(string, tag="1")]
    pub sn: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApduResponse {
    #[prost(string, tag="1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    #[prost(string, tag="1")]
    pub tt: std::string::String,
}
/// check_update api
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUpdateResponse {
    #[prost(string, tag="1")]
    pub se_id: std::string::String,
    #[prost(string, tag="2")]
    pub sn: std::string::String,
    #[prost(string, tag="3")]
    pub status: std::string::String,
    #[prost(string, tag="4")]
    pub sdk_mode: std::string::String,
    #[prost(message, repeated, tag="5")]
    pub available_app_list: ::std::vec::Vec<AvailableAppBean>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableAppBean {
    #[prost(string, tag="1")]
    pub app_name: std::string::String,
    #[prost(string, tag="2")]
    pub app_logo: std::string::String,
    #[prost(string, tag="3")]
    pub installed_version: std::string::String,
    #[prost(string, tag="4")]
    pub last_updated: std::string::String,
    #[prost(string, tag="5")]
    pub latest_version: std::string::String,
    #[prost(string, tag="6")]
    pub install_mode: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdkInfoResponse {
    #[prost(string, tag="1")]
    pub sdk_version: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleAction {
    #[prost(string, tag="1")]
    pub ble_name: std::string::String,
}
