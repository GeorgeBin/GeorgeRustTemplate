# GeorgeRustTemplate
Rust 程序模板工程



### IDE

RustRover 2025.1.3

插件：Slint



### 依赖

UI 相关：

- [Slint](https://slint.dev/) ：1.12.1  [Template](https://github.com/slint-ui/slint-rust-template)



### 打包

Windows 系统 .exe 打包：cargo + [winresource](https://github.com/BenjaminRi/winresource)（给 exe 添加图标）

```shell
# 安装 target
rustup target add x86_64-pc-windows-gnu

# 安装 mingw，这一步可能需要很久
brew install mingw-w64

# 配置工程 BuildConfig.toml、build.rs
参考工程文件

# 打包 exe
cargo build --release --target x86_64-pc-windows-gnu
```



macOS 系统 .app 打包：[cargo-bundle](https://github.com/burtonageo/cargo-bundle)（暂时仅使用其 macOS 打包能力，仅支持 Intel 芯片）

```shell
# 安装 cargo-bundle
cargo install cargo-bundle
# 配置工程 Cargo.toml
略
# 打包 macOS 程序（.app），备注：暂不支持 arm 芯片
cargo bundle --release --target x86_64-apple-darwin
```



