# RustTemplate 首版初始化清单（V1）

本文档用于指导 RustTemplate 第一版工程初始化，适合：

- 自己手工创建工程
- 交给 Codex 生成初始骨架
- 作为 code review 的验收清单

---

## A. 根目录初始化

- [ ] 创建根目录 `rust-template/`
- [ ] 初始化 Git 仓库
- [ ] 创建根 `Cargo.toml`
- [ ] 创建 `Cargo.lock`
- [ ] 创建 `rust-toolchain.toml`
- [ ] 创建 `.gitignore`
- [ ] 创建 `README.md`
- [ ] 创建 `LICENSE`
- [ ] 创建 `.cargo/config.toml`
- [ ] 创建 `docs/` 目录

建议：

- `edition = "2024"`
- `resolver = "2"`
- 根上统一 `workspace.dependencies`
- 先把 lint、fmt、clippy 规则固定下来

---

## B. 第一批 crate

### base 层

- [ ] `base/error`
- [ ] `base/log`
- [ ] `base/time`
- [ ] `base/types`

### 内部主链路

- [ ] `crates/platform`
- [ ] `crates/platform-std`
- [ ] `crates/model`
- [ ] `crates/core`
- [ ] `crates/runtime`
- [ ] `crates/sdk`

### 示例与测试

- [ ] `examples/rust-basic`
- [ ] `tests/integration`

### 文档

- [ ] `docs/ARCHITECTURE.md`
- [ ] `docs/WORKSPACE.md`

---

## C. 每个 crate 的最小内容

### `base/error`

- [ ] `Result<T>`
- [ ] `ErrorCode`
- [ ] `ErrorCategory`
- [ ] `BaseError`
- [ ] 单元测试

### `base/log`

- [ ] `LogLevel`
- [ ] `LogRecord`
- [ ] `Logger` trait
- [ ] no-op logger
- [ ] 单元测试

### `base/time`

- [ ] `Clock` trait
- [ ] `SystemClock`
- [ ] `FakeClock`
- [ ] 单元测试

### `base/types`

- [ ] 通用 ID 类型
- [ ] 版本号类型
- [ ] 标签或名称类型
- [ ] 单元测试

### `crates/platform`

- [ ] `TaskSpawner`
- [ ] `SleepProvider`
- [ ] `FileSystem`
- [ ] `KvStore`（可先占位）
- [ ] `NetworkClient`（可先占位）
- [ ] fake/mock trait 实现接口预留

### `crates/platform-std`

- [ ] 默认 logger adapter
- [ ] 默认 clock adapter
- [ ] 默认 spawner
- [ ] 默认 sleep provider
- [ ] basic file system adapter

### `crates/model`

- [ ] `Config`
- [ ] `State`
- [ ] `Event`
- [ ] 对外 DTO
- [ ] serde 支持（按需）

### `crates/core`

- [ ] 生命周期状态机
- [ ] 命令定义
- [ ] 状态迁移逻辑
- [ ] 事件生成规则
- [ ] 单元测试

### `crates/runtime`

- [ ] runtime context
- [ ] event dispatcher
- [ ] shutdown token
- [ ] worker / task 管理骨架
- [ ] 集成测试

### `crates/sdk`

- [ ] `Builder`
- [ ] `Handle`
- [ ] `init/start/stop/destroy`
- [ ] `get_state`
- [ ] `subscribe` 或回调式事件入口
- [ ] 对 `model` 的 re-export

### `examples/rust-basic`

- [ ] build 一个最小 config
- [ ] 创建 SDK
- [ ] start
- [ ] 订阅或打印事件
- [ ] stop
- [ ] destroy

---

## D. 根 `Cargo.toml` 验收

- [ ] `members` 已登记所有首批 crate
- [ ] `workspace.package` 已声明 edition / license / rust-version
- [ ] `workspace.dependencies` 已抽出公共依赖
- [ ] `workspace.lints` 已配置
- [ ] profile 已按需设置

建议首批统一依赖：

