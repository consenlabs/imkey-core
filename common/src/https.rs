use crate::constants;
//use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::ImkeyError;
use futures::{future, Future, Stream};
use http::StatusCode;
use hyper::client::Client;
use hyper::header::HeaderValue;
use hyper::{Body, Error, Method, Request};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

/**
http post request
*/
pub fn post2<T: Serialize>(action: &str, req_data: &T) -> reqwest::Response {
    let url: String = constants::URL.to_string() + action;
    // let mut url = String::from("http://localhost:8080/imkey/");
    //    url.push_str(action);
    let client = reqwest::Client::new();
    let response: reqwest::Response = client.post(&*url).json(&req_data).send().unwrap();
    response
}

pub fn post(action: &str, req_data: Vec<u8>) -> Result<String, ImkeyError> {
    let uri: hyper::Uri = format!("{}{}", constants::URL.to_string(), action)
        .to_string()
        .parse()
        .unwrap();
    let mut req = Request::new(Body::from(req_data));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();

    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let work = client.request(req).and_then(|res| {
        println!("Response: {}", res.status());
        //        if(!res.status().is_success()){
        //            Err(ImkeyError::NETWORK_ERROR)
        //        }
        res.into_body()
            .fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            })
            .and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
    });

    let res_data = event_loop.run(work).unwrap();
    println!(
        "We've made it outside the request! \
         We got back the following from our \
         request:\n"
    );
    println!("{}", res_data);
    Ok(res_data)
}
