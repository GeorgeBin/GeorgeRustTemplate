# RustTemplate 架构设计（V2）

## 1. 文档定位

本文档定义一个 **面向长期演进的 Rust 跨平台库模板工程**，其核心目标不是服务某一个具体业务，而是作为一套可复用的工程骨架，承载后续不同领域 SDK。

该模板需要同时满足两类使用场景：

1. **Rust 原生调用**
   - 作为普通 crate 被其他 Rust 工程依赖
   - 保留 Rust 风格 API：类型安全、`Result`、`async`、builder、stream/subscription 等

2. **非 Rust 平台调用**
   - Android（Kotlin / Java）
   - iOS（Swift / Objective-C）
   - OpenHarmony（ArkTS / NAPI / FFI）
   - C / C++
   - 其他语言绑定

因此，RustTemplate 的本质不是“一个库”，而是：

- 一套 **分层清晰** 的 workspace 结构
- 一套 **平台无关核心** 的实现方法
- 一套 **可导出 ABI / 绑定** 的边界规则
- 一套 **可裁剪、可测试、可发布** 的工程约束

---

## 2. 总体设计目标

### 2.1 目标

本模板应满足以下目标：

- 核心逻辑使用 Rust 实现，且尽可能平台无关
- 对 Rust 使用者暴露自然、简洁、可组合的 API
- 对非 Rust 使用者暴露稳定、简单、受控的导出接口
- 能通过 feature 对能力进行裁剪
- 能进行 ABI 演进与版本管理
- 能覆盖单元测试、集成测试、绑定层烟囱测试
- 能支持多平台发布流水线与产物管理

### 2.2 非目标

为了避免模板失控，明确以下内容 **不是** 第一阶段目标：

- 不是 `no_std` 模板
- 不是 UI 模板
- 不是具体业务协议模板
- 不是“一开始就拆成二十多个 crate”的极端微拆分模板
- 不是把 Android/iOS/OHOS 平台代码都放进核心层的混合工程

换言之：**先做长期可维护的 SDK 骨架，再承载业务能力。**

---

## 3. 核心设计原则

你之前的架构已经明确了三件最重要的事：

- 核心逻辑平台无关
- Rust API 与跨平台导出 API 分离
- 生命周期、错误模型、依赖方向要先定义清楚

在此基础上，V2 版本进一步收紧为以下原则。

### 原则 1：Core 只关心“做什么”，不关心“在哪做”

`core` 不允许直接依赖：

- Android API / JNI
- Apple Foundation / Objective-C runtime
- OpenHarmony NAPI runtime
- UI 框架
- 平台路径规则
- 平台线程/Looper/Handler 细节

`core` 只关心：

- 数据模型
- 生命周期状态机
- 命令与事件
- 业务编排规则
- 错误语义
- 能力抽象接口

### 原则 2：Runtime 负责“如何跑起来”

`core` 与 `runtime` 必须分工明确：

- `core`：纯规则、纯状态、纯编排
- `runtime`：异步任务、调度、后台 worker、事件分发、监督机制

这是 V2 最重要的增强之一。很多 Rust SDK 工程早期把这两者混在一起，结果后期：

- 业务状态机里到处是 tokio 细节
- FFI 层很难解释线程模型
- Android/iOS 回调线程不可控
- 单测难写，假实现难打

### 原则 3：Bindings 只能做翻译，不能做决策

`bindings/*` 的职责只有四个：

- 类型映射
- 生命周期映射
- 异步模型映射
- 错误模型映射

Bindings 层 **不能承载核心业务判断**，也不能形成第二套状态机。

### 原则 4：Public Model 与 Base Type 必须分离

`base/types` 只放跨项目可复用的小基础类型，不放某个 SDK 的领域配置、领域事件、领域 DTO。

领域公共模型应进入独立的 `model`（或 `api-model`）crate，以避免：

- 具体业务类型污染基础库
- 后续多个 SDK 共用模板时，`base/types` 变成杂物箱
- bindings/sdks/core 共享类型时职责混乱

### 原则 5：所有导出 API 必须先经过“FFI 安全性检查”

凡是跨 ABI 边界的类型，必须满足：

