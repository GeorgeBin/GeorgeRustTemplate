use std::time::{SystemTime, UNIX_EPOCH};

#[uniffi::export]
pub fn is_valid_ipv4(ip: String) -> bool {
    corelib::utils::is_valid_ipv4(ip)
}

#[derive(Debug, uniffi::Error)]
pub enum NtpError {
    Io { io_message: String },
    InvalidResponse,
    Unknown,
}

impl std::fmt::Display for NtpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NtpError::Io { io_message } => write!(f, "IO error: {io_message}"),
            NtpError::InvalidResponse => write!(f, "NTP response invalid"),
            NtpError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl From<corelib::ntp::NtpError> for NtpError {
    fn from(err: corelib::ntp::NtpError) -> Self {
        match err {
            corelib::ntp::NtpError::Io(e) => NtpError::Io {
                io_message: e.to_string(),
            },
            corelib::ntp::NtpError::InvalidResponse => NtpError::InvalidResponse,
            corelib::ntp::NtpError::Unknown => NtpError::Unknown,
        }
    }
}

#[uniffi::export(callback_interface)]
pub trait NtpCallback: Send + Sync {
    fn on_success(&self, unix_millis: i64);
    fn on_error(&self, error: NtpError);
}

#[uniffi::export]
pub fn ntp_sync(server: String, callback: Box<dyn NtpCallback>) {
    let server = normalize_ntp_server(&server);
    std::thread::spawn(move || {
        let client = corelib::ntp::NtpClient::new(&server);
        match client.sync_time() {
            Ok(time) => {
                let unix_millis = system_time_to_unix_millis(time);
                callback.on_success(unix_millis);
            }
            Err(err) => {
                callback.on_error(err.into());
            }
        }
    });
}

fn normalize_ntp_server(server: &str) -> String {
    if server.contains(':') {
        server.to_string()
    } else {
        format!("{server}:123")
    }
}

fn system_time_to_unix_millis(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as i64,
        Err(_) => 0,
    }
}
