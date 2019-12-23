use std::env;
extern crate prost_build;

fn main() {
    // tcx-api
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/api.proto"], &["src/"]).unwrap();

    // tcx-eth
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/eth.proto"], &["src/"]).unwrap();

    // tcx-eth
    env::set_var("OUT_DIR", "../api/src");
    prost_build::compile_protos(&["src/device.proto"], &["src/"]).unwrap();
}
