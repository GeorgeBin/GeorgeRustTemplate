# RustTemplate 架构设计基线（当前工程版）

## 1. 文档目的

本文档用于定义 **GeorgeRustTemplate** 当前阶段应当遵循的完整架构基线。

它不是某一个业务功能的设计稿，也不是某一个 demo 的说明文档，而是：

- 当前 workspace 的统一分层说明
- 各 crate 的职责边界说明
- 依赖方向与约束说明
- 当前阶段允许做什么、不允许做什么的规则说明
- 后续让 Codex 继续修正工程时的统一依据

本模板的目标不是“先做一个演示功能，再慢慢抽架构”，而是：

> 先把模板工程本身做对，再在模板之上添加具体功能或 demo。

因此，模板工程中的 `base/*`、`crates/*`、`bindings/*` 应当始终优先服务于“长期可复用的 Rust 跨平台库骨架”，而不是服务于某个临时演示能力。

---

## 2. 当前阶段的核心判断

当前工程已经完成了以下方向上的收敛：

- `base/error`：统一错误协议层
- `base/log`：日志协议层
- `base/types`：基础值对象层
- `crates/model`：产品/领域公开模型层（当前几乎为空）
- `crates/core`：核心编排层（当前几乎为空）
- `crates/runtime`：通用运行时承托层
- `crates/sdk`：Rust 对外门面层的最小壳
- `crates/platform-std`：std 环境下的默认具体后端（当前主要承接 logging）

同时，之前用于演示的 NTP 功能已从模板骨架主线中移除。

这意味着当前模板的主线不再是“围绕 NTP 的跨平台库”，而是：

> 一个可承载未来 `ptt`、`rtc`、`media`、`device`、`ntp` 等不同能力域的 Rust 跨平台库模板工程。

这是当前最重要的架构前提。

---

## 3. 总体目标

### 3.1 目标

本模板工程应同时服务两类调用方：

#### Rust 调用方

- 作为普通 Rust crate 依赖
- 获得自然、稳定、可组合的 Rust API
- 通过 `Builder`、`Context`、`Handle`、`Result` 等方式使用能力

#### 非 Rust 调用方

- Android（Kotlin / Java）
- iOS（Swift / Objective-C）
- OpenHarmony（ArkTS / NAPI / FFI）
- C / C++
- 其他语言绑定

因此本模板必须同时具备：

- 平台无关的核心层
- 受控的运行时承托层
- 清晰的 Rust façade
- 可继续扩展的 bindings 边界

### 3.2 非目标

在当前阶段，以下内容不是模板骨架的目标：

- 不是某个具体业务协议的完整 SDK
- 不是“先写出能跑的 demo，再反向整理架构”
- 不是 UI 模板
- 不是一开始就做成庞大的多能力全集
- 不是把平台代码、运行时代码、业务规则写在一起
- 不是把 demo 功能直接塞进 `core` / `runtime` / `sdk` 主线

---

## 4. 核心设计原则

### 原则 1：模板优先，demo 次之

任何演示功能都不得反向塑造模板骨架。

正确顺序应为：

1. 先定义模板层次
2. 先定义基础协议
3. 先定义依赖方向
4. 再在模板之上添加具体功能示例

### 原则 2：平台无关核心

`core` 不允许直接依赖：

- `std::net::*`
- 平台 API
- JNI / NAPI / Objective-C runtime
- UI 框架
- subscriber 安装
- 平台线程模型

`core` 只允许承载：

- 核心规则
- 输入校验
- 状态迁移
- 纯解析逻辑
- 核心语义错误
- 面向外部世界的端口抽象

### 原则 3：运行时与核心分离

`runtime` 与 `core` 必须分层：

- `core` 回答“应该做什么”
- `runtime` 回答“如何在具体执行环境中做”

因此：

- `core` 不直接管理线程、socket、executor
- `runtime` 不定义业务规则

### 原则 4：基础协议与具体后端分离

`base/log` 已经证明这条原则是正确的：

- `base/log` 只做协议
- `platform-std` 承接具体 tracing/std 后端

同样的思路未来也适用于：

- 时间源
- 文件系统
- 网络客户端
- 安全存储
- 任务调度器

