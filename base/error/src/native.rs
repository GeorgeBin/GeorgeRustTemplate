use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeError {
    pub source: &'static str,
    pub code: Option<String>,
    pub message: Option<String>,
}
