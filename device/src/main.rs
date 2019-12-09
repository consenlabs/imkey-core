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
    se_query_request::build_request_data(seid, sn, None).se_query();

    //hyper test

    //    let reqdata = se_secure_check_request {
    //        seid: seid,
    //        sn: sn,
    //        deviceCert: device_cert,
    //        stepKey: String::from("01"),
    //        statusWord: None,
    //        commandID: String::from("seSecureCheck"),
    //        cardRetDataList: None,
    //    };
    //    let req_data = serde_json::to_vec_pretty(&reqdata).unwrap();
    //    println!("{:?}", req_data);

    //    let json = r#"{"library":"hyper"}"#;
    //    let uri: hyper::Uri = "https://localhost:8443/imkey/seSecureCheck"
    //        .parse()
    //        .unwrap();
    //    let mut req = Request::new(Body::from(resdata));
    //    //    let mut req = Request::new(Body::empty());
    //    *req.method_mut() = Method::POST;
    //    *req.uri_mut() = uri.clone();
    //    req.headers_mut().insert(
    //        hyper::header::CONTENT_TYPE,
    //        HeaderValue::from_static("application/json"),
    //    );
    //
    //    let mut event_loop = Core::new().unwrap();
    //    let handle = event_loop.handle();
    //
    //    //rt::run(rt::lazy(|| {
    //    // 4 is number of blocking DNS threads
    //    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    //    let client = Client::builder().build::<_, hyper::Body>(https);
    //
    //    let work = client.request(req).and_then(|res| {
    //        println!("Response: {}", res.status());
    //        //println!("Headers: \n{}", res.headers());
    //
    //        res.into_body()
    //            .fold(Vec::new(), |mut v, chunk| {
    //                v.extend(&chunk[..]);
    //                future::ok::<_, Error>(v)
    //            })
    //            .and_then(|chunks| {
    //                let s = String::from_utf8(chunks).unwrap();
    //                future::ok::<_, Error>(s)
    //            })
    //    });
    //
    //    let user = event_loop.run(work).unwrap();
    //    println!("We've made it outside the request! \
    //              We got back the following from our \
    //              request:\n");
    //    println!("{}", user);

    //GET
    //        client
    //            .get("https://localhost:8443/imkey/test".parse().unwrap()).map(|res|{
    //            println!("{}", res.status());
    //        }).map_err(|e|{
    //            println!("request error: {}", e);
    //        })

    //        let future = client.request(req).and_then(|body|{
    //
    //            let s = ::std::str::from_utf8(&body.into()).expect("httpbin sends utf-8 JSON");
    //            println!("{}", s);
    //        });

    //POST
    /*
    client.request(req).and_then(|res| {
        println!("Response: {}", res.status());
        /*
        res
                .into_body()
                // Body is a stream, so as each chunk arrives...
                .for_each(|chunk| {
                    io::stdout()
                        .write_all(&chunk)
                        .map_err(|e| {
                            panic!("example expects stdout is open, error={}", e)
                        })
                })
        */
        res.body()
            .fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                //future::ok::<_, _>(v);
            })
            .and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                //future::ok::<_, _>(s);
            })
    })
    */
    //}));
    //}

    //fn test(req_data : Vec<u8>, action : &str)-> Result<String, ImkeyError>{
    //    let uri: hyper::Uri = "https://localhost:8443/imkey/seSecureCheck"
    //        .parse()
    //        .unwrap();
    //    let mut req = Request::new(Body::from(req_data));
    //    //    let mut req = Request::new(Body::empty());
    //    *req.method_mut() = Method::POST;
    //    *req.uri_mut() = uri.clone();
    //    req.headers_mut().insert(
    //        hyper::header::CONTENT_TYPE,
    //        HeaderValue::from_static("application/json"),
    //    );
    //
    //    let mut event_loop = Core::new().unwrap();
    //    let handle = event_loop.handle();
    //
    //    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    //    let client = Client::builder().build::<_, hyper::Body>(https);
    //
    //    let work = client.request(req).and_then(|res| {
    //        println!("Response: {}", res.status());
    ////        if(!res.status().is_success()){
    ////            Err(ImkeyError::NETWORK_ERROR)
    ////        }
    //        res.into_body()
    //            .fold(Vec::new(), |mut v, chunk| {
    //                v.extend(&chunk[..]);
    //                future::ok::<_, Error>(v)
    //            })
    //            .and_then(|chunks| {
    //                let s = String::from_utf8(chunks).unwrap();
    //                future::ok::<_, Error>(s)
    //            })
    //    });
    //
    //    let res_data = event_loop.run(work).unwrap();
    //    println!("We've made it outside the request! \
    //              We got back the following from our \
    //              request:\n");
    //    println!("{}", res_data);
    //    Ok(res_data)
    //}

    //        se_query_request::build_request_data(seid, sn, None).se_query();

    //    use key_manager::KeyManager;
    //    let mut temp = KeyManager::new();
    //    temp.gen_encrypt_key(&"18090000000000860001010000000204".to_string(), &"imKey01190300020".to_string());
    //    println!("{:?}", temp.encry_key.unwrap());
    //    println!("{:?}", temp.iv.unwrap());
    //    let r = KeyManager::get_key_file_data(&String::from("/Users/caixiaoguang/workspace/GIT/imkey-core/"), &"18090000000000860001010000000204".to_string());
    //    println!("{}", r);
    //
    //    temp.decrypt_keys(r.as_bytes());
    //    println!("\n");
    //    println!("encry_key value is : {:?}", temp.encry_key.unwrap());
    //    println!("check_sum value is : {:?}", temp.check_sum.unwrap());
    //    println!("session_key value is : {:?}", temp.session_key.unwrap());
    ////    println!(se_pub_key value is : "{:?}", temp.se_pub_key.unwrap());
    ////    println!(pub_key value is : "{:?}", temp.pub_key.unwrap());
    //    println!("pri_key value is : {:?}", temp.pri_key.unwrap());
    //    println!("iv value is : {:?}", temp.iv.unwrap());
    //
    //    //本地生成ECC密钥对
    //    temp.gen_local_keys();

    DeviceManage::bind_check();
}
