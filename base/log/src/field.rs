use alloc::string::{String, ToString};

/// Stores one structured log field as a key/value pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogField {
    pub key: &'static str,
    pub value: String,
}

impl LogField {
    /// Creates one structured log field from a static key and string-like value.
    pub fn new(key: &'static str, value: impl ToString) -> Self {
        Self {
            key,
            value: value.to_string(),
        }
    }
}
