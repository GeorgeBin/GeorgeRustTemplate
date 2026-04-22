# george-base-types

`george-base-types` 是 RustTemplate 的基础值对象层。

它负责为多个 crate 共享、平台无关、语义稳定的小型值对象提供统一归属地。它不是业务模型层，也不是“公共杂物箱”。

## 边界

本 crate 负责：

- 基础值对象
- 裸 `u64`、`String` 等标量的轻量语义包装
- 带最小必要校验的稳定 newtype

本 crate 不负责：

- 业务模型、配置模型、事件模型
- 协议结构、平台 DTO、FFI 传输类型
- “为了以后”预埋的大量抽象

与 `crates/model` 的边界很直接：

- 跨 crate 复用、平台无关、语义稳定的小值对象放在 `george-base-types`
- 某个产品或领域自己的公开模型放在 `crates/model`

## 当前首版范围

当前实现只包含两组类型：

- `HandleId`
- `RequestId`
- `InternalCorrelationId`
- `NonEmptyString`
- `InvalidIdError`
- `EmptyStringError`

其中：

- `HandleId`、`RequestId`、`InternalCorrelationId` 使用 `NonZeroU64` 表达非零 ID
- `NonEmptyString` 使用 `trim()` 判空，但保留原始字符串内容，不做裁剪或归一化
- 错误类型保持轻量，不引入第三方依赖
- `InternalCorrelationId` 明确只表示 SDK/进程内请求链路关联号，不占用未来跨进程字符串型 `CorrelationId` 的语义名

## 命名边界

当前 crate 不承担 HTTP header、gRPC、WebSocket、日志系统等跨进程 tracing 标识的统一抽象。

如果后续需要真正跨进程、跨协议传播的关联字段，应新增字符串型 `CorrelationId`，而不是继续扩展当前的 `InternalCorrelationId`。

## 设计约束

- 保持平台无关
- 只放值对象，不放大模型
- 至少存在明确的跨 crate 复用场景
- 不为了潜在未来需求提前引入泛型框架、宏系统或复杂抽象

## 实现风格

本 crate 保持轻量：

- 默认启用 `std`
- 支持 `no_std + alloc`
- 不引入任何第三方依赖

首版刻意收敛到少量稳定 API，后续只在边界清晰、确有复用需求时再扩展。
