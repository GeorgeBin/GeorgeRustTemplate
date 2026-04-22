use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const NET_DNS_LOOKUP_FAILED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(3, 2, 1),
    name: "NET_DNS_LOOKUP_FAILED",
    kind: ErrorKind::Unavailable,
    default_message: "dns lookup failed",
};

pub const NET_TCP_TIMEOUT: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(3, 3, 1),
    name: "NET_TCP_TIMEOUT",
    kind: ErrorKind::Timeout,
    default_message: "tcp connection timed out",
};

pub const NET_HTTP_NOT_FOUND: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(3, 6, 404),
    name: "NET_HTTP_NOT_FOUND",
    kind: ErrorKind::NotFound,
    default_message: "http resource not found",
};
