# george-base-log

`george-base-log` 是 RustTemplate 工程中的**基础日志协议层**。

它只负责定义稳定、轻量、可跨层复用的日志抽象，不负责任何具体平台或运行时下的日志后端实现。

---

## 1. 职责边界

本 crate **负责**：

- `LogLevel`
- `LogField`
- `LogRecord`
- `Logger` trait
- `LoggerExt`
- `SharedLogger`
- `NoopLogger`

本 crate **不负责**：

- tracing subscriber 安装
- console / file 日志后端
- 文件滚动与旧日志清理
- `PathBuf` / `std::fs` / 本地日期规则
- Android Logcat / iOS NSLog / OHOS HiLog 等平台实现

这些具体能力统一放在 `crates/platform-std` 或未来的平台实现 crate 中处理。

---

## 2. 设计目标

`george-base-log` 的目标是：

1. 为整个 workspace 提供统一的日志协议
2. 让 `core / runtime / sdk` 依赖抽象，而不是依赖 tracing subscriber
3. 保持 API 稳定、简单、低耦合
4. 保留 `no_std + alloc` 演进空间

因此本 crate 采用：

- `default = ["std"]`
- `#![cfg_attr(not(feature = "std"), no_std)]`
- `extern crate alloc`

同时避免引入具体后端依赖，如 `tracing`、`tracing-subscriber`、`chrono`、`thiserror` 等。

---

## 3. 核心类型

### 3.1 `LogLevel`

表示日志等级：

- `Error`
- `Warn`
- `Info`
- `Debug`
- `Trace`

提供：

- `as_str()`
- `Display`

### 3.2 `LogField`

表示一条结构化字段：

```rust
pub struct LogField {
    pub key: &'static str,
    pub value: String,
}
```

### 3.3 `LogRecord`

表示一条协议层日志记录：

```rust
pub struct LogRecord {
    pub level: LogLevel,
    pub target: &'static str,
    pub message: String,
    pub module_path: Option<&'static str>,
    pub file: Option<&'static str>,
    pub line: Option<u32>,
    pub fields: Vec<LogField>,
}
```

提供：

- `LogRecord::new(...)`
- `LogRecord::with_field(...)`

说明：

- `target` 是稳定的路由/分类信息
- `fields` 用于携带轻量结构化上下文
- `module_path / file / line` 为可选项，不要求所有 logger 自动补齐

### 3.4 `Logger`

协议层 logger trait：

```rust
pub trait Logger: Send + Sync + 'static {
    fn enabled(&self, level: LogLevel, target: &str) -> bool {
        true
    }

    fn log(&self, record: &LogRecord);

    fn flush(&self) {}
}
```

### 3.5 `LoggerExt`

提供便捷方法：

- `error`
- `warn`
- `info`
- `debug`
- `trace`

这些方法当前只负责构造基础 `LogRecord`，**不会自动捕获调用点 source location**。如果后续需要调用点信息，应通过宏或额外 helper 扩展，而不是让协议层先耦合具体实现。

### 3.6 `SharedLogger` 与 `NoopLogger`

- `SharedLogger = Arc<dyn Logger>`
- `NoopLogger`：丢弃所有日志记录的空实现

---

## 4. 使用方式

### 4.1 依赖抽象

业务层、核心层、运行时层应优先依赖 `Logger` trait 或 `SharedLogger`，而不是直接依赖具体日志后端。

### 4.2 构造记录

```rust
use george_base_log::{LogLevel, LogRecord};

let record = LogRecord::new(LogLevel::Info, "demo.startup", "application started")
    .with_field("version", "0.1.0");
```

### 4.3 使用便捷方法

```rust
use george_base_log::{Logger, LoggerExt, NoopLogger};

let logger = NoopLogger;
logger.info("demo.startup", "application started");
```

---

## 5. 与 `george-platform-std` 的关系

推荐分层如下：

- `george-base-log`：定义日志协议
- `george-platform-std`：提供 std runtime 下的默认实现

例如：

- `TracingForwardLogger` 会实现 `Logger`
- `install_global_tracing(...)` 只存在于 `george-platform-std`
- 文件落盘、rollover、cleanup 也只存在于 `george-platform-std`

这样可以保证：

- `base/log` 保持稳定、纯净
- `core / runtime / sdk` 不被具体 subscriber 安装逻辑污染
- executable / demo 可以选择默认 std 实现，也可以替换成未来的平台实现

---

## 6. 小结

`george-base-log` 的最终定位是：

> 为整个 RustTemplate 提供统一、轻量、稳定的日志协议层。

它坚持的原则是：

- **协议层只定义抽象**
- **具体后端放在平台实现层**
- **库层依赖 logger trait，而不是依赖 tracing subscriber**

