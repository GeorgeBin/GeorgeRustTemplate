use std::sync::Arc;

use george_base_log::{NoopLogger, SharedLogger};

use crate::RuntimeContext;

/// Builder for [`RuntimeContext`].
///
/// The builder only wires stable runtime-wide dependencies. Concrete feature
/// adapters should add their own focused builders when they need resources.
#[derive(Default)]
pub struct RuntimeBuilder {
    logger: Option<SharedLogger>,
}

impl RuntimeBuilder {
    /// Creates an empty runtime builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Injects the logger shared by runtime adapters.
    pub fn with_logger(mut self, logger: SharedLogger) -> Self {
        self.logger = Some(logger);
        self
    }

    /// Builds a runtime context.
    ///
    /// If no logger was provided, the context uses [`NoopLogger`].
    pub fn build(self) -> RuntimeContext {
        RuntimeContext::new(self.logger.unwrap_or_else(|| Arc::new(NoopLogger)))
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use george_base_log::{LogLevel, LogRecord, Logger};

    use super::RuntimeBuilder;
    use crate::RuntimeContext;

    struct MarkerLogger {
        calls: AtomicUsize,
    }

    impl MarkerLogger {
        fn new() -> Self {
            Self {
                calls: AtomicUsize::new(0),
            }
        }

        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    impl Logger for MarkerLogger {
        fn enabled(&self, _level: LogLevel, target: &str) -> bool {
            target == "runtime-test"
        }

        fn log(&self, _record: &LogRecord) {
            self.calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn new_builds_context() {
        let context = RuntimeBuilder::new().build();

        assert!(context.logger().enabled(LogLevel::Info, "runtime"));
    }

    #[test]
    fn default_logger_is_usable_noop_logger() {
        let context = RuntimeBuilder::new().build();
        let record = LogRecord::new(LogLevel::Info, "runtime-test", "message");

        assert!(context.logger().enabled(LogLevel::Info, "runtime-test"));
        context.logger().log(&record);
        context.logger().flush();
    }

    #[test]
    fn with_logger_injects_shared_logger() {
        let marker = Arc::new(MarkerLogger::new());
        let logger = marker.clone();
        let context = RuntimeBuilder::new().with_logger(logger).build();
        let record = LogRecord::new(LogLevel::Info, "runtime-test", "message");

        context.logger().log(&record);

        assert_eq!(marker.calls(), 1);
    }

    #[test]
    fn context_builder_builds_context() {
        let context = RuntimeContext::builder().build();

        assert!(context.logger().enabled(LogLevel::Info, "runtime"));
    }
}
