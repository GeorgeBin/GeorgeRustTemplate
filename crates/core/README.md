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

- `std::net::UdpSocket` 或任何真实 socket I/O
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

## 为什么 NTP 的 std 网络实现必须移出 core

旧实现把 UDP 通信、协议解析、错误和用例逻辑都堆在 `core`，导致 core 直接依赖 std 网络和运行时时间类型。

这会让核心层失去可替换性和可测试性，也让 runtime 与平台层没有明确接入点。

现在 `ntp` 模块拆成：

- `error`：核心语义错误
- `parser`：纯 NTP 响应解析
- `port`：NTP 传输端口抽象
- `service`：NTP 核心用例服务

这样 core 只保留规则和编排，真实网络实现可以迁移到 runtime 或更外层。
