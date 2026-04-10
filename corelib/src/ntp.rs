//! base/lib.rs
//!
//! 这是跨平台 NTP 客户端的核心逻辑模块。
//! 不包含任何 UI 或平台特定代码。

use std::io;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// NTP 协议常量
const NTP_TIMESTAMP_DELTA: u64 = 2208988800; // 1970 -> 1900

/// NTP 客户端结构体
#[derive(Debug, Clone)]
pub struct NtpClient {
    pub server: String,
    pub timeout: Duration,
}

impl NtpClient {
    /// 创建新的 NtpClient
    pub fn new(server: &str) -> Self {
        Self {
            server: server.to_string(),
            timeout: Duration::from_secs(3),
        }
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 同步时间（发送 NTP 请求并返回服务器时间）
    pub fn sync_time(&self) -> Result<SystemTime, NtpError> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(self.timeout))?;
        socket.connect(&self.server)?;

        // 构造 NTP 请求数据（48 字节）
        let mut packet = [0u8; 48];
        packet[0] = 0b11100011; // LI, Version, Mode
        socket.send(&packet)?;

        // 接收响应
        let mut buffer = [0u8; 48];
        socket.recv(&mut buffer)?;

        parse_transmit_timestamp(&buffer)
    }
}

pub fn parse_transmit_timestamp(packet: &[u8]) -> Result<SystemTime, NtpError> {
    if packet.len() < 48 {
        return Err(NtpError::InvalidResponse);
    }

    // 解析 40~43 字节：Transmit Timestamp seconds
    let secs = u32::from_be_bytes(
        packet[40..44]
            .try_into()
            .map_err(|_| NtpError::InvalidResponse)?,
    ) as u64;

    if secs < NTP_TIMESTAMP_DELTA {
        return Err(NtpError::InvalidResponse);
    }

    let unix_secs = secs - NTP_TIMESTAMP_DELTA;
    Ok(UNIX_EPOCH + Duration::from_secs(unix_secs))
}

/// 错误类型
#[derive(Error, Debug)]
pub enum NtpError {
    #[error("IO 错误: {0}")]
    Io(#[from] io::Error),

    #[error("NTP 响应无效")]
    InvalidResponse,

    #[error("未知错误")]
    Unknown,
}

pub const DEFAULT_NTP_SERVER: &str = "ntp.aliyun.com:123";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_transmit_timestamp() {
        let mut packet = [0u8; 48];
        let unix_secs = 1_700_000_000u64;
        let ntp_secs = unix_secs + NTP_TIMESTAMP_DELTA;
        packet[40..44].copy_from_slice(&(ntp_secs as u32).to_be_bytes());

        let parsed = parse_transmit_timestamp(&packet).unwrap();
        let parsed_secs = parsed.duration_since(UNIX_EPOCH).unwrap().as_secs();

        assert_eq!(parsed_secs, unix_secs);
    }

    #[test]
    fn rejects_short_packets() {
        let packet = [0u8; 47];
        let result = parse_transmit_timestamp(&packet);

        assert!(matches!(result, Err(NtpError::InvalidResponse)));
    }

    #[test]
    fn rejects_timestamps_before_unix_epoch() {
        let mut packet = [0u8; 48];
        packet[40..44].copy_from_slice(&1u32.to_be_bytes());

        let result = parse_transmit_timestamp(&packet);

        assert!(matches!(result, Err(NtpError::InvalidResponse)));
    }
}
