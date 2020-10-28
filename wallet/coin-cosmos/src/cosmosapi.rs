#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub amount: std::string::String,
    #[prost(string, tag = "2")]
    pub denom: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StdFee {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::std::vec::Vec<Coin>,
    #[prost(string, tag = "2")]
    pub gas: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxInput {
    #[prost(string, tag = "1")]
    pub account_number: std::string::String,
    #[prost(string, tag = "2")]
    pub chain_id: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub fee: ::std::option::Option<StdFee>,
    #[prost(string, tag = "4")]
    pub memo: std::string::String,
    #[prost(string, tag = "5")]
    pub msgs: std::string::String,
    #[prost(string, tag = "6")]
    pub sequence: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxOutput {
    #[prost(string, tag = "1")]
    pub tx_data: std::string::String,
    #[prost(string, tag = "2")]
    pub tx_hash: std::string::String,
}
