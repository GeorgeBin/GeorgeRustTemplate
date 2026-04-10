# Dashboard

一个基于 Rust 与 Slint 构建的轻量级原生桌面壳应用。

## 项目简介

Dashboard 当前定位为桌面演示壳层，用于承载后续的产品原型或演示内容。当前工程保留的是一套简洁的原生桌面基础框架，主要包括：

- Windows / Linux 自绘标题栏
- 侧边栏导航
- 内置 Examples 页面（NTP 查询 / Slint 交互示例）
- 设置持久化
- 主题切换
- 中英文界面
- 更新检查

当前在工作区中，`dashboard` 已作为正式桌面应用入口使用。

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
cargo run -p dashboard
```

构建发布版本：

```powershell
cargo build -p dashboard --release
```

工作区自检：

```powershell
just check
just test
just lint
```

Windows / macOS 构建：

```powershell
cargo build -p dashboard --release
cargo bundle -p dashboard --release --target x86_64-apple-darwin
```

当前 macOS `.app` 的 bundle 元信息在 `Cargo.toml` 的 `[package.metadata.bundle]` 中维护。

## 代码入口

- UI 入口：`assets/slint/app.slint`
- 应用启动入口：`src/main.rs`
- 配置与系统信息：`src/config/mod.rs`
