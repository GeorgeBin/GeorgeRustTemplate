pub mod cleanup;
pub mod config;
pub mod error;
pub mod file;
pub mod init;

pub use cleanup::cleanup_old_logs;
pub use config::{
    CleanupConfig, ConsoleLogConfig, FileLogConfig, LogConfig, LogLevel, RuntimeLogConfig,
};
pub use error::LogInitError;
pub use init::{LoggingHandle, init_logging, logging};
