#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag="1")]
    pub amount: std::string::String,
    #[prost(string, tag="2")]
    pub denom: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StdFee {
    #[prost(message, repeated, tag="1")]
    pub amount: ::std::vec::Vec<Coin>,
    #[prost(string, tag="2")]
    pub gas: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignData {
    #[prost(string, tag="1")]
    pub account_number: std::string::String,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub fee: ::std::option::Option<StdFee>,
    #[prost(message, optional, tag="4")]
    pub memo: ::std::option::Option<::std::string::String>,
    #[prost(message, repeated, tag="5")]
    pub msgs: ::std::vec::Vec<Msg>,
    #[prost(string, tag="6")]
    pub sequence: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(string, tag="1")]
    pub r#type: std::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::std::option::Option<MsgValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValue {
    #[prost(message, repeated, tag="1")]
    pub amount: ::std::vec::Vec<Coin>,
    #[prost(map="string, string", tag="2")]
    pub addresses: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxReq {
    #[prost(message, optional, tag="1")]
    pub sign_data: ::std::option::Option<SignData>,
    #[prost(string, tag="2")]
    pub path: std::string::String,
    #[prost(string, tag="3")]
    pub payment_dis: std::string::String,
    #[prost(string, tag="4")]
    pub to_dis: std::string::String,
    #[prost(string, tag="5")]
    pub from_dis: std::string::String,
    #[prost(string, tag="6")]
    pub fee_dis: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxRes {
    #[prost(string, tag="1")]
    pub tx_data: std::string::String,
    #[prost(string, tag="2")]
    pub tx_hash: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosAddressReq {
    #[prost(string, tag="1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosAddressRes {
    #[prost(string, tag="1")]
    pub address: std::string::String,
}