- 不暴露借用语义
- 不暴露泛型
- 不暴露 trait object
- 不暴露复杂生命周期
- 不要求对端理解 Rust 所有权模型

跨平台导出优先使用：

- 句柄
- 简单配置对象
- 字符串 / 字节数组 / 数字 / 布尔值
- 受控回调接口
- 明确错误码

### 原则 6：依赖必须单向，且“上层知道下层，下层不知道上层”

你原文中已经强调了单向依赖与平台无关核心，这一点必须继续保持并写入工程约束中。

---

## 4. 推荐的目标架构

V2 推荐使用 **六层结构**，比你原来的五层更精确：

1. **Base 基础设施层**
2. **Platform Contract 层**
3. **Domain/Public Model 层**
4. **Core 规则与状态层**
5. **Runtime 执行与调度层**
6. **Facade / Bindings 对外层**

关系如下：

```text
Bindings / Export
        ↑
Rust Facade (sdk)
        ↑
Runtime
        ↑
Core
        ↑
Model + Platform Contracts
        ↑
Base
```

这里最关键的变化是：

- 将 **领域公共模型** 从 `base/types` 中剥离
- 将 **runtime** 从“可选小补丁”提升为“独立且明确的层”
- 将 **platform** 明确为“契约层”，而不是“平台实现混合层”

---

## 5. 推荐 Workspace 目录

## 5.1 标准版目录

```text
rust-template/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── LICENSE
├── .cargo/
│   └── config.toml
├── xtask/
│   └── src/
├── base/
│   ├── error/
│   ├── log/
│   ├── time/
│   ├── types/
│   └── utils/
├── crates/
│   ├── platform/
│   ├── platform-std/
│   ├── model/
│   ├── core/
│   ├── runtime/
│   ├── sdk/
│   └── ffi-common/
├── bindings/
│   ├── uniffi/
│   ├── c/
│   ├── android/
│   ├── ios/
│   └── ohos/
├── examples/
│   ├── rust-basic/
│   └── rust-advanced/
├── tests/
│   ├── integration/
│   └── ffi-smoke/
├── docs/
│   ├── ARCHITECTURE.md
│   ├── WORKSPACE.md
│   ├── ERRORS.md
│   ├── API.md
│   └── RELEASE.md
└── .github/
    └── workflows/
```

## 5.2 为什么增加 `xtask/`

建议加入 `xtask` crate，而不是主要依赖 shell 脚本。原因：

- 统一封装构建、代码生成、打包、发布任务
- 便于在 macOS / Linux / Windows 下复用
- 适合封装 `uniffi-bindgen`、Android 产物复制、OHOS 构建、示例运行、版本号写入等流程

典型命令：

- `cargo xtask codegen`
- `cargo xtask build-android`
- `cargo xtask build-ohos`
- `cargo xtask release --target ios`

---

## 6. 各 crate 职责定义

## 6.1 `base/error`

职责：

- 统一 `Result<T>`
- 最基础错误类型与错误元信息
- 错误分类抽象
- 错误码基础设施
- 错误链/上下文附加能力

约束：

- 不承载具体 SDK 的领域错误枚举
- 不依赖 bindings
- 不耦合 JNI/NAPI/UniFFI

建议内容：

- `ErrorCode`
- `ErrorCategory`
- `ErrorContext`
- `BaseError`
- `Result<T>`

## 6.2 `base/log`

职责：

- 日志等级
- 日志记录结构
- Logger trait
- 日志门面接口

约束：

- 不直接依赖 Android Logcat / iOS NSLog / OHOS HiLog
- 平台日志实现应放入 `platform-std` 或 bindings 侧桥接

## 6.3 `base/time`

职责：

- `Clock` 抽象
- 时间戳与持续时间工具
- 测试友好时间源

建议：

- 区分 `WallClock` 与 `MonotonicClock`
- 避免业务逻辑直接依赖系统时间函数

## 6.4 `base/types`

职责：

- 与具体 SDK 无关的小型基础类型
- ID、名称、标签、分页、版本号等通用模型

约束：

- 不存放具体业务配置、事件或会话状态这类领域类型

## 6.5 `base/utils`

职责：

- 轻量工具函数
- IP / 字符串 / 字节等通用校验辅助
- 序列化辅助
- 字节/字符串帮助函数

