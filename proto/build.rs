use std::env;
extern crate prost_build;

fn main() {
    // tcx-api
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/api.proto"], &["src/"]).unwrap();

    // tcx-eth
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/eth.proto"], &["src/"]).unwrap();

    // tcx-btc
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/btc.proto"], &["src/"]).unwrap();

    // tcx-eos
    env::set_var("OUT_DIR", "../common/src");
    prost_build::compile_protos(&["src/eos.proto"], &["src/"]).unwrap();

    // tcx-cosmos
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/cosmos.proto"], &["src/"]).unwrap();

    // device
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/device.proto"], &["src/"]).unwrap()
}

#[cfg(test)]
mod tests {
    use std::env;
    extern crate prost_build;

    #[test]
    fn it_works() {
    }
}