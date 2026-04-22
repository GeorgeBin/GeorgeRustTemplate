use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const COMMON_INVALID_INPUT: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(0, 0, 1),
    name: "COMMON_INVALID_INPUT",
    kind: ErrorKind::InvalidInput,
    default_message: "invalid input",
};

pub const COMMON_INTERNAL: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(0, 0, 2),
    name: "COMMON_INTERNAL",
    kind: ErrorKind::Internal,
    default_message: "internal error",
};
