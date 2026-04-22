use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const SEC_AUTH_PERMISSION_DENIED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(6, 1, 1),
    name: "SEC_AUTH_PERMISSION_DENIED",
    kind: ErrorKind::PermissionDenied,
    default_message: "permission denied",
};

pub const SEC_CERT_VERIFY_FAILED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(6, 2, 1),
    name: "SEC_CERT_VERIFY_FAILED",
    kind: ErrorKind::Verify,
    default_message: "certificate verification failed",
};
