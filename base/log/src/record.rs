use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{LogField, LogLevel};

/// Represents one log record emitted through the logging protocol layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    pub level: LogLevel,
    pub target: &'static str,
    pub message: String,
    pub module_path: Option<&'static str>,
    pub file: Option<&'static str>,
    pub line: Option<u32>,
    pub fields: Vec<LogField>,
}

impl LogRecord {
    /// Creates a new log record with no source location and no structured fields.
    pub fn new(level: LogLevel, target: &'static str, message: impl Into<String>) -> Self {
        Self {
            level,
            target,
            message: message.into(),
            module_path: None,
            file: None,
            line: None,
            fields: Vec::new(),
        }
    }

    /// Appends one structured field to the record.
    pub fn with_field(mut self, key: &'static str, value: impl ToString) -> Self {
        self.fields.push(LogField::new(key, value));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::LogRecord;
    use crate::LogLevel;

    #[test]
    fn new_initializes_expected_defaults() {
        let record = LogRecord::new(LogLevel::Info, "demo", "started");

        assert_eq!(record.level, LogLevel::Info);
        assert_eq!(record.target, "demo");
        assert_eq!(record.message, "started");
        assert_eq!(record.module_path, None);
        assert_eq!(record.file, None);
        assert_eq!(record.line, None);
        assert!(record.fields.is_empty());
    }

    #[test]
    fn with_field_appends_fields_in_order() {
        let record = LogRecord::new(LogLevel::Warn, "demo", "warning")
            .with_field("attempt", 1)
            .with_field("user", "alice");

        assert_eq!(record.fields.len(), 2);
        assert_eq!(record.fields[0].key, "attempt");
        assert_eq!(record.fields[0].value, "1");
        assert_eq!(record.fields[1].key, "user");
        assert_eq!(record.fields[1].value, "alice");
    }
}
