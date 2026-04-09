# GeorgeRustTemplate
Rust 程序模板工程



### IDE

RustRover 2025.2.1

插件：Slint



### 依赖

UI 相关：

- [Slint](https://slint.dev/) ：1.15.1  [Template](https://github.com/slint-ui/slint-rust-template)



### 工程结构

build：打包生成的可执行文件或库文件。目录：build/平台/编译类型/文件

corelib：核心功能，尽量减少外部依赖

dashboard：项目正式桌面端，使用 Slint 构建原生 UI。

demo：用于演示各种功能

executable：历史模板占位包，当前不再作为主桌面应用入口。

executable_esp32：

shared：编写库文件（例如：Android.so、ohos.so），主要包含各种桥接代码。

unleash：用于放置脚本。



> 其他：Zap：非常简短、有动感，表示“快速执行”、“搞定”。



#### 二级目录：Dashboard / Demo

```lua
create
 ├── src：代码
 │    ├── ui：UI 相关
 │    │
 │    └── x
 │
 ├── ware：资源文件
 │    ├── logo：应用图标
 │    │    ├── logo.png：UI 使用
 │    │    ├── macOS.icns：打包 macOS.app 使用
 │    │    └── windows.ico：打包 windows.exe 使用
 │    │
 │    ├── slint：slint UI 文件
 │    │    ├── app.slint / main.slint：主界面
 │    │    └── xxx.slint：
 │    │
 │    └── x
 │
 ├── build.rs：编译相关（slint 文件编译、exe 增加图标等）
 ├── BuilConfig.toml：编译指导（处理编译 windows 时的指向）
 └── Cargo.toml
```




### 打包

Windows 系统 .exe 打包：cargo + [winresource](https://github.com/BenjaminRi/winresource)（给 exe 添加图标）

```shell
# 环境配置：
# 安装 target
rustup target add x86_64-pc-windows-gnu
# 安装 mingw，这一步可能需要很久
brew install mingw-w64
# 配置工程 BuildConfig.toml、build.rs
参考工程文件


# 项目正式桌面端
cargo build --package dashboard --release --target x86_64-pc-windows-gnu
# 示例程序
cargo build --package demo --release --target x86_64-pc-windows-gnu
```



macOS 系统 .app 打包：[cargo-bundle](https://github.com/burtonageo/cargo-bundle)（暂时仅使用其 macOS 打包能力，仅支持 Intel 芯片）

```shell
# 环境配置：
# 安装 cargo-bundle
cargo install cargo-bundle
# 配置工程 Cargo.toml
略


# 打包 macOS 程序（.app），备注：暂不支持 arm 芯片
cargo bundle --package dashboard --release --target x86_64-apple-darwin
# 示例程序
cargo bundle --package demo --release --target x86_64-apple-darwin
```



Android 端使用：

生成 so 文件、kt 文件

拷贝 kt 文件

拷贝 so 文件

添加依赖：implementation "net.java.dev.jna:jna:5.12.0@aar"
