

use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use ethereum_types::{Address, H256, U256};
use std::str::FromStr;
use coin_ethereum::address::EthAddress;

#[no_mangle]
pub extern "C" fn get_address() {
    EthAddress::get_address("m/44'/60'/0'/0/0");
}

#[no_mangle]
pub extern "C" fn sign_transaction() {
    let nonce = U256::from(8);
    let gas_price = U256::from(20000000008 as usize);
    let gas_limit = U256::from(189000);
    let to = Action::Call(Address::from_str("3535353535353535353535353535353535353535").unwrap());
    let value = U256::from(512 as usize);
    // let data = Vec::new();

    let path = "m/44'/60'/0'/0/0";
    let payment = "0.01 ETH";
    let receiver = "0xE6F4142dfFA574D1d9f18770BF73814df07931F3";
    let sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b";
    let fee = "0.0032 ether";

    // Transaction::sign_transaction(
    //     nonce, gas_price, gas_limit, to, value, data, None, path, payment, receiver, sender, fee,
    // );
}
