#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Eth2MsgSignInput {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Eth2MsgSignOutput {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
