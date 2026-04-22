use core::fmt;

use template_model::ntp::NtpRequest;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NtpTransportError {
    Timeout,
    Unavailable,
    UpstreamFailed,
}

impl fmt::Display for NtpTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => f.write_str("timeout"),
            Self::Unavailable => f.write_str("transport unavailable"),
            Self::UpstreamFailed => f.write_str("upstream failed"),
        }
    }
}

impl std::error::Error for NtpTransportError {}

pub trait NtpTransport {
    fn query(&self, request: &NtpRequest) -> Result<[u8; 48], NtpTransportError>;
}
