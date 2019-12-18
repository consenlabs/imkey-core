use std::env;
extern crate prost_build;

fn main() {
    // tcx-api
    env::set_var("OUT_DIR", "../wallet/interface/src");
    prost_build::compile_protos(&["src/api.proto"], &["src/"]).unwrap();

    //    // tcx-chain
    //    env::set_var("OUT_DIR", "../tcx-chain/src");
    //    prost_build::compile_protos(&["src/tron.proto"], &["src/"]).unwrap();

    // tcx-eth
    env::set_var("OUT_DIR", "../wallet/interface/src");
    prost_build::compile_protos(&["src/eth.proto"], &["src/"]).unwrap();

    //    let targets = vec!["arm64-v8a", "armeabi-v7a", "x86", "x86_64"];
    //    for target in targets {
    //        println!("cargo:rustc-link-search=../../android/tokencore/build/intermediates/cmake/release/obj/{}/", target);
    //    }
}
