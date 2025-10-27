//! base/lib.rs
//!
//! 这是跨平台 NTP 客户端的核心逻辑模块。
//! 不包含任何 UI 或平台特定代码。

use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io;
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

        // 解析 40~43 字节：Transmit Timestamp seconds
        let secs = u32::from_be_bytes(buffer[40..44].try_into().unwrap()) as u64;
        let ntp_time = secs - NTP_TIMESTAMP_DELTA;

        Ok(UNIX_EPOCH + Duration::from_secs(ntp_time))
    }
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


pub const DEFAULT_NTP_SERVER: &str = "pool.ntp.org:123";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_time() {
        let client = NtpClient::new(DEFAULT_NTP_SERVER);
        let result = client.sync_time();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
