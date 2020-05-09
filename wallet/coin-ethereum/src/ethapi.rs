#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTxReq {
    #[prost(string, tag="1")]
    pub nonce: std::string::String,
    #[prost(string, tag="2")]
    pub gas_price: std::string::String,
    #[prost(string, tag="3")]
    pub gas_limit: std::string::String,
    #[prost(string, tag="4")]
    pub to: std::string::String,
    #[prost(string, tag="5")]
    pub value: std::string::String,
    #[prost(string, tag="6")]
    pub data: std::string::String,
    #[prost(string, tag="7")]
    pub chain_id: std::string::String,
    #[prost(string, tag="8")]
    pub path: std::string::String,
    #[prost(string, tag="9")]
    pub payment: std::string::String,
    #[prost(string, tag="10")]
    pub receiver: std::string::String,
    #[prost(string, tag="11")]
    pub sender: std::string::String,
    #[prost(string, tag="12")]
    pub fee: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTxRes {
    #[prost(string, tag="1")]
    pub tx_data: std::string::String,
    #[prost(string, tag="2")]
    pub tx_hash: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAddressReq {
    #[prost(string, tag="1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAddressRes {
    #[prost(string, tag="1")]
    pub address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthMessageSignReq {
    #[prost(string, tag="1")]
    pub path: std::string::String,
    #[prost(string, tag="2")]
    pub message: std::string::String,
    #[prost(string, tag="3")]
    pub sender: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthMessageSignRes {
    #[prost(string, tag="1")]
    pub signature: std::string::String,
}