约束：

- 只允许非常克制地放真正通用的东西
- 禁止成为“放不下就丢进 utils”的垃圾桶

---

## 6.6 `crates/platform`

这是 **平台契约层**，不是平台实现层。

职责：

- 定义核心层依赖的能力接口（ports）
- 形成核心层访问外部世界的边界

建议包含的 trait：

- `Logger`
- `Clock`
- `TaskSpawner`
- `SleepProvider`
- `FileSystem`
- `KvStore`
- `SecureStore`
- `NetworkClient`
- `ConnectivityProvider`
- `RandomProvider`

约束：

- 只定义接口与必要的小型适配类型
- 不直接写 Android/iOS/OHOS 的具体实现

---

## 6.7 `crates/platform-std`

这是建议新增的 crate。

职责：

- 为桌面 / CLI / 测试环境提供一套默认的 `std` 实现
- 封装 tracing、tokio fs、reqwest、文件日志等偏通用实现

这样做的好处：

- `core` 不会因为方便而偷偷依赖具体库
- 桌面 Demo、examples、测试可以直接使用默认实现
- 移动平台若有特殊实现，可单独替换

---

## 6.8 `crates/model`

这是建议新增的 **领域公共模型层**。

职责：

- 领域配置模型
- 领域事件模型
- 领域 DTO
- 领域状态枚举
- 对外稳定的公共类型

例如对于 PTT：

- `PttConfig`
- `TalkState`
- `GroupInfo`
- `MemberInfo`
- `PttEvent`

为什么不继续用 `config/` 与 `event/` 分散拆分：

- 对模板工程而言，第一阶段过细拆分收益不大
- `model` 更适合作为 bindings/sdk/core 共享的公共边界
- 后期若领域膨胀，再从 `model` 中拆出 `config` 与 `event`

---

## 6.9 `crates/core`

这是 **规则与状态机层**，是整个 SDK 的业务大脑。

职责：

- 生命周期状态机
- 领域状态机
- 命令校验
- 事件生成规则
- 服务装配规则
- 业务流程编排

应当具备的特征：

- 尽量“同步优先”建模
- 尽量不直接依赖具体异步运行时
- 易于单元测试
- 易于使用 fake/mock platform 实现

推荐内部结构：

```text
crates/core/src/
├── lifecycle/
├── state/
├── command/
├── event/
├── service/
├── assembler/
└── lib.rs
```

注意：

- `core` 可以依赖 `model` 与 `platform`
- `core` 不应该依赖 `bindings/*`
- `core` 不应该直接依赖 `android`、`ohos`、`objc` 相关库

---

## 6.10 `crates/runtime`

这是 **执行与调度层**。

职责：

- actor / worker 生命周期
- channel 调度
- 后台任务管理
- supervisor / restart 策略
- 事件总线实现
- runtime shutdown 协调
- callback dispatch 桥接前的统一分发

建议内容：

- `RuntimeContext`
- `RuntimeHandle`
- `WorkerRegistry`
- `Supervisor`
- `EventDispatcher`
- `ShutdownToken`

明确边界：

- `core` 负责“事件该不该发、状态该怎么变”
- `runtime` 负责“在哪个线程/任务里发、怎么停、怎么收尾”

这一步会极大改善后续 Android / iOS / OHOS 的接入体验。

---

## 6.11 `crates/sdk`

这是 Rust 用户唯一应该优先依赖的门面层。

职责：

- 暴露 `Builder`
- 暴露 `Handle`
- 暴露 `start/stop/destroy`
- 暴露事件订阅接口
- re-export 公共模型
- 屏蔽内部 crate 的复杂性

建议 API 风格：

```rust
let sdk = MySdk::builder()
    .with_platform(platform)
    .with_config(config)
    .build()?;

sdk.start().await?;
let mut events = sdk.subscribe();
```

约束：

- Rust 用户原则上不需要直接依赖 `core`/`runtime`
- `sdk` 是唯一稳定承诺的 Rust 外观层

---

## 6.12 `crates/ffi-common`

职责：

- FFI 句柄基础设施
- FFI 安全字符串/字节工具
- 回调注册与取消注册辅助
- 错误码映射公共逻辑
- ABI 版本信息

