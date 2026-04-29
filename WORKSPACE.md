# RustTemplate Workspace 约定（V1）

## 1. 文档目的

本文档用于定义 **RustTemplate 模板工程的 workspace 组织方式、crate 边界、依赖规则、构建约束与演进方式**。

本文件是 `ARCHITECTURE.md` 的工程落地补充文档。

- `ARCHITECTURE.md` 解决：为什么这样分层、总体架构如何设计
- `WORKSPACE.md` 解决：目录如何落地、crate 如何组织、依赖如何约束、开发如何推进

本文档默认面向如下目标：

- 模板工程使用 Rust workspace 管理多个 crate
- 核心逻辑平台无关
- 可逐步导出到 Android、iOS、OpenHarmony、C/C++ 等平台
- 适合长期演进，而不是一次性 Demo 项目

---

## 2. Workspace 设计原则

### 2.1 单一 workspace，统一治理

整个模板工程使用 **单一 workspace** 管理所有 crate，而不是拆成多个相互松散的仓库。

原因：

- 便于统一版本治理
- 便于统一 lint / test / fmt / clippy / docs
- 便于共享基础 crate
- 便于统一 feature 策略
- 便于后续接入 CI/CD 与平台构建脚本

### 2.2 目录分层必须映射职责分层

workspace 的目录不是为了好看，而是为了表达依赖方向与职责边界。

因此目录必须体现以下事实：

- `base/` 是基础设施资产
- `crates/` 是模板内部核心能力层
- `bindings/` 是导出层
- `examples/` 是示例，不参与正式架构分层
- `tests/` 是集成与烟囱测试
- `xtask/` 是工程编排工具
- `docs/` 是架构与约定文档

### 2.3 workspace 中的 crate 必须“有边界”

严禁在 workspace 中出现职责模糊的 crate，例如：

- `common`
- `shared-common`
- `misc`
- `helper`
- `temp`
- `manager-utils`

如果一个 crate 无法用一句话定义边界，通常说明拆分不合理。

### 2.4 模板第一阶段避免极端微拆分

RustTemplate 是模板，不是论文式微服务示意图。

第一阶段应采用 **中等粒度 crate 划分**：

- 足够分层
- 足够清晰
- 但不为了“看起来专业”而拆太碎

建议先稳定以下核心 crate：

- base/error
- base/log
- base/time
- base/types
- crates/platform
- crates/platform-std
- crates/model
- crates/core
- crates/runtime
- crates/sdk
- crates/ffi-common

后续若业务复杂度确实增加，再继续拆分。



## 3. 推荐目录结构

## 3.1 标准目录

```text
rust-template/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── LICENSE
├── .gitignore
├── .cargo/
│   └── config.toml
├── base/
│   ├── error/
│   ├── log/
│   ├── time/
│   ├── types/
│   └── utils/
├── bindings/
│   ├── uniffi/
│   ├── c/
│   ├── android/
│   ├── ios/
│   └── ohos/
├── crates/
│   ├── platform/
│   ├── platform-std/
│   ├── model/
│   ├── core/
│   ├── runtime/
│   ├── sdk/
│   └── ffi-common/
├── docs/
│   ├── ARCHITECTURE.md
│   ├── WORKSPACE.md
│   ├── ERRORS.md
│   ├── API.md
│   └── RELEASE.md
├── examples/
│   ├── rust-basic/
│   └── rust-demo/
├── tests/
│   ├── integration/
│   └── ffi-smoke/
├── xtask/
│   ├── Cargo.toml
│   └── src/
└── .github/
    └── workflows/
```



## 4. 各目录的工程职责

## 4.1 `base/`

放置跨项目可复用的基础设施 crate。

特点：

- 尽量不理解具体业务
- 尽量不依赖上层 crate
- 可被多个未来 SDK 复用
- 追求稳定、克制、低耦合

子目录建议：

- `error/`：错误基础设施
- `log/`：日志抽象
- `time/`：时间抽象
- `types/`：通用基础类型
- `utils/`：非常克制的轻量工具函数

## 4.2 `crates/`

放置模板内部主要能力层。

这里的 crate 共同组成一个完整 SDK 的内部结构。

建议包括：

