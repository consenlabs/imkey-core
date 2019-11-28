extern crate bitcoin;
pub use bitcoin::{Address};
use std::str::FromStr;
use bitcoin::Error;

pub struct bitcoin_trans_data{
    pub to : String, //收款地址
//        change_idx : i32,
    pub  amount : i64, //付款金额
    pub fee : i64, //手续费
    pub outputs : Vec<Utxo>,
//        payment : String,
//        toDis : String,
    pub from : String, //付款地址
//        feeDis : String, //
}

impl bitcoin_trans_data{
    pub fn bitcoin_sign(& self){
        //产生Address对象，通过地址判断网络类型以及交易类型
        let address_from = Address::from_str(self.from.as_str()).unwrap();
        //判断发送地址是否标准
        let result = address_from.is_standard();
        if(!result){
            panic!("from address error");
        }
        //判断接受地址是否标准
        let address_to = Address::from_str(self.to.as_str()).unwrap();
        if(!address_to.is_standard()){
            panic!("to address error");
        }
        //获取UTXO总额，并校验支付额度是否满足
        let mut total_amount : i64 = 0;
        for utxo in &self.outputs{
            println!("{}", utxo.amount);
            total_amount += utxo.amount;
        }
        println!("tatal amount is : {}", total_amount);
        if total_amount < (self.amount + self.fee){
            panic!("余额不足");
        }


    }
}

pub struct Utxo{
    pub txHash : String,
    pub vout : i32,
    pub amount : i64,
    pub address : String,
    pub script_pubkey : String,
    pub derived_path : String,
//    pub sequence : i64,
}

pub struct SignResult {
    pub signature: String,
    pub tx_hash: String,
    pub wtx_id: String,
}

fn get_xpub(path: String, verify_flag: bool){
    
}