use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const CONFIG_MISSING_VALUE: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(1, 0, 1),
    name: "CONFIG_MISSING_VALUE",
    kind: ErrorKind::InvalidInput,
    default_message: "required config value is missing",
};

pub const CONFIG_INVALID_VALUE: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(1, 0, 2),
    name: "CONFIG_INVALID_VALUE",
    kind: ErrorKind::InvalidInput,
    default_message: "config value is invalid",
};
