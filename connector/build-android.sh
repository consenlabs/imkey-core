cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

cp target/aarch64-linux-android/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/arm64-v8a/libconnector.so
cp target/armv7-linux-androideabi/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/armeabi-v7a/libconnector.so
cp target/i686-linux-android/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/x86/libconnector.so
cp target/x86_64-linux-android/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/x86_64/libconnector.so





