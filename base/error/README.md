# george-base-error

`george-base-error` 是 RustTemplate 工程中的**统一错误协议层**。

它负责定义：

- 稳定的错误码体系
- 统一的错误语义模型
- 统一的运行时错误实例结构
- 基础领域 catalog 的注册方式
- Rust 内部错误传播所需的基础类型

它**不负责**：

- 某个产品线的完整业务错误全集
- Android / iOS / OpenHarmony 的具体导出 DTO
- 某个平台绑定层的私有异常模型
- 所有第三方库错误的自动全量映射

跨平台导出 DTO、FFI 友好结构、桥接层错误包装，统一放在 `crates/ffi-common` 或 `bindings/*` 中处理。

---

## 1. 设计目标

`george-base-error` 的设计目标如下：

1. **统一**：整个 workspace 共享一套错误协议。
2. **稳定**：错误码一旦发布，不轻易改变语义。
3. **可扩展**：支持未来多个产品线、多个平台持续扩展。
4. **可诊断**：保留 detail、native、context、source chain。
5. **可跨层复用**：既可用于 Rust 内部传播，也可被桥接层安全映射。
6. **可自动化**：便于通过 Codex 等工具生成 catalog 与样板代码。

---

## 2. 总体设计思想

统一错误分为两层：

### 2.1 稳定语义层

由以下静态字段构成：

- `code`
- `name`
- `kind`
- `default_message`

这一层用于：

- 程序判断
- 文档归档
- 埋点统计
- 稳定协议传递
- 客户支持定位

### 2.2 运行时实例层

由以下动态字段构成：

- `detail`
- `native`
- `context`
- `source`

这一层用于：

- 调试日志
- 故障诊断
- 原始错误保留
- Rust 错误链衔接

核心原则：

> **错误码表达统一语义；native 保留原始现场。**

---

## 3. 错误码设计

本 crate 采用 **7 位错误码**：

```text
DDSSRRR
```

含义：

- `DD`：领域（Domain），`00~99`
- `SS`：子域（Subdomain），`00~99`
- `RRR`：原因码（Reason），`000~999`

示例：

- `0306404`：网络 / HTTP / 404 Not Found
- `0303001`：网络 / TCP / 连接超时
- `0501001`：数据 / JSON / 解码失败
- `0702001`：平台 / JNI / 回调线程不可用

采用 7 位码而不是 5 位码，是因为该模板工程面向长期演进的跨平台 SDK 体系，需要为 network / protocol / ffi / media / product 扩展预留足够空间。

---

## 4. native error 与统一错误码的关系

统一错误码表示的是**本 SDK 体系内部对错误语义的稳定归类**。

`native` 表示的是底层真实返回的信息，例如：

- HTTP 状态码
- errno
- 第三方库报错文本
- TLS / JNI / NAPI 的底层原始错误

因此：

- 不应只靠错误码承载所有底层细节
- 不应丢弃原生错误信息
- 标准化且跨平台一致的编号，可视情况借用到 `RRR`
- 平台私有、库私有、上下文相关的编号，应先映射为统一语义，再保留原生信息

---

## 5. 领域与子域建议

### 5.1 Domain (`DD`)

| DD | Domain | 说明 |
|---|---|---|
| `00` | Common | 通用基础错误 |
| `01` | Config / Input | 配置、参数、初始化输入 |
| `02` | Runtime / Lifecycle | 生命周期、状态、并发、任务 |
| `03` | Network | 网络、DNS、Socket、TCP、HTTP、TLS |
| `04` | Storage / IO | 文件、缓存、持久化、磁盘 |
| `05` | Data / Codec / Protocol | JSON、Protobuf、协议、校验 |
| `06` | Security / Auth | 权限、认证、证书、签名 |
| `07` | Platform / FFI | JNI、UniFFI、NAPI、桥接线程 |
| `08` | Media / Device | 音视频、设备、硬件资源 |
| `09` | Product Shared | 产品共享但非基础层的语义 |
| `10~99` | Reserved | 预留 |

