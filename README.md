# imkey-connector



# Build

### 安装 rust

* 安装 rustup `$ curl https://sh.rustup.rs -sSf | sh`
* 添加环境变量 `$ source $HOME/.cargo/env`

### Android

1. 安装 android target

   `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android` 

2. 安装 android studio

3. 配置`~/.cargo/config` ，xxx 替换为用户名

   ```
   [target.aarch64-linux-android]
   ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar"
   linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android22-clang"
   
   
   [target.armv7-linux-androideabi]
   ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar"
   linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi22-clang"
   
   [target.i686-linux-android]
   ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android-ar"
   linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android22-clang"
   
   
   [target.x86_64-linux-android]
   ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android-ar"
   linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android22-clang"
   ```

4. 编译
   * 每次修改rust代码需要在connector 目录下执行 `./build-android.sh` 
   * 用android studio 打开 android 目录运行 example

### IOS

1. 安装 ios target

   `rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios x86_64-apple-ios`

2. 安装xcode

3. 安装cargo lipo 和 cbindgen

   ```
   cargo install cargo-lipo   
   cargo install cbindgen 
   ```

4. 编译
   * 每次修改rust代码需执行 `./build-ios.sh` 
   * 双击打开 ios/Examples/ 下的 .xcworkspace 即可打开 example