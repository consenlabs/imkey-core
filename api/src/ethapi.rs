/// FUNCTION: sign_tx(SignParam{input: EthTxInput}): EthTxOutput
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTxInput {
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
    #[prost(bytes, tag="6")]
    pub data: std::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub chain_id: u64,
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
    #[prost(string, tag="13")]
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTxOutput {
    #[prost(string, tag="1")]
    pub signature: std::string::String,
    #[prost(string, tag="2")]
    pub tx_hash: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAddressResponse {
    #[prost(string, tag="1")]
    pub address: std::string::String,
}
