/// Action Wrapper
/// There is a `call_imkey_api` method in tcx which act as a endpoint like RPC. It accepts a `ImkeyAction` param which method field is
/// the real action and param field is the real param of that method.
/// When an error occurred, the `call_imkey_api` will return a `Response` which isSuccess field be false and error field is the reason
/// which cause the error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImkeyAction {
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub param: ::std::option::Option<::prost_types::Any>,
}
/// A common response when error occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub is_success: bool,
    #[prost(string, tag = "2")]
    pub error: std::string::String,
}
