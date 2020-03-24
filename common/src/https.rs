use crate::constants;
use serde::Serialize;
use futures::{future, Future, Stream};
use hyper::client::Client;
use hyper::header::HeaderValue;
use hyper::{Body, Error, Method, Request};
use tokio_core::reactor::Core;
use crate::Result;

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

pub fn post(action: &str, req_data: Vec<u8>) -> Result<String> {
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

    let mut event_loop = Core::new()?;
//    let handle = event_loop.handle();

    let https = hyper_tls::HttpsConnector::new(4)?;
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
                let s = String::from_utf8(chunks).expect("return_message_conver_error");
                future::ok::<_, Error>(s)
            })
    });

    let res_data = event_loop.run(work)?;
    Ok(res_data)
}
