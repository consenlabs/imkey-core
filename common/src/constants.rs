pub const URL: &str = "https://imkeyserver.com:10443/imkey/";

pub const TSM_ACTION_SE_SECURE_CHECK: &str = "/seSecureCheck";
pub const TSM_ACTION_APP_DOWNLOAD: &str = "/appDownload";
pub const TSM_ACTION_APP_UPDATE: &str = "/appUpdate";
pub const TSM_ACTION_APP_DELETE: &str = "/appDelete";
pub const TSM_ACTION_SE_ACTIVATE: &str = "/seActivate";
pub const TSM_ACTION_SE_QUERY: &str = "/seInfoQuery";
pub const TSM_ACTION_AUTHCODE_STORAGE: &str = "/authCodeStorage";
pub const TSM_ACTION_DEVICE_CERT_CHECK: &str = "/deviceCertCheck";

pub const TSM_RETURN_CODE_SUCCESS: &str = "000000";

//apud related constant
pub const LC_MAX: u32 = 245;

pub const ETH_AID: &str = "695F657468";
pub const EOS_AID: &str = "695F656F73";
pub const BTC_AID: &str = "695F627463";
pub const COSMOS_AID: &str = "695F636F736D6F73";

//path
pub const COSMOS_PATH: &str = "m/44'/118'/0'/0/0";
pub const EOS_PATH: &str = "m/44'/194'/0'/0/0";

pub const VERSION: &str = "1.2.00";

pub const MAX_UTXO_NUMBER: usize = 252;
pub const EACH_ROUND_NUMBER: usize = 5;
pub const DUST_THRESHOLD: i64 = 2730;
