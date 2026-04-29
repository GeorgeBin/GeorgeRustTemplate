# template-model / crates/model

`crates/model` 是 RustTemplate 的产品/领域公开模型层。

它负责定义会被 `core`、`runtime`、`sdk`、`bindings` 共同理解的稳定领域模型，不负责运行逻辑、平台映射或 FFI 包装。

本 crate 默认启用 `std`，同时保持 `no_std` 兼容风格，避免把不必要的运行时依赖带入模型层。

## 边界

与 `base/*` 的边界：

- `base/*` 放平台无关、跨 crate 复用的小型值对象和协议
- `crates/model` 放某个产品或能力域自己的公开 request / response / params / state 模型

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

## 当前阶段

当前模板不内置任何具体业务模型，也不保留空的 `common/` 占位目录。

新增产品能力时，应按能力域创建清晰模块，并只放对外稳定的语义类型。只有当多个能力域确实共享某些领域模型，且这些模型又不适合进入 `base/*` 时，才新增 focused shared 模块。

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