### 原则 5：值对象、模型、核心规则三层分离

- `base/types`：小型通用值对象
- `crates/model`：产品/领域公开模型
- `crates/core`：规则、编排、端口

禁止把三者揉成一层。

### 原则 6：门面层必须稳定、瘦、对外友好

`sdk` 是 Rust 用户的首选入口。

它的职责是：

- 收口内部复杂度
- 统一入口形态
- re-export 稳定公开类型

它不应该变成：

- runtime 实现层
- bindings 适配层
- 平台专有逻辑层

### 原则 7：依赖必须单向

所有 crate 必须遵循单向依赖，禁止下层反向知道上层。

---

## 5. 推荐的总体分层

当前模板工程应采用如下分层：

```text
bindings / external bridges
        ↑
crates/sdk
        ↑
crates/runtime
        ↑
crates/core
        ↑
crates/model
        ↑
base/*
```

同时，`crates/platform-std` 作为一个特殊层存在：

- 它不是 `base/*`
- 它也不是 `bindings/*`
- 它是“标准平台具体实现集合”
- 它可以为 `sdk`、应用壳、demo、tests 提供默认具体实现

更准确地说：

```text
                   crates/platform-std
                         ↑
bindings / apps ───────→ sdk
                         ↑
                      runtime
                         ↑
                        core
                         ↑
                       model
                         ↑
                      base/*
```

---

## 6. Workspace 当前推荐结构

```text
GeorgeRustTemplate/
├── Cargo.toml
├── README.md
├── base/
│   ├── error/
│   ├── log/
│   ├── types/
│   └── utils/
├── crates/
│   ├── core/
│   ├── model/
│   ├── platform-std/
│   ├── runtime/
│   └── sdk/
├── examples/
│   └── rust-demo/
└── xtask/
```

后续可以继续扩展：

```text
crates/
├── ffi-common/
├── platform/
└── <domain>/...

bindings/
├── c/
├── uniffi/
├── android/
├── ios/
└── ohos/
```

但当前阶段不应为了“未来可能会用”而一次性建太多空 crate。

---

## 7. 各层职责定义

## 7.1 `base/error`

定位：**统一错误协议层**。

负责：

- 统一错误码机制
- 错误语义描述
- 运行时错误实例
- 错误上下文与 native 信息

不负责：

- FFI DTO
- 平台桥接错误对象
- 某个具体业务域的完整错误全集

原则：

- 错误码表达稳定语义
- native 保留原始现场
- 统一协议优先于随意拼接文本

## 7.2 `base/log`

定位：**日志协议层**。

负责：

- `LogLevel`
- `LogField`
- `LogRecord`
- `Logger` trait
- `LoggerExt`
- `SharedLogger`
- `NoopLogger`

不负责：

- tracing subscriber 安装
- 文件滚动
- 日志清理
- 平台日志后端

## 7.3 `base/types`

定位：**基础值对象层**。

负责：

- 跨多个 crate 复用的小型值对象
- 裸标量的轻量语义包装
- 带最小必要校验的稳定 newtype

当前适合放：

- `HandleId`
- `RequestId`
- `CorrelationId`
- `NonEmptyString`

不负责：

- 领域 request / response
- runtime context
- 平台 DTO
- 复杂业务模型

## 7.4 `base/utils`

定位：**轻量通用工具层**。

要求：

- 只放真正通用且边界清晰的小工具
- 严禁成为“放不下就丢进去”的垃圾桶

当前阶段应保持克制。

---

## 7.5 `crates/model`

定位：**产品/领域公开模型层**。

负责：

- 稳定的 request / response / params / options / state / event 模型
- 供 `core`、`runtime`、`sdk`、`bindings` 共同理解的公共模型

不负责：

- 行为逻辑
- 运行逻辑
- 线程模型
- FFI DTO
- 平台类型

当前阶段建议：

- 保持极小
- 没有真实跨模块复用的模型时，不要预埋大量空模块
- 空的 `common/` 最多只能短暂存在，不应长期作为“未来再说”的占位目录

## 7.6 `crates/core`

定位：**核心编排层**。

负责：

