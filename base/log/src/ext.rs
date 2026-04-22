use alloc::string::String;

use crate::{LogLevel, LogRecord, Logger};

/// Convenience helpers for constructing and emitting simple log records.
pub trait LoggerExt: Logger {
    /// Emits an error record with the given target and message.
    fn error(&self, target: &'static str, message: impl Into<String>) {
        self.emit(LogLevel::Error, target, message);
    }

    /// Emits a warn record with the given target and message.
    fn warn(&self, target: &'static str, message: impl Into<String>) {
        self.emit(LogLevel::Warn, target, message);
    }

    /// Emits an info record with the given target and message.
    fn info(&self, target: &'static str, message: impl Into<String>) {
        self.emit(LogLevel::Info, target, message);
    }

    /// Emits a debug record with the given target and message.
    fn debug(&self, target: &'static str, message: impl Into<String>) {
        self.emit(LogLevel::Debug, target, message);
    }

    /// Emits a trace record with the given target and message.
    fn trace(&self, target: &'static str, message: impl Into<String>) {
        self.emit(LogLevel::Trace, target, message);
    }

    fn emit(&self, level: LogLevel, target: &'static str, message: impl Into<String>) {
        if self.enabled(level, target) {
            let record = LogRecord::new(level, target, message);
            self.log(&record);
        }
    }
}

impl<T: Logger + ?Sized> LoggerExt for T {}

#[cfg(test)]
mod tests {
    use alloc::{string::String, vec::Vec};
    use std::sync::Mutex;

    use super::LoggerExt;
    use crate::{LogLevel, LogRecord, Logger};

    #[derive(Default)]
    struct CollectLogger {
        records: Mutex<Vec<LogRecord>>,
    }

    impl Logger for CollectLogger {
        fn log(&self, record: &LogRecord) {
            self.records
                .lock()
                .expect("records mutex poisoned")
                .push(record.clone());
        }
    }

    #[test]
    fn info_and_error_helpers_emit_records() {
        let logger = CollectLogger::default();

        logger.info("demo", "started");
        logger.error("demo", String::from("failed"));

        let records = logger.records.lock().expect("records mutex poisoned");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].level, LogLevel::Info);
        assert_eq!(records[0].target, "demo");
        assert_eq!(records[0].message, "started");
        assert_eq!(records[1].level, LogLevel::Error);
        assert_eq!(records[1].message, "failed");
    }
}
