# cargo clean
cargo lipo --release
cbindgen src/lib.rs -l c > target/connector.h

cp target/universal/release/libconnector.a ../ios/imKeyConnector/Classes/include/libconnector.a
cp target/connector.h ../ios/imKeyConnector/Classes/include/connector.h