- [ ] `thiserror`
- [ ] `tracing`
- [ ] `serde`
- [ ] `serde_json`
- [ ] `tokio`（若首版选择 tokio runtime）

---

## E. 依赖方向检查

必须验证：

- [ ] `base/*` 未依赖 `crates/*`
- [ ] `platform` 未依赖 `core/runtime/sdk`
- [ ] `model` 未依赖 `runtime/sdk/bindings`
- [ ] `core` 未依赖 `bindings/*`
- [ ] `runtime` 未依赖 `bindings/*`
- [ ] `sdk` 未依赖具体平台实现 crate

建议在代码评审时专门检查一次依赖边界。

---

## F. 生命周期首版验收

至少明确以下状态：

- [ ] `Created`
- [ ] `Initializing`
- [ ] `Initialized`
- [ ] `Starting`
- [ ] `Running`
- [ ] `Stopping`
- [ ] `Stopped`
- [ ] `Destroying`
- [ ] `Destroyed`

至少明确以下接口：

- [ ] `init`
- [ ] `start`
- [ ] `stop`
- [ ] `destroy`
- [ ] `get_state`

至少明确以下规则：

- [ ] 幂等规则
- [ ] 非法状态迁移错误
- [ ] stop 是否等待后台任务退出
- [ ] destroy 后句柄是否还可查询状态

---

## G. 错误模型首版验收

- [ ] 有统一 `Result<T>`
- [ ] 有统一错误码结构
- [ ] 有统一错误分类
- [ ] 有上下文附加能力
- [ ] `sdk` 对外不直接裸露底层随机错误

建议先固定错误类别：

- [ ] 参数错误
- [ ] 状态错误
- [ ] 配置错误
- [ ] 运行时错误
- [ ] 外部依赖错误
- [ ] 内部错误

---

## H. 测试首版验收

- [ ] `cargo fmt --all` 通过
- [ ] `cargo clippy --workspace --all-targets` 通过
- [ ] `cargo test --workspace` 通过
- [ ] 至少 1 个 `sdk` 生命周期集成测试
- [ ] 至少 1 个 fake platform 测试
- [ ] 至少 1 个 example 可运行

---

## I. 第二阶段预留

以下内容不必首版一次做完，但目录和设计上应留好位置：

- [ ] `crates/ffi-common`
- [ ] `bindings/c`
- [ ] `bindings/uniffi`
- [ ] `tests/ffi-smoke`
- [ ] `xtask`
- [ ] `docs/ERRORS.md`
- [ ] `docs/API.md`
- [ ] `docs/RELEASE.md`

---

## J. 第三阶段预留

- [ ] `bindings/android`
- [ ] `bindings/ios`
- [ ] `bindings/ohos`
- [ ] Android 产物整理脚本
- [ ] iOS xcframework 产物整理脚本
- [ ] OHOS 打包脚本
- [ ] CI 多平台流水线
- [ ] ABI version 约定
- [ ] FFI smoke test

---

## K. 一票否决项

若出现以下情况，建议暂停继续堆功能，先修架构：

- [ ] `core` 中已出现 Android/JNI/OHOS/iOS 平台调用
- [ ] `bindings` 中开始写业务状态机
- [ ] `base/types` 中堆入大量领域类型
- [ ] `utils` 已变成杂物箱
- [ ] `sdk` 已经暴露太多内部 crate 细节
- [ ] 生命周期仍未固定就开始做 FFI
- [ ] 回调线程规则仍未定义就开始对接 App

---

## L. 建议的首版完成标准

当满足以下条件时，可以认为 RustTemplate 第一版骨架可用：

- [ ] workspace 结构稳定
- [ ] 主链路 crate 已建立
- [ ] `sdk` Rust API 可运行
- [ ] 生命周期完整
- [ ] 错误模型完整到可用程度
- [ ] 默认平台实现可运行
- [ ] 示例可运行
- [ ] 集成测试可运行
- [ ] 文档至少有 `ARCHITECTURE.md` 与 `WORKSPACE.md`

到这里，就可以进入第二阶段：FFI 与 bindings。
