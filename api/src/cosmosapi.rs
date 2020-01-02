// FUNCTION: sign_tx(SignParam{input: CosmosTxInput}): CosmosTxOutput

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(string, tag = "2")]
    pub denom: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StdFee {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::std::vec::Vec<Coin>,
    #[prost(uint64, tag = "2")]
    pub gas: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxInput {
    #[prost(uint64, tag = "1")]
    pub account_number: u64,
    #[prost(string, tag = "2")]
    pub chain_id: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub fee: ::std::option::Option<StdFee>,
    #[prost(string, tag = "4")]
    pub memo: std::string::String,
    ///@@XM repeated is not allowed for map type
    #[prost(map = "string, message", tag = "5")]
    pub msgs: ::std::collections::HashMap<std::string::String, ::prost_types::Any>,
    #[prost(uint64, tag = "6")]
    pub sequence: u64,
    #[prost(string, tag = "7")]
    pub path: std::string::String,
    #[prost(string, tag = "8")]
    pub payment_dis: std::string::String,
    #[prost(string, tag = "9")]
    pub to_dis: std::string::String,
    #[prost(string, tag = "10")]
    pub from_dis: std::string::String,
    #[prost(string, tag = "11")]
    pub fee_dis: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxOutput {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
    #[prost(string, tag = "2")]
    pub tx_hash: std::string::String,
}
