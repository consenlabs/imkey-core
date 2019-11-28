# imkey-connector



# Build

### 安装 rust

* 安装 rustup `$ curl https://sh.rustup.rs -sSf | sh`
* 添加环境变量 `$ source $HOME/.cargo/env`

### Android

1. 安装 android target

   `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android` 

2. 安装 android studio，安装NDK

4. 编译
   * 每次修改rust代码需要在connector 目录下执行 `./build-android.sh` 
   * 用android studio 打开 android 目录运行 example

### IOS
1. 安装xcode

2. 编译
   * 每次修改rust代码需执行 `./build-ios.sh` 
   * 双击打开 ios/Examples/ 下的 .xcworkspace 即可打开 example