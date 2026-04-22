use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtpRequest {
    pub server: String,
}

#[derive(Debug, Clone)]
pub struct NtpResponse {
    pub server_time: SystemTime,
}
