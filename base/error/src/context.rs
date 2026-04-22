use alloc::string::{String, ToString};

/// Stores one structured key/value pair attached to a runtime error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContext {
    pub key: &'static str,
    pub value: String,
}

impl ErrorContext {
    /// Creates one structured context item from a static key and string-like value.
    pub fn new(key: &'static str, value: impl ToString) -> Self {
        Self {
            key,
            value: value.to_string(),
        }
    }
}
