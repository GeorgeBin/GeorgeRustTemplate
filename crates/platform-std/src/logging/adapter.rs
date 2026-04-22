use george_base_log::{LogLevel, LogRecord, Logger, SharedLogger};
use std::sync::Arc;

#[derive(Debug, Default, Clone, Copy)]
pub struct TracingForwardLogger;

impl Logger for TracingForwardLogger {
    fn enabled(&self, _level: LogLevel, _target: &str) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        let fields = format_fields(record);
        let target = record.target;

        match record.level {
            LogLevel::Error => tracing::event!(
                target: "george_base_log",
                tracing::Level::ERROR,
                log_target = %target,
                message = %record.message,
                module_path = ?record.module_path,
                file = ?record.file,
                line = ?record.line,
                fields = %fields
            ),
            LogLevel::Warn => tracing::event!(
                target: "george_base_log",
                tracing::Level::WARN,
                log_target = %target,
                message = %record.message,
                module_path = ?record.module_path,
                file = ?record.file,
                line = ?record.line,
                fields = %fields
            ),
            LogLevel::Info => tracing::event!(
                target: "george_base_log",
                tracing::Level::INFO,
                log_target = %target,
                message = %record.message,
                module_path = ?record.module_path,
                file = ?record.file,
                line = ?record.line,
                fields = %fields
            ),
            LogLevel::Debug => tracing::event!(
                target: "george_base_log",
                tracing::Level::DEBUG,
                log_target = %target,
                message = %record.message,
                module_path = ?record.module_path,
                file = ?record.file,
                line = ?record.line,
                fields = %fields
            ),
            LogLevel::Trace => tracing::event!(
                target: "george_base_log",
                tracing::Level::TRACE,
                log_target = %target,
                message = %record.message,
                module_path = ?record.module_path,
                file = ?record.file,
                line = ?record.line,
                fields = %fields
            ),
        }
    }
}

pub fn shared_tracing_logger() -> SharedLogger {
    Arc::new(TracingForwardLogger)
}

fn format_fields(record: &LogRecord) -> String {
    if record.fields.is_empty() {
        return String::new();
    }

    record
        .fields
        .iter()
        .map(|field| format!("{}={}", field.key, field.value))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::{TracingForwardLogger, shared_tracing_logger};
    use george_base_log::{LogLevel, LogRecord, Logger};

    #[test]
    fn tracing_forward_logger_log_does_not_panic() {
        let logger = TracingForwardLogger;
        let record = LogRecord::new(LogLevel::Info, "demo", "started").with_field("attempt", 1);

        logger.log(&record);
    }

    #[test]
    fn shared_tracing_logger_returns_trait_object() {
        let logger = shared_tracing_logger();
        let record = LogRecord::new(LogLevel::Warn, "demo", "warning");

        logger.log(&record);
    }
}