- `platform/`：平台契约层
- `platform-std/`：默认 `std` 实现
- `model/`：领域公共模型
- `core/`：规则与状态机层
- `runtime/`：执行与调度层
- `sdk/`：Rust 对外门面层
- `ffi-common/`：FFI 共享能力层

## 4.3 `bindings/`

放置跨语言 / 跨平台导出层。

这里的 crate 只负责：

- ABI 暴露
- 类型转换
- 回调桥接
- 导出产物整理

不得承载：

- 核心业务逻辑
- 第二套状态机
- 与平台 UI 直接耦合的逻辑

## 4.4 `examples/`

放置 Rust 示例工程。

建议分为：

- `rust-basic/`：最小化示例
- `rust-advanced/`：高级或调试型示例

目的：

- 帮助验证 `sdk` crate 的对外体验
- 用于文档示例与 smoke test
- 用于演示默认 `platform-std` 的接入方式

## 4.5 `tests/`

放置集成测试与 FFI 烟囱测试。

建议：

- `integration/`：Rust 端完整生命周期测试
- `ffi-smoke/`：导出句柄/错误码/回调烟囱测试

不要把复杂集成测试全部挤进某一个 crate 内部的 `tests/` 目录。

## 4.6 `xtask/`

放置工程编排工具。

典型职责：

- codegen
- copy artifacts
- build android
- build ohos
- build ios
- release prepare
- doctor / env check

推荐优先用 Rust 写，而不是主要依赖 shell 脚本。

## 4.7 `docs/`

放置架构与工程约束。

建议长期维护以下文档：

- `ARCHITECTURE.md`
- `WORKSPACE.md`
- `ERRORS.md`
- `API.md`
- `RELEASE.md`

---

## 5. 推荐 crate 划分

## 5.1 `base/error`

边界：统一错误基础设施。

应放：

- `Result<T>`
- `ErrorCode`
- `ErrorCategory`
- `BaseError`
- `ErrorContext`

不应放：

- 具体业务生命周期错误实现
- FFI 空指针错误
- JNI / NAPI / UniFFI 桥接错误

## 5.2 `base/log`

边界：日志抽象。

应放：

- `LogLevel`
- `LogRecord`
- `Logger` trait
- `LogFacade`

不应放：

- Android Logcat 具体实现
- iOS NSLog 具体实现
- 平台文件日志路径规则

## 5.3 `base/time`

边界：时间抽象。

应放：

- `Clock`
- `MonotonicClock`
- `WallClock`
- 时间工具

## 5.4 `base/types`

边界：与具体业务无关的小基础模型。

应放：

- 通用 ID
- 版本号
- 标签类型
- 分页类型
- 轻量共享 DTO

不应放：

- `PttEvent`
- `RtcSessionInfo`

## 5.5 `base/utils`

边界：非常克制的通用工具函数集合。

应放：

- IP 校验
- 字节/字符串帮助函数
- 与业务无关的小型辅助函数

不应放：

- 领域规则
- 大型通用框架代码
- 放不下就塞进去的杂项逻辑

## 5.6 `crates/platform`

边界：平台能力契约。

应放：

- `TaskSpawner`
- `SleepProvider`
- `FileSystem`
- `KvStore`
- `SecureStore`
- `NetworkClient`
- `ConnectivityProvider`
- `RandomProvider`

不应放：

- `tokio` 具体实现
- Android/iOS/OHOS 的平台实现

## 5.6 `crates/platform-std`

边界：默认 `std` 环境实现。

应放：

- 基于 `tokio` 或其他运行时的默认 spawner
- tracing logger adapter
- 文件系统默认实现
- 测试/桌面环境可直接复用的实现

不应放：

- Android JNI
- OHOS NAPI
- iOS Foundation runtime 桥接

## 5.7 `crates/model`

边界：领域公共模型。

应放：

- `Config`
- `Event`
- `Status`
- `Command DTO`
- 对外稳定结构体

建议：

- 第一阶段把 `config` / `event` / `dto` 先统一放在 `model`
- 后期确实有必要，再从 `model` 中拆细

## 5.8 `crates/core`

边界：规则、状态机、业务编排。

应放：

- 生命周期状态机
- 命令校验
- 状态迁移
- 事件生成规则
- 核心服务装配规则

