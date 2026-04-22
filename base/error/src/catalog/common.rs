use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const COMMON_UNKNOWN: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(0, 0, 1),
    name: "COMMON_UNKNOWN",
    kind: ErrorKind::Internal,
    default_message: "unknown error",
};

pub const COMMON_INTERNAL: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(0, 0, 2),
    name: "COMMON_INTERNAL",
    kind: ErrorKind::Internal,
    default_message: "internal error",
};
