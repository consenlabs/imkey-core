use crate::constants::{
    BTC_AID, COSMOS_AID, EOS_AID, ETH_AID, FILECOIN_AID, IMK_AID, KUSAMA_AID, NERVOS_AID,
    POLKADOT_AID, TEZOS_AID, TRON_AID,
};

pub fn get_appname_by_instid(instid: &str) -> Option<&str> {
    match instid {
        BTC_AID => Some("BTC"),
        ETH_AID => Some("ETH"),
        EOS_AID => Some("EOS"),
        COSMOS_AID => Some("COSMOS"),
        FILECOIN_AID => Some("FILECOIN"),
        KUSAMA_AID => Some("KUSAMA"),
        POLKADOT_AID => Some("POLKADOT"),
        TRON_AID => Some("TRON"),
        IMK_AID => Some("IMK"),
        NERVOS_AID => Some("NERVOS"),
        TEZOS_AID => Some("TEZOS"),
        _ => None,
    }
}
pub fn get_instid_by_appname(appname: &str) -> Option<&str> {
    match appname {
        "BTC" => Some(BTC_AID),
        "ETH" => Some(ETH_AID),
        "EOS" => Some(EOS_AID),
        "COSMOS" => Some(COSMOS_AID),
        "FILECOIN" => Some(FILECOIN_AID),
        "KUSAMA" => Some(KUSAMA_AID),
        "POLKADOT" => Some(POLKADOT_AID),
        "TRON" => Some(TRON_AID),
        "IMK" => Some(IMK_AID),
        "NERVOS" => Some(NERVOS_AID),
        "TEZOS" => Some(TEZOS_AID),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::applet::{get_appname_by_instid, get_instid_by_appname};
    #[test]
    fn get_appname_by_instid_test() {
        assert_eq!(get_appname_by_instid("695F627463").unwrap(), "BTC");
        assert_eq!(get_appname_by_instid("695F657468").unwrap(), "ETH");
        assert_eq!(get_appname_by_instid("695F656F73").unwrap(), "EOS");
        assert_eq!(get_appname_by_instid("695F636F736D6F73").unwrap(), "COSMOS");
        assert_eq!(
            get_appname_by_instid("695F6B315F66696C").unwrap(),
            "FILECOIN"
        );
        assert_eq!(get_appname_by_instid("695F696D6B").unwrap(), "IMK");
        assert!(get_appname_by_instid("1111111111").is_none());
    }

    #[test]
    fn get_instid_by_appname_test() {
        assert_eq!(get_instid_by_appname("BTC").unwrap(), "695F627463");
        assert_eq!(get_instid_by_appname("ETH").unwrap(), "695F657468");
        assert_eq!(get_instid_by_appname("EOS").unwrap(), "695F656F73");
        assert_eq!(get_instid_by_appname("COSMOS").unwrap(), "695F636F736D6F73");
        assert_eq!(get_instid_by_appname("IMK").unwrap(), "695F696D6B");
        assert_eq!(
            get_instid_by_appname("FILECOIN").unwrap(),
            "695F6B315F66696C"
        );
        assert!(get_instid_by_appname("APPLET").is_none());
    }
}
