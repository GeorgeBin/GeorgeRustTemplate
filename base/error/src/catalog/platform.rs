use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const PLATFORM_CALLBACK_UNAVAILABLE: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(7, 6, 1),
    name: "PLATFORM_CALLBACK_UNAVAILABLE",
    kind: ErrorKind::Unavailable,
    default_message: "platform callback thread is unavailable",
};
