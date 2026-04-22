use crate::{LogLevel, LogRecord, Logger};

/// Logger implementation that discards all log records.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopLogger;

impl Logger for NoopLogger {
    fn enabled(&self, _level: LogLevel, _target: &str) -> bool {
        true
    }

    fn log(&self, _record: &LogRecord) {}

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::NoopLogger;
    use crate::{LogLevel, LogRecord, Logger};

    #[test]
    fn noop_logger_is_enabled_and_does_not_panic() {
        let logger = NoopLogger;
        let record = LogRecord::new(LogLevel::Info, "demo", "message");

        assert!(logger.enabled(LogLevel::Info, "demo"));
        logger.log(&record);
        logger.flush();
    }
}
