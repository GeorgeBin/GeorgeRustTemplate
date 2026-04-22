use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Upstream,
    InvalidInput,
    InvalidState,
    Timeout,
    Cancelled,
    NotFound,
    Conflict,
    PermissionDenied,
    Unavailable,
    Parse,
    Encode,
    Decode,
    Verify,
    Internal,
}

impl ErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Upstream => "upstream",
            Self::InvalidInput => "invalid_input",
            Self::InvalidState => "invalid_state",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::PermissionDenied => "permission_denied",
            Self::Unavailable => "unavailable",
            Self::Parse => "parse",
            Self::Encode => "encode",
            Self::Decode => "decode",
            Self::Verify => "verify",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