不应放：

- 导出层逻辑
- 平台 API 直接调用
- 大量运行时细节

## 5.9 `crates/runtime`

边界：执行与调度。

应放：

- actor / worker
- event dispatcher
- supervisor
- shutdown coordination
- callback dispatch 前的统一分发逻辑

不应放：

- 第二套业务规则
- bindings 侧逻辑

## 5.10 `crates/sdk`

边界：Rust 用户门面层。

应放：

- `Builder`
- `Handle`
- `start/stop/destroy`
- 订阅接口
- re-export `model`

约束：

- Rust 用户原则上只依赖这个 crate
- 不建议要求外部直接依赖 `core` 或 `runtime`

## 5.11 `crates/ffi-common`

边界：FFI 公共缓冲区。

应放：

- `HandleId`
- `OpaqueHandle`
- `FfiError`
- `FfiResultCode`
- `CallbackSlot`
- `AbiVersion`

约束：

- 尽量保持薄
- 不要演化成“所有 bindings 的复杂共享大仓库”

---

## 6. 根 `Cargo.toml` 约定

推荐根 `Cargo.toml` 统一管理：

- workspace members
- workspace package metadata
- workspace dependency versions
- lint 配置
- 常用 profile 配置

示例：

```toml
[workspace]
resolver = "2"
members = [
    "xtask",

    "base/error",
    "base/log",
    "base/time",
    "base/types",
    "base/utils",

    "crates/platform",
    "crates/platform-std",
    "crates/model",
    "crates/core",
    "crates/runtime",
    "crates/sdk",
    "crates/ffi-common",

    "bindings/uniffi",
    "bindings/c",
    "bindings/android",
    "bindings/ios",
    "bindings/ohos",

    "examples/rust-basic",
    "examples/rust-advanced",
]

[workspace.package]
edition = "2024"
license = "MIT"
repository = "https://example.com/rust-template"
rust-version = "1.85"

[workspace.dependencies]
thiserror = "2"
tracing = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }

[workspace.lints.rust]
unsafe_code = "warn"

[workspace.lints.clippy]
todo = "warn"
unimplemented = "warn"
unwrap_used = "warn"
expect_used = "warn"
```

### 6.1 关于 `workspace.dependencies`

建议尽量把公共依赖版本统一放在根上，原因：

- 避免各 crate 版本漂移
- 便于升级
- 便于审查依赖

但也要克制：

- 只有真正跨 crate 共享的依赖才放根上
- 不要把所有偶发依赖都塞进 `workspace.dependencies`

---

## 7. 依赖方向规则

## 7.1 总体方向

```text
base/*
   ↑
crates/platform
   ↑
crates/model
   ↑
crates/core
   ↑
crates/runtime
   ↑
crates/sdk
   ↑
crates/ffi-common
   ↑
bindings/*
```

## 7.2 强约束

必须遵守：

- `base/*` 不得依赖 `crates/*`
- `base/*` 不得依赖 `bindings/*`
- `platform` 不得依赖 `core` / `runtime` / `sdk`
- `model` 不得依赖 `runtime` / `sdk` / `bindings/*`
- `core` 不得依赖 `bindings/*`
- `runtime` 不得依赖 `bindings/*`
- `sdk` 不得依赖具体平台 bindings
- `bindings/*` 可以依赖 `sdk` 与 `ffi-common`

## 7.3 一个重要约束

不允许为了“方便”在 `core` 中直接写：

- `cfg(target_os = "android")`
- `jni` 相关调用
- `napi` 相关调用
- `objc` 相关调用

平台差异必须通过 `platform` 契约 + 上层实现注入。

---

## 8. feature 组织策略

## 8.1 feature 分类

建议把 feature 分为四组：

### 基础能力

- `std`
- `runtime-tokio`
- `runtime-smol`
- `tracing-log`
- `file-log`

### 领域能力

- `net`
- `http`
- `crypto`
- `kv`
- `secure-store`
- `telemetry`

### 导出能力

- `ffi`
- `uniffi`
- `c-abi`
- `android-binding`
- `ios-binding`
- `ohos-binding`

### 测试与调试能力

- `test-util`
- `mock-platform`
- `internal-metrics`

## 8.2 feature 原则