这是跨平台绑定层的重要缓冲区。

建议内容：

- `HandleId`
- `OpaqueHandle`
- `FfiError`
- `FfiResultCode`
- `CallbackSlot`
- `AbiVersion`

---

## 6.13 `bindings/*`

每个 bindings crate 对应一种导出策略，而不是一个“平台业务实现”。

### `bindings/uniffi`

适合：

- Kotlin / Swift 的中高层绑定
- 追求开发效率的场景

### `bindings/c`

适合：

- 稳定 ABI
- C/C++ 接入
- 其他语言二次桥接
- 长期导出边界的保守方案

### `bindings/android`

适合：

- Android 特定打包、JNI 辅助、AAR 产物整理
- Kotlin/Java 对接层的整理

### `bindings/ios`

适合：

- XCFramework 组装
- Swift 侧包装辅助

### `bindings/ohos`

适合：

- NAPI/ArkTS 导出
- OHOS 特定打包脚本与桥接逻辑

共同约束：

- 不写核心业务逻辑
- 不写第二套状态机
- 不在 bindings 中决定业务规则

---

## 7. 推荐的依赖方向

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

更精确地说：

- `base/*` 不依赖 `crates/*`、`bindings/*`
- `platform` 依赖 `base/*`
- `platform-std` 依赖 `platform` 与 `base/*`
- `model` 依赖 `base/*`
- `core` 依赖 `model`、`platform`、`base/*`
- `runtime` 依赖 `core`、`platform`、`model`、`base/*`
- `sdk` 依赖 `runtime`、`core`、`model`、`platform`
- `ffi-common` 依赖 `sdk` 或最少量公共类型（视实现而定）
- `bindings/*` 依赖 `sdk`、`ffi-common` 以及各自需要的导出工具链

### 7.1 一个重要约束

`ffi-common` 是否依赖 `sdk`，取决于你想让它多“薄”。

- 若 `ffi-common` 只做纯工具：不要依赖 `sdk`
- 若 `ffi-common` 还承担统一句柄仓库与生命周期桥接：可有限依赖 `sdk`

模板初期建议：

- `ffi-common` 尽量薄
- 以减少未来 `bindings/*` 的耦合半径

---

## 8. 生命周期设计

你原文已经提出 `Created -> Initialized -> Starting -> Running -> Stopping -> Stopped -> Destroyed` 这条主线，这个方向是对的。

V2 建议做以下增强。

## 8.1 生命周期状态

推荐完整状态：

- `Created`
- `Initializing`
- `Initialized`
- `Starting`
- `Running`
- `Stopping`
- `Stopped`
- `Destroying`
- `Destroyed`
- `Faulted`（可选）

说明：

- `Initializing` / `Destroying` 作为瞬时态有利于并发保护
- `Faulted` 适合长连接、流式、硬件接入类 SDK

## 8.2 对外统一 API

对外统一暴露：

- `init`
- `start`
- `stop`
- `destroy`
- `update_config`（可选）
- `reset`（谨慎使用）
- `get_state`

## 8.3 生命周期规则

必须明确写进文档：

- 所有状态迁移是否幂等
- 非法迁移返回什么错误码
- `destroy` 后句柄是否还能查询状态
- `stop` 是否阻塞等待后台任务退出
- 回调在 `stop/destroy` 过程中是否仍可能收到尾部事件

没有这些规则，bindings 很快就会出现平台间行为不一致问题。

---

## 9. 并发与线程模型

这是原文里尚未充分展开，但对跨平台库至关重要的一章。

## 9.1 内部并发原则

建议采用：

- 外部 API：句柄式
- 内部协调：actor / command queue / channel
- 状态修改：单写多读或串行化更新
- 停止流程：统一 shutdown token

## 9.2 回调线程规则必须明确

对外必须说明：

- 回调在哪个线程触发
- 是否保证串行
- 是否可能重入
- 是否可能晚于 `stop()` 返回
- Android / iOS / OHOS 是否需要切回 UI 线程由上层自行处理

推荐默认策略：

- SDK 仅保证回调来自“SDK 管理线程/任务”
- 不承诺在 UI 主线程
- UI 线程切换由上层 App 负责

