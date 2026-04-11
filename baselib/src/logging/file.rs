use super::config::FileLogConfig;
use super::error::LogInitError;
use chrono::{Local, NaiveDate};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

type CurrentDateFn = Arc<dyn Fn() -> NaiveDate + Send + Sync>;

pub struct LocalDateRollingFileAppender {
    state: AppenderState,
    current_date: CurrentDateFn,
}

struct AppenderState {
    directory: PathBuf,
    file_prefix: String,
    active_date: NaiveDate,
    file: File,
}

pub fn ensure_log_directory(path: &Path) -> Result<(), LogInitError> {
    std::fs::create_dir_all(path).map_err(LogInitError::CreateDirectory)
}

pub fn build_file_appender(config: &FileLogConfig) -> Result<LocalDateRollingFileAppender, LogInitError> {
    LocalDateRollingFileAppender::new(config)
}

impl LocalDateRollingFileAppender {
    pub fn new(config: &FileLogConfig) -> Result<Self, LogInitError> {
        Self::with_current_date(config, Arc::new(|| Local::now().date_naive()))
    }

    fn with_current_date(
        config: &FileLogConfig,
        current_date: CurrentDateFn,
    ) -> Result<Self, LogInitError> {
        if config.file_prefix.trim().is_empty() {
            return Err(LogInitError::InvalidConfig(
                "file_prefix must not be empty when file logging is enabled".to_string(),
            ));
        }

        ensure_log_directory(&config.directory)?;

        let active_date = current_date();
        let file = open_log_file(&config.directory, &config.file_prefix, active_date)
            .map_err(LogInitError::BuildFileAppender)?;

        Ok(Self {
            state: AppenderState {
                directory: config.directory.clone(),
                file_prefix: config.file_prefix.clone(),
                active_date,
                file,
            },
            current_date,
        })
    }

    fn maybe_rollover(&mut self) -> io::Result<()> {
        let current_date = (self.current_date)();
        if current_date == self.state.active_date {
            return Ok(());
        }

        let next_file = open_log_file(
            &self.state.directory,
            &self.state.file_prefix,
            current_date,
        )?;
        self.state.file.flush()?;
        self.state.file = next_file;
        self.state.active_date = current_date;
        Ok(())
    }
}

impl Write for LocalDateRollingFileAppender {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.maybe_rollover()?;
        self.state.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.state.file.flush()
    }
}

fn open_log_file(directory: &Path, prefix: &str, date: NaiveDate) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(directory.join(log_file_name(prefix, date)))
}

fn log_file_name(prefix: &str, date: NaiveDate) -> String {
    format!("{prefix}.{}.log", date.format("%Y-%m-%d"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn test_config(directory: PathBuf) -> FileLogConfig {
        FileLogConfig {
            enabled: true,
            directory,
            file_prefix: "demo".to_string(),
        }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let unique = format!(
            "baselib-local-log-{name}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

    #[test]
    fn keeps_writing_to_same_file_within_same_local_day() {
        let directory = temp_dir("same-day");
        let today = NaiveDate::from_ymd_opt(2026, 4, 11).expect("valid date");
        let current_date = Arc::new(move || today);
        let mut appender =
            LocalDateRollingFileAppender::with_current_date(&test_config(directory.clone()), current_date)
                .expect("appender should initialize");

        writeln!(appender, "first").expect("first write");
        writeln!(appender, "second").expect("second write");

        let content = std::fs::read_to_string(directory.join("demo.2026-04-11.log"))
            .expect("log file should exist");
        assert!(content.contains("first"));
        assert!(content.contains("second"));

        let _ = std::fs::remove_dir_all(directory);
    }

    #[test]
    fn rolls_over_when_local_date_changes() {
        let directory = temp_dir("rollover");
        let current = Arc::new(Mutex::new(
            NaiveDate::from_ymd_opt(2026, 4, 11).expect("valid date"),
        ));
        let current_date = {
            let current = current.clone();
            Arc::new(move || *current.lock().expect("clock mutex poisoned"))
        };
        let mut appender =
            LocalDateRollingFileAppender::with_current_date(&test_config(directory.clone()), current_date)
                .expect("appender should initialize");

        writeln!(appender, "before-midnight").expect("first write");
        *current.lock().expect("clock mutex poisoned") =
            NaiveDate::from_ymd_opt(2026, 4, 12).expect("valid date");
        writeln!(appender, "after-midnight").expect("second write");

        let first = std::fs::read_to_string(directory.join("demo.2026-04-11.log"))
            .expect("first file should exist");
        let second = std::fs::read_to_string(directory.join("demo.2026-04-12.log"))
            .expect("second file should exist");
        assert!(first.contains("before-midnight"));
        assert!(!first.contains("after-midnight"));
        assert!(second.contains("after-midnight"));

        let _ = std::fs::remove_dir_all(directory);
    }
}
