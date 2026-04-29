# template-core / crates/core

`crates/core` 是 RustTemplate 的核心编排层。

它负责把基础值对象、领域公开模型和外部能力端口组织成稳定、可测试的核心用例，不负责真实平台接入或运行时执行。

## 负责什么

- 核心用例服务
- 领域规则与输入防御
- 纯解析逻辑
- 面向外部依赖的端口抽象
- 底层失败到核心语义错误的映射

## 不负责什么

- 真实 socket I/O、文件 I/O 或平台 API 调用
- `SystemTime` 作为公开领域返回值
- tracing subscriber、平台代码、FFI、UI
- runtime 调度、线程模型、异步驱动

## 分层边界

- `base/*` 提供基础协议、错误协议和值对象
- `template-model` 提供公开领域模型
- `template-core` 解释模型、执行业务规则、定义端口
- `template-runtime` 提供运行时承托和具体实现
- `template-sdk` 对 Rust 用户暴露更友好的 facade
- `bindings/*` 处理跨语言边界

## 当前首版范围

当前模板不内置任何具体业务用例。

新增能力时，先在 `model` 定义稳定公开模型，再在 `core` 定义核心服务和外部端口。真实平台实现应放在 `runtime`、`platform-*` 或应用壳层中，而不是直接写入 `core`。
