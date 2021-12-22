#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxInput {
    #[prost(string, tag = "1")]
    pub tx_hash: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxOutput {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
