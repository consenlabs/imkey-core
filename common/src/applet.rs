use crate::constants::{
    BTC_AID, COSMOS_AID, EOS_AID, ETH_AID, FILECOIN_AID, IMK_AID, KUSAMA_AID, NERVOS_AID,
    POLKADOT_AID, TEZOS_AID, TRON_AID,
};

//todo: check the aid
pub fn get_appname_by_instid(instid: &str) -> Option<&str> {
    match instid {
        "695F627463" => Some("Bitcoin"),
        "695F657468" => Some("Ethereum"),
        "695F656F73" => Some("EOS"),
        "695F636F736D6F73" => Some("Cosmos"),
        "695F6B315F66696C" => Some("Filecoin"),
        "695F65645F6B736D" => Some("Kusama"),
        "695F65645F646F74" => Some("Polkadot"),
        "695F6B315F747278" => Some("TRON"),
        "695F626368" => Some("Bitcion Cash"),
        "695F6C7463" => Some("Litecoin"),
        "695F696D6B" => Some("IMK"),
        NERVOS_AID => Some("Nervos"),
        TEZOS_AID => Some("Tezos"),
        _ => None,
    }
}
pub fn get_instid_by_appname(appname: &str) -> Option<&str> {
    match appname {
        "Nervos" => Some(NERVOS_AID),
        "Tezos" => Some(TEZOS_AID),
        "Bitcoin" => Some("695F627463"),
        "Ethereum" => Some("695F657468"),
        "EOS" => Some("695F656F73"),
        "Cosmos" => Some("695F636F736D6F73"),
        "Filecoin" => Some("695F6B315F66696C"),
        "Kusama" => Some("695F65645F6B736D"),
        "Polkadot" => Some("695F65645F646F74"),
        "TRON" => Some("695F6B315F747278"),
        "Filecoin" => Some("4695F6B315F66696C"),
        "Bitcion Cash" => Some("695F626368"),
        "Litecoin" => Some("695F6C7463"),
        "IMK" => Some("695F696D6B"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::applet::{get_appname_by_instid, get_instid_by_appname};
    #[test]
    fn get_appname_by_instid_test() {
        assert_eq!(get_appname_by_instid("695F627463").unwrap(), "Bitcoin");
        assert_eq!(get_appname_by_instid("695F657468").unwrap(), "Ethereum");
        assert_eq!(get_appname_by_instid("695F656F73").unwrap(), "EOS");
        assert_eq!(get_appname_by_instid("695F636F736D6F73").unwrap(), "Cosmos");
        assert_eq!(
            get_appname_by_instid("695F6B315F66696C").unwrap(),
            "FILECOIN"
        );
        assert_eq!(get_appname_by_instid("695F626368").unwrap(), "Bitcoin");
        assert_eq!(get_appname_by_instid("695F6C7463").unwrap(), "Litecoin");
        assert_eq!(get_appname_by_instid("695F696D6B").unwrap(), "IMK");
        assert!(get_appname_by_instid("1111111111").is_none());
    }

    #[test]
    fn get_instid_by_appname_test() {
        assert_eq!(get_instid_by_appname("Bitcoin").unwrap(), "695F627463");
        assert_eq!(get_instid_by_appname("Ethereum").unwrap(), "695F657468");
        assert_eq!(get_instid_by_appname("EOS").unwrap(), "695F656F73");
        assert_eq!(get_instid_by_appname("Cosmos").unwrap(), "695F636F736D6F73");
        assert_eq!(get_instid_by_appname("IMK").unwrap(), "695F696D6B");
        assert_eq!(
            get_instid_by_appname("Filecoin").unwrap(),
            "695F6B315F66696C"
        );
        assert_eq!(get_instid_by_appname("Bitcion Cash").unwrap(), "695F626368");
        assert_eq!(get_instid_by_appname("Litecoin").unwrap(), "695F6C7463");
        assert!(get_instid_by_appname("APPLET").is_none());
    }
}
