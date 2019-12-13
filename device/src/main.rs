extern crate reqwest;
use std::collections::HashMap;
//use reqwest::{Client, Response};
//use reqwest::Result;
use serde::{Deserialize, Serialize};

use app_delete::app_delete_request;
use app_download::app_download_request;
use app_update::app_update_request;
use device::se_query::se_query_request;
use se_activate::se_activate_request;
use se_secure_check::se_secure_check_request;

pub mod app_delete;
pub mod app_download;
pub mod app_update;
pub mod se_activate;
pub mod se_secure_check;
pub mod auth_code_storage;
pub mod device_cert_check;
pub mod manager;
pub mod hid_api;

extern crate futures;
extern crate hyper_tls;
extern crate tokio_core;

use futures::future;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

use hyper::client::Client;
use hyper::header::HeaderValue;
use hyper::Error;
use hyper::{Body, Method, Request};
////use hyper::header::{Authorization, Accept, UserAgent, qitem};
//use hyper::mime::Mime;
use hyper_tls::HttpsConnector;

//use std::error::Error;
use std::io;
use std::io::Write;
//use http::StatusCode;
//use device::error::ImkeyError;
use common::https;
use device::device_binding::DeviceManage;
use device::key_manager::KeyManager;

pub mod device_binding;
pub mod key_manager;
use base64::{decode, encode};
use hex::FromHex;
use rand::rngs::OsRng;
use rsa::{BigUint, PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};

fn main() {
    //SE安全检查
    let seid: String = "18080000000000860001010000000015".to_string();
    let sn: String = "imKey01190200001".to_string();
    let device_cert : String = "BF2181CC7F2181C8931019030000000000860001010000003963420200015F200401020304950200805F2504201810145F2404FFFFFFFF5300BF20007F4947B0410467CCF4014F12CD42C97C5526CA9885C7ABFD7CA2D3CEBD04F5CA647C03F461B2E4D52B331166E67A55531ADBE69FE59F0ECE9ECAD58285BD551152A103847C3EF002DFFE5F3747304502203D64BF429F953C0912CFF02A5756B82B268293CF5D949FEC754415A6396CC5FB02210085E06EBC9981363E265CDA6E5B9670B197D030C6BEEF5DAA8D63EF27714473279000".to_string();

    //    match se_secure_check_request::build_request_data(seid, sn, device_cert).se_secure_check(){
    //        Ok(()) => println!("success!"),
    //        Err(e) => println!("{}", e),
    //    }

    //应用下载
    let instance_aid: String = "695F657468".to_string();
    //    app_download_request::build_request_data(seid, instance_aid, device_cert, None).app_download();

    //应用更新
    //     app_update_request::build_request_data(seid, instance_aid, device_cert, None).app_update();

    //应用删除
    //        app_delete_request::build_request_data(seid, instance_aid, device_cert).app_delete();

    //SE激活
    //        se_activate_request::build_request_data(seid, sn, device_cert).se_activate();

    //SE应用信息查询
    //    se_query_request::build_request_data(seid, sn, None).se_query();

    //    let device_manager = DeviceManage::new();
    //    device_manager.bind_check();
    //    device_manager.bind_acquire(&"xxxxxxxxx".to_string());

    //    let pub_key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxmJ6bwSFsz3cHKfgYsZOiEETO5JGpB9A0HZ7rkTqsu9FPQCP+we42f380hiCSH7MTakzyX5JQkKto84CxaBRiapJQQ53GmboEA5Dyxr2zGELWe5OuyNv84xirXsdEd+9TgVNGeM0k5GjH16JynISNGmpNfSOuJjLq3LLOUw/7J5BY16ulUEHoXrHuMYyHY8XVa05FanSOY2yaKP2Qs7py+n4Ls1a1k6+3d5mYB3CuJHi/t33La9if6j6FvfGQNtmG+Fdy0J02VdtmNvrIMJTCQIDAQAB";
    //    let data = decode(pub_key).unwrap();
    //    println!("{:?}", hex::encode(data));
    //    //30820122300d06092a864886f70d01010105000382010f003082010a0282010100c6627a6f0485b33ddc1ca7e062c64e8841133b9246a41f40d0767bae44eab2ef453d008ffb07b8d9fdfcd21882487ecc4da933c97e494242ada3ce02c5a05189aa49410e771a66e8100e43cb1af6cc610b59ee4ebb236ff38c62ad7b1d11dfbd4e054d19e3349391a31f5e89ca72123469a935f48eb898cbab72cb394c3fec9e41635eae954107a17ac7b8c6321d8f1755ad3915a9d2398db268a3f642cee9cbe9f82ecd5ad64ebeddde66601dc2b891e2feddf72daf627fa8fa16f7c640db661be15dcb4274d9576d98dbeb20c253090203010001

    let path = "/Users/caixiaoguang/workspace/GIT/imkey-core/".to_string();
    let auth_code = "HLZ58BPTL".to_string();
    let mut temp_devvice_manager = device_binding::DeviceManage::new();
    temp_devvice_manager.bind_check(&path);
//    let bind_code = device_binding::display_bind_code();
//    let test_data = String::from_utf8(auth_code.into_bytes());
//    println!("{}", test_data.unwrap());
//    temp_devvice_manager.bind_acquire(&bind_code);


//    let hid_device = hid_api::connect();
//    let apdu = "00a4040005695F696D6B".to_string();
//    let response = hid_api::send(&hid_device, &apdu);
//    println!("{}", response);
//    let response = hid_api::send(&hid_device, &"80CA004400".to_string());
//    println!("{}", response);


}
