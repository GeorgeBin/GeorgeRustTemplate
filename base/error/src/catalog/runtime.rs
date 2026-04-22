use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const RUNTIME_INVALID_STATE: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(2, 0, 1),
    name: "RUNTIME_INVALID_STATE",
    kind: ErrorKind::InvalidState,
    default_message: "runtime is in an invalid state",
};

pub const RUNTIME_TASK_CANCELLED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(2, 0, 2),
    name: "RUNTIME_TASK_CANCELLED",
    kind: ErrorKind::Cancelled,
    default_message: "runtime task was cancelled",
};
