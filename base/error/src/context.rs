use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContext {
    pub key: &'static str,
    pub value: String,
}
