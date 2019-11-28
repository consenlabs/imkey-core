ANDROID_NDK_TOOLCHAINS=$HOME/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin

JNI_LIBS=../android/imkeylibrary/src/main/jniLibs
if [ ! -d $JNI_LIBS ]; then
    mkdir $JNI_LIBS
    mkdir $JNI_LIBS/arm64-v8a
    mkdir $JNI_LIBS/armeabi-v7a
    mkdir $JNI_LIBS/x86
    mkdir $JNI_LIBS/x86_64
fi

pushd ../api
JNI_LIBS=../android/imkeylibrary/src/main/jniLibs

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release


cp ../target/aarch64-linux-android/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/arm64-v8a/libconnector.so
cp ../target/armv7-linux-androideabi/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/armeabi-v7a/libconnector.so
cp ../target/i686-linux-android/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/x86/libconnector.so
cp ../target/x86_64-linux-android/release/libconnector.so ../android/imkeylibrary/src/main/jniLibs/x86_64/libconnector.so
popd




