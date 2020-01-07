use mq::message;

#[cfg(target_os = "ios")]
#[derive(Debug)]
pub struct CosmosTransaction {
    pub path: String,
    pub sign_datas: Vec<EosSignData>,
}