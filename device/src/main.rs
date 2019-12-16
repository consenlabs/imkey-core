extern crate reqwest;
use std::collections::HashMap;
//use reqwest::{Client, Response};
//use reqwest::Result;
use serde::{Deserialize, Serialize};

use app_delete::app_delete_request;
use app_download::app_download_request;
use app_update::app_update_request;
use device::se_query::se_query_request;
use ecdh;
use ecdh::private_key::PrivateKey as ecdhPrivateKey;
use ecdh::public_key::PublicKey as ecdhPublicKey;
use se_activate::se_activate_request;
use se_secure_check::se_secure_check_request;

pub mod app_delete;
pub mod app_download;
pub mod app_update;
pub mod auth_code_storage;
pub mod device_cert_check;
pub mod hid_api;
pub mod manager;
pub mod se_activate;
pub mod se_query;
pub mod se_secure_check;

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

    //    let path = "/Users/caixiaoguang/workspace/myproject/imkey-core/".to_string();
    //    let bind_code = "WBMDJM9S".to_string();
    //    let mut temp_devvice_manager = device_binding::DeviceManage::new();
    //    temp_devvice_manager.bind_check(&path);
    ////    let bind_code = device_binding::display_bind_code();
    ////    let test_data = String::from_utf8(auth_code.into_bytes());
    ////    println!("{}", test_data.unwrap());
    //    temp_devvice_manager.bind_acquire(&bind_code);

    //    let hid_device = hid_api::connect();
    //    let apdu = "00a4040005695F696D6B".to_string();
    //    let response = hid_api::send(&hid_device, &apdu);
    //    println!("{}", response);
    //    let response = hid_api::send(&hid_device, &"80CA004400".to_string());
    //    println!("{}", response);

    use secp256k1::ecdh::SharedSecret;
    use secp256k1::{PublicKey, SecretKey};

    //let pub_key = Vec::from_hex("0403089D8A83A87F24D906303A49D39669D17B0F7AB76EB098A65AFEF31154E75DEE5B87B69CBF78F11E831A4961C8A8F031C2869EA0716C798F76F5E91338DC35");
    //let pri_key = Vec::from_hex("D83332655D254FB8BF9BD43570C5A3E188113363749E9F9A27AA4CC0600D3089");
    let pri_key = hex::decode("D83332655D254FB8BF9BD43570C5A3E188113363749E9F9A27AA4CC0600D3089");
    let pub_key = hex::decode("0403089D8A83A87F24D906303A49D39669D17B0F7AB76EB098A65AFEF31154E75DEE5B87B69CBF78F11E831A4961C8A8F031C2869EA0716C798F76F5E91338DC35");
    //let pub_key = hex::decode("0303089D8A83A87F24D906303A49D39669D17B0F7AB76EB098A65AFEF31154E75D");
    let se_pub_key_obj = PublicKey::from_slice(pub_key.clone().unwrap().as_slice()).unwrap();

    let locl_pri_key_obj = SecretKey::from_slice(pri_key.clone().unwrap().as_slice()).unwrap();
    let sec = SharedSecret::new(&se_pub_key_obj, &locl_pri_key_obj);
    println!("{:?}", hex::encode_upper(&sec[..]));
    let ptr: *const u8 = sec.as_ptr() as *const u8;
    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {}!", val_back);
        }
    }

    /*
    let public_key = vec![
        2, 1, 0, 163, 215, 7, 212, 111, 65, 12, 71, 241, 53, 52, 251, 41, 237, 3, 29, 101, 63, 116,
        130, 150, 64, 159, 132, 150, 85, 202, 191, 31, 227, 17, 30, 34, 46, 102, 166, 187, 133, 4,
        84, 239, 190, 162, 174, 161, 40, 3, 203, 213, 79, 238, 16, 123, 90, 254, 108, 134, 181,
        104, 112, 100, 116, 20, 238,
    ];
    let private_key = vec![
        1, 220, 254, 121, 176, 90, 169, 167, 226, 22, 16, 143, 36, 56, 183, 61, 167, 195, 174, 191,
        140, 134, 86, 16, 123, 213, 40, 103, 174, 46, 250, 54, 119, 172, 247, 135, 144, 60, 99, 14,
        242, 129, 212, 64, 121, 172, 200, 4, 121, 60, 129, 126, 58, 16, 23, 225, 56, 245, 56, 32,
        109, 226, 94, 27, 162, 83,
    ];
    */
    let test_pubkey = ecdhPublicKey::from_vec(&pub_key.unwrap()).unwrap();
    let test_prvkey = ecdhPrivateKey::from_vec(&pri_key.unwrap()).unwrap();

    //let test_pubkey = ecdhPublicKey::from_vec(&public_key).unwrap();
    //let test_prvkey = ecdhPrivateKey::from_vec(&private_key).unwrap();

    let alice = ecdhPrivateKey::generate().unwrap();
    let bob = ecdhPrivateKey::generate().unwrap();
    let eve = ecdhPrivateKey::generate().unwrap();

    //let alice_symm_key = ecdh::ECDH::compute_key(&alice, &bob.get_public_key());
    let alice_symm_key = ecdh::ECDH::compute_key(&test_prvkey, &test_pubkey);
    let bob_symm_key = ecdh::ECDH::compute_key(&bob, &alice.get_public_key());
    let eve_symm_key = ecdh::ECDH::compute_key(&eve, &alice.get_public_key());

    println!("alice priv: {:?}", alice.to_vec());
    println!("alice pub: {:?}", alice.get_public_key().to_vec());
    println!("bob priv: {:?}", bob.to_vec());
    println!("bob pub: {:?}", bob.get_public_key().to_vec());

    println!("alice_symm_key: {:?}", alice_symm_key.unwrap().to_vec());
    println!("bob_symm_key: {:?}", bob_symm_key.unwrap().to_vec());
    println!("eve_symm_key: {:?}", eve_symm_key.unwrap().to_vec());
}