- 规则
- 规则输入校验
- 状态变换规则
- 纯解析逻辑
- 核心语义错误
- 面向外部依赖的端口抽象

不负责：

- 真实 I/O
- 真实时间源
- 真实 executor
- subscriber 安装
- FFI
- UI

当前阶段建议：

- 宁可保持极小，也不要把 demo-specific core 放进来
- 当没有真实通用 use case 时，可以先只保留最小占位骨架
- 但不应长期停留在“只有一句注释”的状态；一旦确定首个通用能力域，应优先在这里长出真正的端口与规则

## 7.7 `crates/runtime`

定位：**运行时承托层**。

负责：

- runtime-wide 共享资源的最小上下文
- 未来 `core` 端口的具体实现归属地
- 对具体执行环境的承托

当前阶段仅保留：

- `RuntimeContext`
- `RuntimeBuilder`
- `SharedLogger` 注入

不负责：

- service locator
- 全局对象仓库
- transport 注册表
- 任务总线
- shutdown 系统
- demo-specific adapter

当前 `RuntimeContext` 必须保持小、稳、显式。

## 7.8 `crates/platform-std`

定位：**std 环境下的默认具体实现集合**。

负责：

- tracing/std logging backend
- 未来可继续承接其他 std 默认实现

当前已经合理承接：

- tracing subscriber 安装
- 文件输出
- 日志切割
- 日志清理
- `TracingForwardLogger`

不负责：

- 业务模型
- 业务规则
- runtime 上下文本体

## 7.9 `crates/sdk`

定位：**Rust 对外门面层**。

负责：

- 统一 Rust 使用入口
- re-export 稳定公开能力
- 隐藏内部层级复杂度

当前阶段由于模板仍在打骨架，`sdk` 可以只做极简 re-export；但后续目标应是：

- 优先让 Rust 用户依赖 `sdk`
- 而不是直接依赖 `runtime`、`core`、`platform-std`

当前极简状态是可接受的，但不是长期终态。

---

## 8. 当前阶段的依赖方向

当前模板推荐采用如下依赖方向：

```text
base/error   \
base/log      \
base/types     >  model
base/utils    /

model  -----> core
base/* -----> core

base/log ----> runtime

base/log ----> platform-std

runtime -----> sdk
```

### 当前阶段依赖规则

- `base/*` 不依赖 `crates/*`
- `model` 可依赖 `base/*`
- `core` 可依赖 `model` 与 `base/*`
- `runtime` 当前只依赖 `base/log`
- `platform-std` 当前只依赖 `base/log`
- `sdk` 当前只依赖 `runtime`

### 未来阶段依赖规则

当出现真实功能域后，可以进一步演进为：

- `runtime` 依赖 `core`（用于实现 ports）
- `sdk` 依赖 `runtime`、`core`、`model`
- `bindings/*` 依赖 `sdk` 与 `ffi-common`

但在功能域尚未确定之前，不要为了“将来要用”而提前把这些依赖都接上。

---

## 9. 当前阶段的工程策略

### 9.1 去除 demo 污染

当前阶段已经明确：

- NTP 不是模板骨架的一部分
- 模板骨架中不保留 NTP-specific request/response/core/runtime/service
- 以后要重新加 demo，也应放在：
  - `examples/`
  - 独立 feature domain
  - 或单独的实验 crate

而不是反向污染 `model` / `core` / `runtime` 主线。

### 9.2 空骨架优于错误骨架

如果某一层当前没有明确的通用职责，宁可先保持小而空，也不要用 demo-specific 代码填满。

但“空”只能是暂时的：

- `core` 未来必须承接真实通用 use case
- `model` 未来必须承接真实公共模型
- `sdk` 未来必须承接真正 façade

### 9.3 先立边界，再补能力

下一阶段 Codex 的工作原则应该是：

1. 优先修正边界
2. 优先修正依赖方向
3. 优先修正 crate 职责
4. 最后才添加真实能力

---

## 10. `sdk` 的目标形态

当前 `sdk` 只是一个最小 re-export 壳，这是阶段性可接受状态。

但架构上应明确，未来 `sdk` 的目标是：