### 5.2 示例子域

#### Network (`DD = 03`)

| SS | Subdomain |
|---|---|
| `00` | Generic Network |
| `02` | DNS |
| `03` | TCP |
| `04` | UDP |
| `05` | Socket / OS |
| `06` | HTTP |
| `07` | TLS |
| `08` | WebSocket |
| `10` | SIP |
| `11` | RTP |
| `13` | WebRTC |

#### Platform (`DD = 07`)

| SS | Subdomain |
|---|---|
| `00` | Generic Platform |
| `01` | FFI Common |
| `02` | JNI / Android |
| `03` | UniFFI |
| `04` | NAPI / OHOS |
| `05` | Swift / iOS |
| `06` | Callback / Threading |
| `07` | Env / Path / Process |

---

## 6. 核心类型设计

### 6.1 `ErrorCode`

负责：

- 从 `domain / subdomain / reason` 构造错误码
- 拆分字段
- 提供 `u32` 值
- 提供 7 位零填充显示

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(u32);

impl ErrorCode {
    pub const fn from_parts(domain: u8, subdomain: u8, reason: u16) -> Self;
    pub const fn as_u32(self) -> u32;
    pub const fn domain(self) -> u8;
    pub const fn subdomain(self) -> u8;
    pub const fn reason(self) -> u16;
}
```

### 6.2 `ErrorKind`

表示跨领域复用的高层错误分类。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Upstream,
    InvalidInput,
    InvalidState,
    Timeout,
    Cancelled,
    NotFound,
    Conflict,
    PermissionDenied,
    Unavailable,
    Parse,
    Encode,
    Decode,
    Verify,
    Internal,
}
```

说明：

- `ErrorKind` 不替代 `ErrorCode`
- 逻辑判断优先使用 `code`
- 大类判断或聚合统计时，再使用 `kind`

### 6.3 `ErrorDescriptor`

表示静态、稳定的错误定义项。

```rust
#[derive(Debug)]
pub struct ErrorDescriptor {
    pub code: ErrorCode,
    pub name: &'static str,
    pub kind: ErrorKind,
    pub default_message: &'static str,
}
```

规则：

- `name` 必须稳定
- `default_message` 应简洁，不夹带动态变量
- 一个 `descriptor` 对应一个稳定语义

### 6.4 `NativeError`

表示底层原生错误现场。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeError {
    pub source: &'static str,
    pub code: Option<String>,
    pub message: Option<String>,
}
```

### 6.5 `ErrorContext`

表示结构化上下文。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContext {
    pub key: &'static str,
    pub value: String,
}
```

### 6.6 `BaseError`

表示 Rust 运行时错误实例。

```rust
#[derive(Debug)]
pub struct BaseError {
    pub desc: &'static ErrorDescriptor,
    pub detail: Option<String>,
    pub native: Option<NativeError>,
    pub context: Vec<ErrorContext>,
    #[cfg(feature = "std")]
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}
```

说明：

- `desc`：稳定语义
- `detail`：动态说明
- `native`：原始现场
- `context`：结构化补充信息
- `source`：Rust 内部错误链，仅在 `std` 能力下保留

---

## 7. 构造方式

建议使用链式 builder 风格：

```rust
impl BaseError {
    pub fn new(desc: &'static ErrorDescriptor) -> Self;
    pub fn detail(mut self, detail: impl Into<String>) -> Self;
    pub fn native(
        mut self,
        source: &'static str,
        code: Option<impl Into<String>>,
        message: Option<impl Into<String>>,
    ) -> Self;
    pub fn context(mut self, key: &'static str, value: impl ToString) -> Self;

    #[cfg(feature = "std")]
    pub fn source<E>(mut self, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static;
}
```

示例：

```rust
let err = BaseError::new(&NET_HTTP_NOT_FOUND)
    .detail("GET https://api.example.com/time returned 404")
    .native("http", Some("404"), Some("Not Found"))
    .context("method", "GET")
    .context("url", "https://api.example.com/time");
```

