#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosTxInput {
    #[prost(string, tag = "1")]
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosTxOutput {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
    #[prost(string, tag = "2")]
    pub edsig: std::string::String,
    #[prost(string, tag = "3")]
    pub sbytes: std::string::String,
}
