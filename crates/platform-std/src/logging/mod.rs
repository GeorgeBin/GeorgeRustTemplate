mod adapter;
mod cleanup;
mod config;
mod error;
mod file;
mod init;

pub use adapter::{TracingForwardLogger, shared_tracing_logger};
pub use cleanup::cleanup_old_logs;
pub use config::{CleanupConfig, ConsoleLogConfig, FileLogConfig, RuntimeLogConfig, StdLogConfig};
pub use error::StdLogInstallError;
pub use file::build_file_appender;
pub use init::{StdLoggingHandle, global_logging, install_global_tracing};