---

## 8. catalog 组织建议

```text
base/error/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── code.rs
    ├── kind.rs
    ├── descriptor.rs
    ├── native.rs
    ├── context.rs
    ├── error.rs
    ├── result.rs
    ├── catalog/
    │   ├── mod.rs
    │   ├── common.rs
    │   ├── config.rs
    │   ├── runtime.rs
    │   ├── network.rs
    │   ├── storage.rs
    │   ├── data.rs
    │   ├── security.rs
    │   └── platform.rs
    └── tests.rs
```

规则：

- `base/error` 只维护基础领域 catalog
- 业务产品错误由各自 crate 扩展
- 不使用无边界的 `common` 杂物堆做业务回收站

---

## 9. 与 `ffi-common` 的边界

根据 RustTemplate 的总体分层，`base/error` 适合放基础错误机制与基础类型；而 `ffi-common` 适合放 FFI 安全字符串、桥接公共结构与错误码映射公共逻辑。

因此推荐：

- `base/error`：定义错误协议本体
- `crates/ffi-common`：定义 `ErrorDto`、`ErrorContextDto`、`to_error_dto()` 等导出辅助结构

推荐 DTO：

```rust
pub struct ErrorContextDto {
    pub key: String,
    pub value: String,
}

pub struct ErrorDto {
    pub code: u32,
    pub code_str: String,
    pub name: String,
    pub kind: String,
    pub message: String,
    pub detail: Option<String>,
    pub native_source: Option<String>,
    pub native_code: Option<String>,
    pub native_message: Option<String>,
    pub context: Vec<ErrorContextDto>,
}
```

说明：

- `code` 用于机器处理
- `code_str` 保留 7 位零填充格式，便于日志、文档、客服定位
- `context` 不应在导出时丢失
- DTO 不直接暴露 `source`

---

## 10. crate 职责边界

### `george-base-error` 负责

- 7 位错误码机制
- 通用错误类型
- 基础领域 catalog
- `Result<T>` 别名
- Rust 内部错误链承接

### `george-base-error` 不负责

- 具体产品错误全集
- 各绑定层私有异常模型
- FFI 导出 DTO
- 全量第三方库错误自动映射

---

## 11. 使用规范

推荐：

- 使用 descriptor 常量定义错误
- 使用 `BaseError::new(&DESC)` 构造运行时实例
- 尽量补充 `detail / native / context`
- 对标准化外部编号做谨慎借用
- 对平台私有错误做语义映射后再保留 native

不推荐：

- 到处手写裸数字错误码
- 仅靠 `detail` 做逻辑判断
- 为所有外部错误实现宽泛 `From<T>`
- 在 `default_message` 中拼动态变量
- 在导出层直接暴露 Rust trait object

---

## 12. 单元测试建议

至少覆盖：

1. `ErrorCode::from_parts()` 与拆分字段
2. 7 位零填充显示格式
3. `BaseError` builder 链式构造
4. descriptor 与 runtime instance 组合
5. catalog 中 `code` / `name` 唯一性校验
6. DTO 转换时 `context` 与 `code_str` 保留正确

---

## 13. 建议公开导出

```rust
pub use code::ErrorCode;
pub use kind::ErrorKind;
pub use descriptor::ErrorDescriptor;
pub use native::NativeError;
pub use context::ErrorContext;
pub use error::BaseError;

pub type Result<T> = core::result::Result<T, BaseError>;
```

---

## 14. 小结

`george-base-error` 的最终定位是：

> 为整个 Rust 跨平台 SDK 体系提供统一、稳定、可扩展的错误协议本体。

它坚持的原则是：

- **错误码表达统一语义**
- **native 保留原始现场**
- **catalog 注册优先于散落临时错误**
- **FFI 导出形态放在 `ffi-common`，不放在 `base/error`**

后续 `ptt-*`、`ntp-*`、`media-*` 等产品线，应基于该协议扩展自己的业务错误 catalog，而不是重建一套错误系统。
