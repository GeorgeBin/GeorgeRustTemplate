use super::config::FileLogConfig;
use super::error::LogInitError;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn ensure_log_directory(path: &std::path::Path) -> Result<(), LogInitError> {
    std::fs::create_dir_all(path).map_err(LogInitError::CreateDirectory)
}

pub fn build_file_appender(config: &FileLogConfig) -> Result<RollingFileAppender, LogInitError> {
    if config.file_prefix.trim().is_empty() {
        return Err(LogInitError::InvalidConfig(
            "file_prefix must not be empty when file logging is enabled".to_string(),
        ));
    }

    ensure_log_directory(&config.directory)?;

    RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(&config.file_prefix)
        .filename_suffix("log")
        .build(&config.directory)
        .map_err(|err| LogInitError::BuildFileAppender(std::io::Error::other(err)))
}