- 成为 Rust 用户唯一优先依赖的入口层
- 提供更自然的 `Builder` / `Context` / `Handle` / `Facade`
- re-export 稳定公开模型和值对象
- 屏蔽 `runtime`、`platform-std` 的内部细节

换句话说：

- 当前 `runtime` 是骨架
- 当前 `sdk` 只是壳
- 未来应逐步把“用户入口形态”收口到 `sdk`

---

## 11. `platform-std` 与 `runtime` 的关系

两者不是一回事。

### `platform-std`

负责：

- std 环境下的默认具体后端
- 例如 tracing logging backend

### `runtime`

负责：

- 运行时上下文
- 运行时共享资源
- 未来 `core` ports 的 concrete adapter 归属地

因此：

- `runtime` 不安装 tracing subscriber
- `platform-std` 不成为 runtime context 本体
- 应用壳 / examples 可以：
  - 先用 `platform-std` 建好具体 logger
  - 再把 `SharedLogger` 注入 `RuntimeBuilder`

这条边界必须长期保持清晰。

---

## 12. 错误与日志的统一方向

### 12.1 错误方向

当前 `base/error` 已经确立了统一错误协议层，因此后续新增功能时应遵循：

- 能力域内部可以有本地错误
- 但最终应可被映射到统一错误语义
- demo-specific 错误类型不应成为模板骨架主线

### 12.2 日志方向

当前 `base/log` + `platform-std` 的路线已经比较健康：

- `base/log` 只定义协议
- `platform-std` 提供默认 logging backend
- `runtime` 只注入 `SharedLogger`
- 应用壳决定是否安装全局 tracing

未来新增能力时，继续沿用这条路线，不要回退到“基础 crate 直接安装 tracing subscriber”的旧路。

---

## 13. 后续添加能力域的正确方式

当模板骨架完成后，未来要新增一个能力域（例如 `ptt`、`rtc`、`media`、`ntp`）时，应按以下方式接入。

### 第一步：先定义模型

在 `crates/model` 中增加：

- request / response
- options / params
- state / event（如确有必要）

### 第二步：再定义核心规则

在 `crates/core` 中增加：

- port traits
- parser
- service / use case
- core error

### 第三步：再定义 runtime 实现

在 `crates/runtime` 中增加：

- 某个 `core` port 的具体 adapter
- 所需资源的 focused builder
- 运行时资源接入

### 第四步：最后收口到 sdk

在 `crates/sdk` 中：

- 暴露更友好的 Rust façade
- 屏蔽内部细节

### 第五步：bindings 仅做翻译

如需 Android / iOS / OHOS / C 导出：

- 由 `bindings/*` 做类型翻译与边界映射
- 不在 bindings 中重新定义第二套业务规则

---

## 14. 当前阶段 Codex 修正的总原则

把本文档放入工程后，Codex 后续修正应遵循以下规则：

1. 先修 crate 职责，不先补 demo
2. 先修依赖方向，不先补功能数量
3. 不把临时演示能力塞进模板主线
4. 不让 `runtime` 变成 service locator
5. 不让 `base/types` 变成 common 杂物箱
6. 不让 `core` 直接承接真实 I/O
7. 不让 `sdk` 长期只是空壳，但短期允许它保持极简
8. 不让 `platform-std` 反向侵入 `runtime` 与 `core`
9. 空模块如果长期没有真实内容，应删除而不是保留占位
10. 一切新增内容都必须先回答：它属于哪一层

---

## 15. 当前阶段的最终结论

当前工程已经完成了一轮重要纠偏：

- 从“围绕 demo 功能演化模板”
- 转向“先把模板工程做对，再添加功能”

因此当前推荐的架构基线是：

- `base/*`：基础协议与值对象
- `crates/model`：领域公开模型（当前极简）
- `crates/core`：核心规则与端口（当前极简）
- `crates/runtime`：通用运行时承托层
- `crates/platform-std`：std 默认后端实现
- `crates/sdk`：Rust façade 的最小壳

这就是当前 GeorgeRustTemplate 应当继续推进的正确主线。

后续若要补真实能力域，应在此架构之上增量演进，而不是再回到“先把 demo 写进骨架”的路径。
