use alloc::string::String;

/// Captures native error information from an underlying platform or library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeError {
    pub source: &'static str,
    pub code: Option<String>,
    pub message: Option<String>,
}

impl NativeError {
    /// Creates a native error payload with optional code and message fields.
    pub fn new<C, M>(source: &'static str, code: Option<C>, message: Option<M>) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        Self {
            source,
            code: code.map(Into::into),
            message: message.map(Into::into),
        }
    }
}
