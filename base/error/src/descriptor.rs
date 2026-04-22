use crate::{ErrorCode, ErrorKind};

#[derive(Debug)]
pub struct ErrorDescriptor {
    pub code: ErrorCode,
    pub name: &'static str,
    pub kind: ErrorKind,
    pub default_message: &'static str,
}
