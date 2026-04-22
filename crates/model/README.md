# template-model / crates/model

`crates/model` 是 RustTemplate 的产品/领域公开模型层。

它负责定义会被 `core`、`runtime`、`sdk`、`bindings` 共同理解的稳定领域模型，不负责运行逻辑、平台映射或 FFI 包装。

本 crate 默认启用 `std`，同时保持 `no_std` 兼容风格，避免把不必要的运行时依赖带入模型层。

## 边界

与 `george-base-types` 的边界：

- `george-base-types` 放平台无关、跨 crate 复用的小型值对象
- `crates/model` 放某个产品或能力域自己的公开 request / response / params / state 模型

因此：

- `NonEmptyString` 这类基础值对象属于 `george-base-types`
- `NtpRequest`、`NtpResponse` 这类领域公开模型属于 `crates/model`

与 `core` / `runtime` / `sdk` / `bindings` 的边界：

- `crates/model` 只表达语义模型
- `core` 和 `runtime` 负责执行逻辑、调度、生命周期和运行时集成
- `sdk` 负责对外能力封装
- `bindings` 负责把模型映射到 Kotlin / Swift / ArkTS / C 等边界

本 crate 不放：

- 运行逻辑
- 平台 DTO
- FFI 包装
- `tokio::*`、`tracing`
- `SystemTime`、`PathBuf`
- `Arc`、`Mutex`
- callback / handle / runtime state

## 当前首版范围

当前只收敛两个模块：

- `common`
- `ntp`

其中：

- `common/` 只做最小预留，用于未来承载多个领域共享、但又不适合进入 `base/types` 的领域公共模型；在出现真实共享模型之前，不应把它当成杂物入口
- `ntp/` 提供稳定的 NTP 请求与响应模型

## 为什么 `NtpResponse` 不再暴露 `SystemTime`

`SystemTime` 适合运行时内部计算，但不适合作为领域公开模型字段长期外露。

这里改为：

- `server_unix_millis: u64`
- `round_trip_millis: Option<u32>`

原因很直接：

- 更容易被 `sdk`、`bindings`、日志和测试共同解释
- 更容易跨 crate、跨语言传递
- 避免把运行时类型直接扩散到模型层

## 当前保持克制的字段设计

`NtpRequest` 目前继续使用：

- `port: u16`
- `timeout_millis: u32`

这不是遗漏建模，而是首版刻意保持克制。

如果后续多个领域模块都重复出现端口、超时等字段，再考虑把 `PortNumber`、`TimeoutMillis` 这类值对象下沉到 `george-base-types`。

## 推荐放什么 / 不放什么

推荐放：

- request / response 模型
- params / options 模型
- 对外稳定的 state / event / result 枚举

不推荐放：

- 网络执行逻辑
- runtime 状态对象
- 平台回调上下文
- FFI 专用结构
- 只为了未来可能需要而预埋的大量抽象

## 推荐目录结构

```text
crates/model/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── common/
    │   └── mod.rs
    └── ntp/
        ├── mod.rs
        ├── request.rs
        └── response.rs
```
