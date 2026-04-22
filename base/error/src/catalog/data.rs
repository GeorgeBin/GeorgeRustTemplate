use crate::{ErrorCode, ErrorDescriptor, ErrorKind};

pub const DATA_JSON_PARSE_FAILED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(5, 1, 1),
    name: "DATA_JSON_PARSE_FAILED",
    kind: ErrorKind::Parse,
    default_message: "json parse failed",
};

pub const DATA_PROTOCOL_DECODE_FAILED: ErrorDescriptor = ErrorDescriptor {
    code: ErrorCode::from_parts(5, 2, 1),
    name: "DATA_PROTOCOL_DECODE_FAILED",
    kind: ErrorKind::Decode,
    default_message: "protocol decode failed",
};
