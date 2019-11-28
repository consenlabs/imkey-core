use crate::constants;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/**
http post request
*/
pub fn post<T: Serialize>(action: &str, req_data: &T) -> Response {
    let url: String = constants::URL.to_string() + action;
    // let mut url = String::from("http://localhost:8080/imkey/");
    //    url.push_str(action);
    let client = reqwest::Client::new();
    let response: Response = client.post(&*url).json(&req_data).send().unwrap();
    response
}
