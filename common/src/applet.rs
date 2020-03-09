
pub fn get_appname_by_instid(instid: &str) -> Option<&str> {
    match instid {
        "695F627463" => Some("BTC"),
        "695F657468" => Some("ETH"),
        "695F656F73" => Some("EOS"),
        "695F636F736D6F73" => Some("COSMOS"),
        "695F696D6B" => Some("IMK"),
        _  => None,
    }
}
pub fn get_instid_by_appname(appname: &str) -> Option<&str> {
    match appname {
        "BTC" => Some("695F627463"),
        "ETH" => Some("695F657468"),
        "EOS" => Some("695F656F73"),
        "COSMOS" => Some("695F636F736D6F73"),
        "IMK" => Some("695F696D6B"),
        _  => None,
    }
}