这样边界最清晰。

---

## 10. 导出层与 ABI 设计

## 10.1 导出设计原则

跨平台导出接口统一采用：

- `create(config) -> handle`
- `start(handle)`
- `stop(handle)`
- `destroy(handle)`
- `register_callback(handle, callback)`
- `unregister_callback(handle)`
- `get_state(handle)`

## 10.2 句柄设计

推荐：

- 对外暴露不透明句柄
- 内部用句柄仓库映射到真实对象
- 禁止跨 ABI 传递 Rust 引用

典型 C ABI 形态：

```c
uint64_t sdk_create(const sdk_config_t* config);
int32_t sdk_start(uint64_t handle);
int32_t sdk_stop(uint64_t handle);
int32_t sdk_destroy(uint64_t handle);
```

## 10.3 类型设计约束

跨 ABI 不应直接暴露：

- `Vec<T>`（除非桥接层转换）
- `HashMap<K, V>`
- `Option<T>` 的裸形态
- 任意复杂 enum
- 闭包
- trait object

需要通过桥接层转换为：

- 基础数值类型
- 简单结构体
- UTF-8 字符串
- 字节数组
- 扁平化 DTO

## 10.4 ABI 版本

强烈建议加入：

- `sdk_get_abi_version()`
- `sdk_get_version_string()`

并约定：

- 语义版本：面向 Rust / 发行包
- ABI 版本：面向 bindings / 动态库兼容性

---

## 11. 错误模型设计

你原文已明确“内部错误可以详细，对外错误必须稳定，错误码优先于自由文本”，这是完全正确的方向。

V2 建议分为四层。

## 11.1 Base 错误

位于 `base/error`：

- `BaseError`
- `ErrorCode`
- `Result<T>`
- 通用上下文能力

## 11.2 Domain 错误

位于 `core` 或独立 `{{domain}}-error`：

- 生命周期错误
- 配置错误
- 参数错误
- 状态冲突错误
- 外部能力不可用错误

## 11.3 Runtime 错误

位于 `runtime`：

- 任务启动失败
- worker 崩溃
- channel 关闭
- shutdown 超时

## 11.4 FFI / Binding 映射错误

位于 `ffi-common` / `bindings/*`：

- 句柄无效
- 回调未注册
- 参数空指针
- UTF-8 转换失败
- 平台桥接失败

### 一条重要原则

**Native 错误不等于内部错误。**

例如：

- Android/JNI 异常
- HTTP 状态码
- socket errno
- OpenHarmony NAPI 错误

这些都应该作为“来源信息”被吸收进统一错误体系，而不是直接成为 SDK 的对外语义中心。

---

## 12. Feature Flag 设计

建议按四类拆分 feature。

## 12.1 基础能力 feature

```toml
[features]
default = ["std", "runtime-tokio"]
std = []
runtime-tokio = []
runtime-smol = []
tracing-log = []
file-log = []
```

## 12.2 能力 feature

```toml
net = []
http = []
crypto = []
kv = []
secure-store = []
telemetry = []
```

## 12.3 导出 feature

```toml
ffi = []
uniffi = []
c-abi = []
android-binding = []
ios-binding = []
ohos-binding = []
```

## 12.4 测试 / 调试 feature

```toml
test-util = []
mock-platform = []
internal-metrics = []
```

### Feature 原则

- feature 应表达“能力”，不要表达“目录存在”
- 尽量避免隐式开启过多重量依赖
- bindings 相关 feature 不应反向污染 core

---

## 13. 命名规范建议

你当前的命名策略总体合理：

- `base/*` 使用 `george-base-*`
- 具体 SDK 使用 `xxx-*`
- bindings 使用 `xxx-binding-*`

V2 仅补充以下建议。

## 13.1 模板中的占位符命名

建议在模板文档中统一使用：

- `{{org}}-base-*`
- `{{domain}}-*`
- `{{domain}}-binding-*`

例如：

- `george-base-error`
- `ptt-runtime`
- `rtc-binding-uniffi`

## 13.2 crate 名建议

- 面向 Rust 用户的主入口尽量叫 `{{domain}}`，而不是 `{{domain}}-sdk`
- 内部层再使用 `{{domain}}-core`、`{{domain}}-runtime`

