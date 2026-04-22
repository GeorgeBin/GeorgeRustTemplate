# george-platform-std

`george-platform-std` 是 RustTemplate 在 **std runtime** 下的默认平台实现层。

它当前主要承接日志相关的具体实现：tracing subscriber 安装、console/file 输出、文件滚动、日志清理，以及运行时日志配置调整。

---

## 1. 职责边界

本 crate **负责**：

- std 环境下的 tracing 日志实现
- 全局 tracing subscriber 安装
- console / file 输出配置
- 本地日期文件滚动
- 旧日志清理
- runtime log config 动态调整
- `TracingForwardLogger`，把 `george-base-log` 的协议记录转发到 tracing

本 crate **不负责**：

- 跨平台日志协议定义
- 业务日志字段规范
- FFI 日志导出结构
- Android JNI / iOS / OHOS 专属运行时桥接

跨平台协议定义统一在 `base/log` 中维护。

---

## 2. 与 `george-base-log` 的关系

推荐分层：

- `george-base-log`：日志协议层
- `george-platform-std`：std runtime 默认实现层

其中：

- `Logger` trait 定义在 `george-base-log`
- `TracingForwardLogger` 实现在 `george-platform-std`
- `install_global_tracing(...)` 也只存在于 `george-platform-std`

这意味着：

- 普通 library crate 应优先依赖 `Logger` trait
- binaries / demos / app shells 可以在入口安装默认 tracing backend

---

## 3. 核心公开 API

### 3.1 配置类型

- `StdLogConfig`
- `ConsoleLogConfig`
- `FileLogConfig`
- `CleanupConfig`
- `RuntimeLogConfig`

这些类型描述 std backend 的 console/file/cleanup/runtime 行为。

### 3.2 安装与运行时控制

- `install_global_tracing`
- `global_logging`
- `StdLoggingHandle`

说明：

- `install_global_tracing(...)` 会安装 **process-wide global tracing subscriber**
- 它是应用入口安装函数，**只应由 binaries / app shells / demos 调用**
- 普通 library crate 不应在正常运行路径中调用它
- `StdLoggingHandle` 用于查询和更新 runtime logging config

### 3.3 后端辅助能力

- `cleanup_old_logs`
- `build_file_appender`
- `TracingForwardLogger`
- `shared_tracing_logger`

其中：

- `TracingForwardLogger` 把 `LogRecord` 转发为 tracing event
- tracing event 会保留真实 `record.target`
- `fields` 当前会先格式化为附加文本字段

---

## 4. 日志安装流程

典型 app / demo 启动流程如下：

1. 构造 `StdLogConfig`
2. 在应用入口调用 `install_global_tracing(config)`
3. 如有需要，通过 `global_logging()` 或返回的 `StdLoggingHandle` 调整 runtime config

示例：

```rust
use george_base_log::LogLevel;
use george_platform_std::{
    CleanupConfig, ConsoleLogConfig, FileLogConfig, StdLogConfig, install_global_tracing,
};
use std::path::PathBuf;

let config = StdLogConfig {
    enabled: true,
    level: LogLevel::Info,
    console: ConsoleLogConfig { enabled: true },
    file: FileLogConfig {
        enabled: true,
        directory: PathBuf::from("./logs"),
        file_prefix: "demo".to_string(),
    },
    cleanup: CleanupConfig {
        enabled: true,
        max_retention_days: 7,
    },
};

let handle = install_global_tracing(config)?;
```

---

## 5. `TracingForwardLogger`

`TracingForwardLogger` 的作用是：

- 接收 `george-base-log::LogRecord`
- 映射日志 level
- 保留 `record.target`
- 把 `message / module_path / file / line / fields` 转发为 tracing event

这使得后续架构可以演进为：

- `core / runtime / sdk` 只感知 `Logger` trait
- executable 安装 tracing subscriber
- 协议层记录通过 `TracingForwardLogger` 进入 tracing 体系，再统一输出到 console/file

实现说明：

- 为了保留真实 tracing target，当前实现会按
  `(level, target, module_path, file, line)` 组合缓存动态 callsite
- 这些 callsite 会在进程生命周期内常驻
- 这是一种“以常驻内存换动态 target”的实现策略
- 对应用代码来说通常可接受，因为这些值大多来自 `'static` 且集合有限
- 它适合模板工程的应用级静态调用点场景，不适合作为无限动态 target 的通用模式

---

## 6. 文件日志能力

当前 std backend 还包含：

- 本地日期滚动文件 appender
- 按 retention days 清理历史日志
- 运行时动态开关 console/file 输出
- 运行时动态调整日志 level

这些能力都是 **std runtime 的默认实现细节**，不应回流到 `base/log`。

---

## 7. 小结

`george-platform-std` 的定位是：

> 为 RustTemplate 的桌面、CLI、测试与 demo 场景提供一套默认可用的 std 日志实现。

它坚持的原则是：

- **协议在 `base/log`**
- **实现放 `platform-std`**
- **全局 subscriber 安装只能由应用入口负责**
