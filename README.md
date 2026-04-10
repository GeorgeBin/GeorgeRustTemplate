# GeorgeRustTemplate

Rust workspace 模板工程，包含：

- `dashboard`：正式桌面端模板，基于 Slint 原生 UI
- `corelib`：纯 Rust 核心逻辑层
- `shared`：`uniffi` 跨语言桥接层
- `samples`： 各个平台的使用示例
- `demo`：示例与实验包

## Quick Start

环境基线：

- Rust toolchain：`stable`
- 组件：`clippy`、`rustfmt`
- UI 框架：[Slint 1.15.1](https://slint.dev/)

常用命令：

```shell
# 运行正式桌面端
cargo run -p dashboard

# 运行示例程序
cargo run -p demo

# 工作区自检
just check
just test
just lint
```

如果未安装 `just`，可以直接运行对应的 `cargo` 命令：

```shell
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## 模板定制清单

创建新项目后，优先替换以下占位信息：

- 仓库名：`GeorgeRustTemplate`
- 应用名、公司名、版权信息
- `bundle id` / `package_name`
- 更新地址、主页地址
- 图标与品牌资源

主要位置：

- 根工作区配置：`Cargo.toml`
- 桌面端元数据：`dashboard/Cargo.toml`
- 移动端绑定配置：`shared/uniffi.toml`

## 工程结构

- `build/`：打包生成的可执行文件或库文件
- `corelib/`：核心功能，尽量减少外部依赖
- `dashboard/`：项目正式桌面端
- `demo/`：示例包
- `executable/`：历史模板占位包，当前不再作为主桌面应用入口
- `shared/`：Android / OHOS 等跨语言桥接
- `unleash/`：构建脚本

`dashboard` / `demo` 目录约定：

```lua
create
 ├── src
 │    ├── ui
 │    └── ...
 ├── ware
 │    ├── logo
 │    ├── slint
 │    └── ...
 ├── build.rs
 ├── BuildConfig.toml
 └── Cargo.toml
```

## 质量门禁

仓库根目录已经补齐工作区级 CI，默认门禁为：

- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

建议在提交前先执行：

```shell
just verify
```

## 打包

Windows `.exe`：

```shell
rustup target add x86_64-pc-windows-gnu
brew install mingw-w64

cargo build --package dashboard --release --target x86_64-pc-windows-gnu
cargo build --package demo --release --target x86_64-pc-windows-gnu
```

macOS `.app`：

```shell
cargo install cargo-bundle

cargo bundle --package dashboard --release --target x86_64-apple-darwin
cargo bundle --package demo --release --target x86_64-apple-darwin
```

Android：

```shell
just build-shared-android
just gen-shared-kotlin
```

Android 侧额外依赖：

```gradle
implementation "net.java.dev.jna:jna:5.12.0@aar"
```
