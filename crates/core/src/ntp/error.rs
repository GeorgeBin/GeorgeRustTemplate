use core::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NtpCoreError {
    InvalidRequest,
    InvalidResponse,
    Timeout,
    TransportUnavailable,
    UpstreamFailed,
}

impl fmt::Display for NtpCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest => f.write_str("invalid request"),
            Self::InvalidResponse => f.write_str("invalid response"),
            Self::Timeout => f.write_str("timeout"),
            Self::TransportUnavailable => f.write_str("transport unavailable"),
            Self::UpstreamFailed => f.write_str("upstream failed"),
        }
    }
}

impl std::error::Error for NtpCoreError {}
