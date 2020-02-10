#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EosSignData {
    #[prost(string, tag="1")]
    pub tx_data: std::string::String,
    #[prost(string, repeated, tag="2")]
    pub pub_keys: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag="3")]
    pub chain_id: std::string::String,
    #[prost(string, tag="4")]
    pub to: std::string::String,
    #[prost(string, tag="5")]
    pub from: std::string::String,
    #[prost(string, tag="6")]
    pub payment: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EosTxInput {
    #[prost(string, tag="1")]
    pub path: std::string::String,
    #[prost(message, repeated, tag="2")]
    pub sign_datas: ::std::vec::Vec<EosSignData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EosTxOutput {
    #[prost(string, tag="1")]
    pub hash: std::string::String,
    #[prost(string, repeated, tag="2")]
    pub signs: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EosPubkey {
    #[prost(string, tag="1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EosPubkeyResponse {
    #[prost(string, tag="1")]
    pub pubkey: std::string::String,
}
