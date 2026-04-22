use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const STORAGE_IO_FAILED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(4, 0, 1),
    name: "STORAGE_IO_FAILED",
    kind: ErrorKind::Upstream,
    default_message: "storage io operation failed",
};

pub const STORAGE_NOT_FOUND: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(4, 0, 2),
    name: "STORAGE_NOT_FOUND",
    kind: ErrorKind::NotFound,
    default_message: "storage resource not found",
};
