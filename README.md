# GeorgeRustTemplate

Rust workspace 模板工程

包含：

- `baselib`：纯 Rust 基础功能库：核心功能，尽量减少外部依赖
- `build`：打包生成的可执行文件或库文件（会被 Git 忽略）
- `corelib`：纯 Rust 核心逻辑层
- `demo`：桌面端模板，基于 Slint，dashboard 样式
- `platforms`：跨语言桥接层
- `samples`： 各个平台的使用示例（Android、ohos、iOS 等）
- `unleash`：打包与发布



## Quick Start

#### 环境基线：

- Rust toolchain：`stable`
- 组件：`clippy`、`rustfmt`
- UI 框架：[Slint 1.15.1](https://slint.dev/)



#### 常用命令：

```shell
# 运行正式桌面端
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
- 桌面端元数据：`demo/Cargo.toml`
- 移动端绑定配置：`shared/uniffi.toml`



## 工程结构

`demo` 目录约定：

```lua
demo
 ├── assets
 │    ├── font
 │    ├── i18n
 │    ├── logo
 │    ├── slint
 │    └── ...
 ├── src
 │    ├── app
 │    ├── ui
 │    └── ...
 ├── build.rs
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

cargo build --package demo --release --target x86_64-pc-windows-gnu
```

macOS `.app`：

```shell
cargo install cargo-bundle

cargo bundle --package demo --release --target x86_64-apple-darwin
```

Linux `.rpm`：

```shell
brew install zig
cargo install cargo-generate-rpm cargo-zigbuild just

just build-rpm-linux
```

默认产物路径：

```shell
target/generate-rpm/demo-*.rpm
```

交叉打包产物路径：

```shell
target/x86_64-unknown-linux-gnu/generate-rpm/demo-*.rpm
```

脚本额外复制到：

```shell
build/x86_64-unknown-linux-gnu/release/rpm/demo-v0.0.2.rpm
```

说明：

- `just build-rpm` 仅适用于 Linux 原生环境。
- 在 macOS 上生成 Linux RPM 时，使用 `cargo-zigbuild` 交叉编译 `x86_64-unknown-linux-gnu`，并通过 `demo/packaging/linux/generate-rpm-cross.toml` 显式引用 Linux ELF 产物。
- `just build-rpm` 会保留 `target/generate-rpm/` 下的默认产物；`just build-rpm-linux` 会保留 `target/x86_64-unknown-linux-gnu/generate-rpm/` 下的交叉打包产物，并额外复制一份到 `build/x86_64-unknown-linux-gnu/release/rpm/demo-v<version>.rpm`。
- 当前 macOS 交叉打包默认使用 `--auto-req disabled`，避免依赖 Linux `ldd`/`find-requires` 工具；如果需要发布到更严格的 RPM 环境，后续应补充手工依赖元数据。

Android：

```shell
just build-shared-android
just gen-shared-kotlin
```

Android 侧额外依赖：

```gradle
implementation "net.java.dev.jna:jna:5.12.0@aar"
```
