use george_base_log::SharedLogger;

use crate::RuntimeBuilder;

/// Shared runtime context passed to concrete runtime adapters.
///
/// The context is intentionally small. It is not a service locator and does not
/// own feature-specific transports, task systems, or platform backends.
pub struct RuntimeContext {
    logger: SharedLogger,
}

impl RuntimeContext {
    pub(crate) fn new(logger: SharedLogger) -> Self {
        Self { logger }
    }

    /// Creates a builder for a runtime context.
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Returns the shared logger used by runtime adapters.
    pub fn logger(&self) -> &SharedLogger {
        &self.logger
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use george_base_log::{LogLevel, LogRecord, Logger};

    use super::RuntimeContext;

    struct CollectLogger;

    impl Logger for CollectLogger {
        fn enabled(&self, level: LogLevel, target: &str) -> bool {
            level >= LogLevel::Info && target == "runtime-test"
        }

        fn log(&self, _record: &LogRecord) {}
    }

    #[test]
    fn logger_returns_shared_logger() {
        let logger = Arc::new(CollectLogger);
        let context = RuntimeContext::new(logger);
        let record = LogRecord::new(LogLevel::Info, "runtime-test", "message");

        assert!(context.logger().enabled(LogLevel::Info, "runtime-test"));
        context.logger().log(&record);
    }
}
