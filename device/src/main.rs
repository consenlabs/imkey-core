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

    //let n = Vec::from_hex("C6627A6F0485B33DDC1CA7E062C64E8841133B9246A41F40D0767BAE44EAB2EF453D008FFB07B8D9FDFCD21882487ECC4DA933C97E494242ADA3CE02C5A05189AA49410E771A66E8100E43CB1AF6CC610B59EE4EBB236FF38C62AD7B1D11DFBD4E054D19E3349391A31F5E89CA721292B7380295745D8968CC5C2D223AC6750BB0ACA27773687E9CD76065E47F42F4AE005459BCE5746BD760646A5BD119BA3469A935F48EB898CBAB72CB394C3FEC9E41635EAE954107A17AC7B8C6321D8F1755AD3915A9D2398DB268A3F642CEE9CBE9F82ECD5AD64EBEDDDE66601DC2B891E2FEDDF72DAF627FA8FA16F7C640DB661BE15DCB4274D9576D98DBEB20C25309");
    /*
    let n = Vec::from_hex("e0d93d2d4edd9cc7bd36e9dd5d4724696cda815ebe5299924e74f1f798955a5c4420d07e766e748412034247e6259119ce1eaadff0f8746ac8858857a00e47cc75027d41adc6dd390cb9304f0f57c581ee5307d0df0c350c256786589d913a889c36a8a33bd93e49878bdd03fc3162b1aa3ca5c6ab8a598b0f6e8a13cb32f74c46e52f882e76b90f5176e22adba9d287df813e405930b6ff630c01832b7f6de4d36c8d90f0c724eb4b00c9c44b2ee185308f86da9bbefaba8135a0e3abc380425dc635d41895f48b7f98ab966829ccb997544efa4bfe8a932fb252f34697676207cdc2f35eb0ae27145e1739e7d03aa71afd5d03dcc1576ea2b3604ca724c2cd");
    let e = Vec::from_hex("010001");
    let mut u32_vec_n = Vec::new();
    for val in n.unwrap() {
        u32_vec_n.push(val as u32);
    }
    let mut u32_vec_e = Vec::new();
    for val in e.unwrap() {
        u32_vec_e.push(val as u32);
    }
    */
    let n = hex::decode("C6627A6F0485B33DDC1CA7E062C64E8841133B9246A41F40D0767BAE44EAB2EF453D008FFB07B8D9FDFCD21882487ECC4DA933C97E494242ADA3CE02C5A05189AA49410E771A66E8100E43CB1AF6CC610B59EE4EBB236FF38C62AD7B1D11DFBD4E054D19E3349391A31F5E89CA721292B7380295745D8968CC5C2D223AC6750BB0ACA27773687E9CD76065E47F42F4AE005459BCE5746BD760646A5BD119BA3469A935F48EB898CBAB72CB394C3FEC9E41635EAE954107A17AC7B8C6321D8F1755AD3915A9D2398DB268A3F642CEE9CBE9F82ECD5AD64EBEDDDE66601DC2B891E2FEDDF72DAF627FA8FA16F7C640DB661BE15DCB4274D9576D98DBEB20C25309");
    let e = hex::decode("010001");
    let u32_vec_n = BigUint::from_bytes_be(&n.unwrap());
    let u32_vec_e = BigUint::from_bytes_be(&e.unwrap());

    //    let rsa_pub_key = RSAPublicKey::new(BigUint::new(u32_vec_n), BigUint::new(u32_vec_e));
    //    let mut rng = OsRng::new().unwrap();
    //    let text = "AAAAAAAA".as_bytes();
    //    let result = rsa_pub_key.unwrap().encrypt(&mut rng, PaddingScheme::PKCS1v15, &text);
    //    println!("{:?}", hex::encode(result.unwrap()));

    let mut rng = OsRng::new().expect("no secure randomness available");
    let bits = 2048;
    let key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    //    let n = key.n();
    //    let e = key.e();
    let d = key.d();
    let primes = key.primes();
    println!("n is {:?}", hex::encode(key.n().to_bytes_be()));
    println!("e is {:?}", hex::encode(key.e().to_bytes_be()));
    println!("d is {:?}", key.d().to_bytes_be());
    //    println!("{:?}", key.primes().to);
    //    let rsa_pub_key = RSAPublicKey::new(n.clone(), e.clone());
    //let rsa_pub_key = RSAPublicKey::new(BigUint::new(u32_vec_n), BigUint::new(u32_vec_e));
    let rsa_pub_key = RSAPublicKey::new(u32_vec_n, u32_vec_e);
    //加密
    let a = "AAAAAAAA".as_bytes();
    println!("{:?}", a);
    let enc_data = rsa_pub_key
        .unwrap()
        .encrypt(&mut rng, PaddingScheme::PKCS1v15, a);
    let b = enc_data.unwrap();
    println!("{:?}", b);
    let dec_data = key
        .decrypt(PaddingScheme::PKCS1v15, b.as_ref())
        .expect("failed to encrypt");
    println!("{:?}", dec_data.as_slice());
}
