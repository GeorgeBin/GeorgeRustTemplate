use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(7, 6, 1),
    name: "PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE",
    kind: ErrorKind::Unavailable,
    default_message: "JNI callback thread is unavailable",
};
