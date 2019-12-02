extern crate reqwest;

use std::collections::HashMap;
use reqwest::{Client, Response};
use reqwest::Result;
use serde::{Serialize, Deserialize};

use se_secure_check::se_secure_check_request;
use app_download::app_download_request;
use app_update::app_update_request;
use app_delete::app_delete_request;
use se_activate::se_activate_request;
use device::se_query::se_query_request;

pub mod se_secure_check;
pub mod app_download;
pub mod app_update;
pub mod app_delete;
pub mod se_activate;
pub mod error;

fn main() {

    //SE安全检查
    let seid : String = "18080000000000860001010000000015".to_string();
    let sn : String = "imKey01190200001".to_string();
    let device_cert : String = "BF2181CC7F2181C8931019030000000000860001010000003963420200015F200401020304950200805F2504201810145F2404FFFFFFFF5300BF20007F4947B0410467CCF4014F12CD42C97C5526CA9885C7ABFD7CA2D3CEBD04F5CA647C03F461B2E4D52B331166E67A55531ADBE69FE59F0ECE9ECAD58285BD551152A103847C3EF002DFFE5F3747304502203D64BF429F953C0912CFF02A5756B82B268293CF5D949FEC754415A6396CC5FB02210085E06EBC9981363E265CDA6E5B9670B197D030C6BEEF5DAA8D63EF27714473279000".to_string();

    match se_secure_check_request::build_request_data(seid, sn, device_cert).se_secure_check(){
        Ok(()) => println!("success!"),
        Err(e) => println!("{}", e),
    }

    //应用下载
    let instance_aid : String = "695F657468".to_string();
//    app_download_request::build_request_data(seid, instance_aid, device_cert, None).app_download();

    //应用更新
//     app_update_request::build_request_data(seid, instance_aid, device_cert, None).app_update();

    //应用删除
//    app_delete_request::build_request_data(seid, instance_aid, device_cert).app_delete();

    //SE激活
//    se_activate_request::build_request_data(seid, sn, device_cert).se_activate();

    //SE应用信息查询
//    se_query_request::build_request_data(seid, sn, None).se_query();

}


