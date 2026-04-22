use george_base_log::LogLevel;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct StdLogConfig {
    pub enabled: bool,
    pub level: LogLevel,
    pub console: ConsoleLogConfig,
    pub file: FileLogConfig,
    pub cleanup: CleanupConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct ConsoleLogConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct FileLogConfig {
    pub enabled: bool,
    pub directory: PathBuf,
    pub file_prefix: String,
}

#[derive(Debug, Clone, Copy)]
pub struct CleanupConfig {
    pub enabled: bool,
    pub max_retention_days: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeLogConfig {
    pub enabled: bool,
    pub level: LogLevel,
    pub console_enabled: bool,
    pub file_enabled: bool,
}

impl From<&StdLogConfig> for RuntimeLogConfig {
    fn from(config: &StdLogConfig) -> Self {
        Self {
            enabled: config.enabled,
            level: config.level,
            console_enabled: config.console.enabled,
            file_enabled: config.file.enabled,
        }
    }
}
