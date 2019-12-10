use super::app_delete;
use super::app_download;
use super::app_update;
use super::se_activate;
use super::se_secure_check::se_secure_check_request;
use app_update::app_update_request;
use se_activate::se_activate_request;
use mq::message::send_apdu;

pub fn get_se_id() -> String{
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CB800005DFFF028101".to_string());
    res.chars().take(res.len()-4).collect()
}

pub fn get_sn() -> String{
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CA004400".to_string());
    res.chars().take(res.len()-4).collect()
}

pub fn get_cert() -> String{
    send_apdu("00A4040000".to_string());
    let res = send_apdu("80CABF2106A6048302151800".to_string());
    res.chars().take(res.len()-4).collect()
}

pub fn check_device() {
    let seid: String = get_se_id();
    let sn: String = get_sn();
    let device_cert : String = get_cert();

    match se_secure_check_request::build_request_data(seid, sn, device_cert).se_secure_check() {
        Ok(()) => println!("success!"),
        Err(e) => println!("{}", e),
    }
}

pub fn active_device() {
    let seid: String = "18080000000000860001010000000015".to_string();
    let sn: String = "imKey01190200001".to_string();
    let device_cert : String = "BF2181CC7F2181C8931019030000000000860001010000003963420200015F200401020304950200805F2504201810145F2404FFFFFFFF5300BF20007F4947B0410467CCF4014F12CD42C97C5526CA9885C7ABFD7CA2D3CEBD04F5CA647C03F461B2E4D52B331166E67A55531ADBE69FE59F0ECE9ECAD58285BD551152A103847C3EF002DFFE5F3747304502203D64BF429F953C0912CFF02A5756B82B268293CF5D949FEC754415A6396CC5FB02210085E06EBC9981363E265CDA6E5B9670B197D030C6BEEF5DAA8D63EF27714473279000".to_string();

    se_activate_request::build_request_data(seid, sn, device_cert).se_activate();
}

pub fn check_update() {
    let seid: String = "18080000000000860001010000000015".to_string();
    let sn: String = "imKey01190200001".to_string();
    let device_cert : String = "BF2181CC7F2181C8931019030000000000860001010000003963420200015F200401020304950200805F2504201810145F2404FFFFFFFF5300BF20007F4947B0410467CCF4014F12CD42C97C5526CA9885C7ABFD7CA2D3CEBD04F5CA647C03F461B2E4D52B331166E67A55531ADBE69FE59F0ECE9ECAD58285BD551152A103847C3EF002DFFE5F3747304502203D64BF429F953C0912CFF02A5756B82B268293CF5D949FEC754415A6396CC5FB02210085E06EBC9981363E265CDA6E5B9670B197D030C6BEEF5DAA8D63EF27714473279000".to_string();

    let instance_aid: String = "695F657468".to_string();
    app_update_request::build_request_data(seid, instance_aid, device_cert, None).app_update();
}

pub fn download() {}