- feature 表达“能力”，不要表达“目录存在”
- bindings feature 不得污染 core 的默认依赖集
- 默认 feature 保持克制
- 重量依赖应尽量由 feature 显式开启

---

## 9. 构建、检查与测试约定

## 9.1 基础命令

建议根目录统一支持：

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
cargo build --workspace
```

## 9.2 分层测试

建议分四层：

### 单元测试

覆盖：

- `base/*`
- `model`
- `core`
- `runtime`

### 集成测试

覆盖：

- `sdk` 生命周期
- 事件流
- fake platform 下的端到端行为

### FFI 烟囱测试

覆盖：

- create / destroy handle
- register / unregister callback
- invalid handle error
- abi version 查询

### 平台 smoke test

覆盖：

- Android 最小调用
- iOS 最小调用
- OHOS 最小调用

## 9.3 CI 最小要求

建议至少有以下检查：

- fmt
- clippy
- test
- docs lint（按需）
- bindings smoke（按阶段启用）

---

## 10. 示例与文档约定

## 10.1 examples 的作用

每个 example 都应回答一个明确问题，例如：

- 如何用默认平台实现启动 SDK
- 如何订阅事件
- 如何优雅 stop / destroy
- 如何替换 logger / clock / network

不要写“为了展示很多能力而过于臃肿”的 example。

## 10.2 docs 的作用

每份文档都要有清晰边界：

- `ARCHITECTURE.md`：总体架构与设计原则
- `WORKSPACE.md`：工程组织与依赖规则
- `ERRORS.md`：错误码、错误语义、错误映射
- `API.md`：对外接口说明
- `RELEASE.md`：版本、产物、发布流程

---

## 11. 命名约定

建议延续当前你已经确定的双前缀策略：

- `base/*`：`{{org}}-base-*`
- `crates/*`：`{{domain}}-*`
- `bindings/*`：`{{domain}}-binding-*`

例如：

- `george-base-error`
- `george-base-log`
- `ptt-runtime`
- `rtc-binding-uniffi`

推荐：

- Rust 用户主入口尽量直接命名为 `{{domain}}`
- 内部层再使用 `{{domain}}-core` / `{{domain}}-runtime`

例如：

- `ptt`
- `ptt-core`
- `ptt-runtime`

---

## 12. 首版落地建议

对 RustTemplate 第一版，我建议 workspace 先落以下内容：

### 必建

- `base/error`
- `base/log`
- `base/time`
- `base/types`
- `crates/platform`
- `crates/platform-std`
- `crates/model`
- `crates/core`
- `crates/runtime`
- `crates/sdk`
- `examples/rust-basic`
- `docs/ARCHITECTURE.md`
- `docs/WORKSPACE.md`

### 第二批再建

- `base/utils`
- `crates/ffi-common`
- `bindings/c`
- `bindings/uniffi`
- `tests/ffi-smoke`
- `xtask`

### 第三批再建

- `bindings/android`
- `bindings/ios`
- `bindings/ohos`
- `.github/workflows/`
- `docs/ERRORS.md`
- `docs/API.md`
- `docs/RELEASE.md`

---

## 13. 不推荐的 workspace 做法

以下做法应避免：

- 一开始就拆出过多小 crate
- 把领域公共模型塞进 `base/types`
- 把 bindings 逻辑混进 `sdk`
- 在 `core` 中直接依赖 tokio/jni/napi/objc 平台细节
- 不定义 crate 边界就先写大量代码
- 没有 smoke test 就直接开始做 Android/iOS/OHOS 导出
- 让 `utils` 成为垃圾桶目录
- 在多个 crate 中重复维护同一错误码与生命周期定义

---

## 14. 结论

RustTemplate 的 workspace 应遵循以下主线：

- 用单一 workspace 统一治理
- 用目录分层表达职责分层
- 用单向依赖保证长期可维护性
- 用中等粒度 crate 划分平衡可读性与复杂度
- 先稳定 `base/platform/model/core/runtime/sdk` 主链路
- 再逐步扩展 FFI、bindings、xtask、平台产物

如果 `ARCHITECTURE.md` 解决的是“架构正确性”，那么 `WORKSPACE.md` 解决的就是“工程可执行性”。
