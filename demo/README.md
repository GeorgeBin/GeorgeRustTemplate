# Demo

一个基于 Rust 与 Slint 构建的轻量级原生桌面壳应用。

## 项目简介

Demo 当前定位为桌面演示壳层，用于承载后续的产品原型或演示内容。当前工程保留的是一套简洁的原生桌面基础框架，主要包括：

- Windows / Linux 自绘标题栏
- 侧边栏导航
- 内置 Examples 页面（NTP 查询 / Slint 交互示例）
- 设置持久化
- 主题切换
- 中英文界面
- 更新检查

当前在工作区中，`demo` 已作为正式桌面应用入口使用。

## 当前界面结构

- `首页`：用于放置后续演示内容的占位首页
- `示例`：内置 NTP 查询工具与 Slint 交互参考
- `设置`：语言、主题、日志路径、安装路径、更新检查周期等配置
- `关于`：版本信息、项目链接和更新入口

## 语言支持

当前界面支持以下语言模式：

- 简体中文
- English
- 跟随系统

当选择“跟随系统”时，程序会根据系统语言解析为 `zh-CN` 或 `en`。

## 开发

环境要求：

- Rust stable
- `clippy`
- `rustfmt`

本地运行：

```powershell
cargo run -p demo
```

构建发布版本：

```powershell
cargo build -p demo --release
```

工作区自检：

```powershell
just check
just test
just lint
```

Windows / macOS 构建：

```powershell
cargo build -p demo --release
cargo bundle -p demo --release --target x86_64-apple-darwin
```

当前 macOS `.app` 的 bundle 元信息在 `Cargo.toml` 的 `[package.metadata.bundle]` 中维护。

Linux RPM 打包：

```powershell
brew install zig
cargo install cargo-generate-rpm cargo-zigbuild just
just build-rpm-linux
```

默认 RPM 产物输出到 `target/generate-rpm/`。

- 在 Linux 原生环境中，可继续使用 `cargo build -p demo --release` 和 `cargo generate-rpm -p demo`。
- 在 macOS 上交叉打 Linux RPM 时，`just build-rpm-linux` 会先生成 `target/x86_64-unknown-linux-gnu/release/demo`，再通过 `demo/packaging/linux/generate-rpm-cross.toml` 打包。
- 当前 macOS 交叉打包默认关闭自动依赖扫描，因为 RPM 的自动依赖解析依赖 Linux 工具链。

## 代码入口

- UI 入口：`assets/slint/app.slint`
- 应用启动入口：`src/main.rs`
- 配置与系统信息：`src/config/mod.rs`
