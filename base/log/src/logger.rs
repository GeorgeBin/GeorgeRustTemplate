use alloc::sync::Arc;

use crate::{LogLevel, LogRecord};

/// Shared trait object type used across the workspace for logger injection.
pub type SharedLogger = Arc<dyn Logger>;

/// Protocol trait implemented by concrete loggers in platform crates.
pub trait Logger: Send + Sync + 'static {
    /// Returns whether a log record should be emitted for this level and target.
    fn enabled(&self, level: LogLevel, target: &str) -> bool {
        let _ = (level, target);
        true
    }

    /// Emits one log record.
    fn log(&self, record: &LogRecord);

    /// Flushes buffered log output if the implementation uses buffering.
    fn flush(&self) {}
}