例如：

- `ptt`：给 Rust 用户使用
- `ptt-core`：内部核心
- `ptt-runtime`：内部调度

这样对 Rust 用户最自然。

---

## 14. 发布与产物设计

## 14.1 产物类型

模板应支持以下产物：

- Rust crate（`rlib`）
- 动态库（`cdylib`）
- 静态库（`staticlib`，按需）
- Android `.so`
- iOS framework / xcframework
- OHOS `.so` / NAPI 产物
- 语言绑定源码（UniFFI 生成的 Kotlin/Swift 等）

## 14.2 发布边界

建议区分：

- `sdk` crate 的 Rust API 版本
- `bindings/*` 的导出包版本
- ABI 版本

并在 `docs/RELEASE.md` 中固定规则。

---

## 15. 测试策略

跨平台模板不能只写 `cargo test`。

## 15.1 测试层次

### 单元测试

覆盖：

- `base/*`
- `model`
- `core`
- `runtime` 的核心逻辑

### 集成测试

覆盖：

- `sdk` 层完整生命周期
- fake platform 下的端到端行为

### FFI 烟囱测试

覆盖：

- 创建/销毁句柄
- 错误码映射
- 回调注册/触发/注销

### 平台绑定测试

覆盖：

- Android JNI/UniFFI smoke test
- iOS 调用 smoke test
- OHOS NAPI smoke test

## 15.2 可测试性要求

模板中的每一层都要支持替换依赖：

- fake logger
- fake clock
- fake network
- fake store
- fake spawner

否则核心层单测会越来越难写。

---

## 16. 三阶段落地方案

## 第一阶段：极简可运行骨架

建立：

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

目标：

- Rust 原生可跑通
- 生命周期完整
- 事件订阅可跑通
- 单测框架建立

## 第二阶段：导出层成型

建立：

- `crates/ffi-common`
- `bindings/c` 或 `bindings/uniffi`

目标：

- 至少一种跨语言导出方式稳定可用
- 错误码/句柄/回调模型固定

## 第三阶段：平台产物完善

建立：

- `bindings/android`
- `bindings/ios`
- `bindings/ohos`
- `xtask` 打包任务
- CI 多目标流水线

目标：

- 形成真正可交付的跨平台模板工程

---

## 17. 不推荐的做法

以下做法应明确禁止：

- 在 `core` 中直接写 `cfg(target_os = "android")`
- 在 `bindings/*` 中编写领域状态机
- 把领域类型塞进 `base/types`
- 把所有“不知道放哪”的东西都塞进 `utils` 或 `common`
- 让 `sdk` 直接暴露过多内部 crate 细节
- 不定义回调线程规则
- 不定义句柄生命周期就急于做 FFI
- 一开始拆出过多粒度极小的 crate

---

## 18. 最终建议

结合你当前已有资料，我给出的最终建议是：

### 应保留的部分

- `base + crates + bindings` 的总分层思路
- 平台无关核心
- Rust API 与导出 API 分离
- 统一生命周期
- 统一错误模型
- 单向依赖

这些在你现有 ARCHITECTURE.md 中已经是正确方向。

### 应增强的部分

- 明确把 `runtime` 从 `core` 中剥离出来
- 增加 `model` 作为公共领域模型层
- 增加 `platform-std` 作为默认实现层
- 增加 `xtask` 作为构建/打包编排工具
- 把 FFI 句柄、线程、回调、ABI 版本规则写成硬约束
- 在模板层面预埋测试矩阵与发布规则

### 最适合作为 RustTemplate 第一版的目录

```text
rust-template/
├── base/
│   ├── error/
│   ├── log/
│   ├── time/
│   ├── types/
│   └── utils/
├── crates/
│   ├── platform/
│   ├── platform-std/
│   ├── model/
│   ├── core/
│   ├── runtime/
│   ├── sdk/
│   └── ffi-common/
├── bindings/
│   ├── uniffi/
│   ├── c/
│   ├── android/
│   ├── ios/
│   └── ohos/
├── xtask/
├── examples/
├── tests/
└── docs/
```

这是我认为在“不过度设计”和“足够长期演进”之间，最平衡的一版。
