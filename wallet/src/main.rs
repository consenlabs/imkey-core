//extern crate bitcoin;
extern crate bitcoincore_rpc;
extern crate secp256k1;

use wallet::bitcoin_trans::{Utxo, bitcoin_trans_data};

fn main(){
    let mut utxos = Vec::new();
    utxos.push(Utxo{
        txHash : "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a".to_string(),
        vout : 0,
        amount : 200000000,
        address : "1Fj93kpLwM1KgTN6C75Z5Bokhays4MmJae".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    utxos.push(Utxo{
        txHash : "45ef8ac7f78b3d7d5ce71ae7934aea02f4ece1af458773f12af8ca4d79a9b531".to_string(),
        vout : 0,
        amount : 200000000,
        address : "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    utxos.push(Utxo{
        txHash : "14c67e92611dc33df31887bbc468fbbb6df4b77f551071d888a195d1df402ca9".to_string(),
        vout : 0,
        amount : 200000000,
        address : "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    utxos.push(Utxo{
        txHash : "117fb6b85ded92e87ee3b599fb0468f13aa0c24b4a442a0d334fb184883e9ab9".to_string(),
        vout : 0,
        amount : 200000000,
        address : "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    utxos.push(Utxo{
        txHash : "45ef8ac7f78b3d7d5ce71ae7934aea02f4ece1af458773f12af8ca4d79a9b531".to_string(),
        vout : 0,
        amount : 200000000,
        address : "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    utxos.push(Utxo{
        txHash : "14c67e92611dc33df31887bbc468fbbb6df4b77f551071d888a195d1df402ca9".to_string(),
        vout : 0,
        amount : 200000000,
        address : "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    utxos.push(Utxo{
        txHash : "117fb6b85ded92e87ee3b599fb0468f13aa0c24b4a442a0d334fb184883e9ab9".to_string(),
        vout : 0,
        amount : 200000000,
        address : "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g".to_string(),
        script_pubkey : "76a914a189f2f7836812aa7a0e36e28a20a10e64010bf688ac".to_string(),
        derived_path : "0/22".to_string(),
    });
    let request_data = bitcoin_trans_data{
        to: "18pMkq6HK5HR36jr7bSd39MpkVCfnP68VV".to_string(),
        amount: 750000000,
        fee: 502130,
        outputs: utxos,
        from: "3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy".to_string()
    };
    request_data.bitcoin_sign();

}