use super::cleanup::cleanup_old_logs;
use super::config::{FileLogConfig, RuntimeLogConfig, StdLogConfig};
use super::error::StdLogInstallError;
use super::file::build_file_appender;
use george_base_log::LogLevel;
use std::io::{Result as IoResult, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use tracing::{Level, Metadata, Subscriber};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::layer::{Filter, Layer};

static LOGGING_RUNTIME: OnceLock<StdLoggingHandle> = OnceLock::new();

#[derive(Clone)]
struct SwapWriter {
    inner: Arc<Mutex<NonBlocking>>,
}

impl Write for SwapWriter {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.inner
            .lock()
            .expect("logging writer mutex poisoned")
            .write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.inner
            .lock()
            .expect("logging writer mutex poisoned")
            .flush()
    }
}

impl<'a> fmt::MakeWriter<'a> for SwapWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[derive(Clone)]
struct RuntimeState {
    enabled: Arc<AtomicBool>,
    level: Arc<AtomicU8>,
    console_enabled: Arc<AtomicBool>,
    file_enabled: Arc<AtomicBool>,
}

impl RuntimeState {
    fn from_runtime_config(config: RuntimeLogConfig) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(config.enabled)),
            level: Arc::new(AtomicU8::new(level_to_u8(config.level))),
            console_enabled: Arc::new(AtomicBool::new(config.console_enabled)),
            file_enabled: Arc::new(AtomicBool::new(config.file_enabled)),
        }
    }

    fn current_runtime_config(&self) -> RuntimeLogConfig {
        RuntimeLogConfig {
            enabled: self.enabled.load(Ordering::Relaxed),
            level: u8_to_log_level(self.level.load(Ordering::Relaxed)),
            console_enabled: self.console_enabled.load(Ordering::Relaxed),
            file_enabled: self.file_enabled.load(Ordering::Relaxed),
        }
    }
}

#[derive(Clone, Copy)]
enum OutputKind {
    Console,
    File,
}

struct RuntimeFilter {
    state: RuntimeState,
    output: OutputKind,
}

impl RuntimeFilter {
    fn output_enabled(&self) -> bool {
        match self.output {
            OutputKind::Console => self.state.console_enabled.load(Ordering::Relaxed),
            OutputKind::File => self.state.file_enabled.load(Ordering::Relaxed),
        }
    }

    fn level(&self) -> Level {
        u8_to_level(self.state.level.load(Ordering::Relaxed))
    }
}

impl<S: Subscriber> Filter<S> for RuntimeFilter {
    fn enabled(
        &self,
        metadata: &Metadata<'_>,
        _ctx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        self.state.enabled.load(Ordering::Relaxed)
            && self.output_enabled()
            && metadata.level() <= &self.level()
    }
}

/// Runtime handle for the process-wide std/tracing logging backend.
///
/// This handle exists only after the application installs the global tracing
/// subscriber. It is intended for binaries and app shells; library crates
/// should not install or own process-wide logging during normal operation.
pub struct StdLoggingHandle {
    state: RuntimeState,
    file_config: Mutex<FileLogConfig>,
    file_writer: Arc<Mutex<NonBlocking>>,
    file_guard: Arc<Mutex<WorkerGuard>>,
}

impl StdLoggingHandle {
    pub fn apply_runtime_config(&self, config: RuntimeLogConfig) -> Result<(), StdLogInstallError> {
        validate_runtime_config(config)?;

        let current = self.current_runtime_config();
        if !current.file_enabled && config.file_enabled {
            self.enable_file_output()?;
        } else if current.file_enabled && !config.file_enabled {
            self.disable_file_output();
        }

        self.state.enabled.store(config.enabled, Ordering::Relaxed);
        self.state
            .level
            .store(level_to_u8(config.level), Ordering::Relaxed);
        self.state
            .console_enabled
            .store(config.console_enabled, Ordering::Relaxed);
        self.state
            .file_enabled
            .store(config.file_enabled, Ordering::Relaxed);

        Ok(())
    }

    pub fn current_runtime_config(&self) -> RuntimeLogConfig {
        self.state.current_runtime_config()
    }

    pub fn update_file_directory(&self, directory: PathBuf) -> Result<(), StdLogInstallError> {
        let mut next_config = self
            .file_config
            .lock()
            .expect("logging file config mutex poisoned")
            .clone();
        next_config.directory = directory;

        if self.state.file_enabled.load(Ordering::Relaxed) {
            let appender = build_file_appender(&next_config)?;
            let (non_blocking, guard) = tracing_appender::non_blocking(appender);

            *self
                .file_writer
                .lock()
                .expect("logging writer mutex poisoned") = non_blocking;
            *self
                .file_guard
                .lock()
                .expect("logging guard mutex poisoned") = guard;
        }

        *self
            .file_config
            .lock()
            .expect("logging file config mutex poisoned") = next_config;

        Ok(())
    }

    fn enable_file_output(&self) -> Result<(), StdLogInstallError> {
        let appender = {
            let config = self
                .file_config
                .lock()
                .expect("logging file config mutex poisoned")
                .clone();
            build_file_appender(&config)?
        };
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);

        *self
            .file_writer
            .lock()
            .expect("logging writer mutex poisoned") = non_blocking;
        *self
            .file_guard
            .lock()
            .expect("logging guard mutex poisoned") = guard;

        Ok(())
    }

    fn disable_file_output(&self) {
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::sink());
        *self
            .file_writer
            .lock()
            .expect("logging writer mutex poisoned") = non_blocking;
        *self
            .file_guard
            .lock()
            .expect("logging guard mutex poisoned") = guard;
    }
}

/// Installs the process-wide global tracing subscriber for the std backend.
///
/// This function is intended for binaries, demos, and app shells. Library
/// crates should not call this during normal operation.
pub fn install_global_tracing(
    config: StdLogConfig,
) -> Result<&'static StdLoggingHandle, StdLogInstallError> {
    if LOGGING_RUNTIME.get().is_some() {
        return Err(StdLogInstallError::AlreadyInitialized);
    }

    validate_init_config(&config)?;

    if config.file.enabled {
        cleanup_old_logs(&config.file, &config.cleanup)?;
    }

    let runtime = RuntimeState::from_runtime_config(RuntimeLogConfig::from(&config));

    let console_layer = fmt::layer().with_target(false).with_filter(RuntimeFilter {
        state: runtime.clone(),
        output: OutputKind::Console,
    });

    let (file_writer, initial_guard) = if config.file.enabled {
        let appender = build_file_appender(&config.file)?;
        tracing_appender::non_blocking(appender)
    } else {
        tracing_appender::non_blocking(std::io::sink())
    };

    let swap_writer = SwapWriter {
        inner: Arc::new(Mutex::new(file_writer)),
    };

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_target(true)
        .with_writer(swap_writer.clone())
        .with_filter(RuntimeFilter {
            state: runtime.clone(),
            output: OutputKind::File,
        });

    let subscriber = tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|err| StdLogInstallError::SetGlobalSubscriber(err.to_string()))?;

    let handle = StdLoggingHandle {
        state: runtime,
        file_config: Mutex::new(config.file.clone()),
        file_writer: swap_writer.inner,
        file_guard: Arc::new(Mutex::new(initial_guard)),
    };

    LOGGING_RUNTIME
        .set(handle)
        .map_err(|_| StdLogInstallError::AlreadyInitialized)?;

    Ok(LOGGING_RUNTIME
        .get()
        .expect("logging handle must be available after initialization"))
}

/// Returns the installed process-wide std logging handle, if initialization has run.
///
/// This accessor is intended for binaries and app shells coordinating global
/// logging state. Library crates should not depend on process-wide logging
/// initialization.
pub fn global_logging() -> Option<&'static StdLoggingHandle> {
    LOGGING_RUNTIME.get()
}

fn validate_init_config(config: &StdLogConfig) -> Result<(), StdLogInstallError> {
    validate_runtime_config(RuntimeLogConfig::from(config))?;

    if config.cleanup.enabled && config.cleanup.max_retention_days < 1 {
        return Err(StdLogInstallError::InvalidConfig(
            "max_retention_days must be >= 1 when cleanup is enabled".to_string(),
        ));
    }

    if config.file.file_prefix.trim().is_empty() {
        return Err(StdLogInstallError::InvalidConfig(
            "file_prefix must not be empty".to_string(),
        ));
    }

    Ok(())
}

fn validate_runtime_config(config: RuntimeLogConfig) -> Result<(), StdLogInstallError> {
    if config.enabled && !config.console_enabled && !config.file_enabled {
        return Err(StdLogInstallError::InvalidConfig(
            "enabled=true requires console or file logging to be enabled".to_string(),
        ));
    }

    Ok(())
}

fn level_to_u8(level: LogLevel) -> u8 {
    match level {
        LogLevel::Error => 1,
        LogLevel::Warn => 2,
        LogLevel::Info => 3,
        LogLevel::Debug => 4,
        LogLevel::Trace => 5,
    }
}

fn u8_to_level(level: u8) -> Level {
    match level {
        1 => Level::ERROR,
        2 => Level::WARN,
        3 => Level::INFO,
        4 => Level::DEBUG,
        5 => Level::TRACE,
        _ => Level::INFO,
    }
}

fn u8_to_log_level(level: u8) -> LogLevel {
    match level {
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        3 => LogLevel::Info,
        4 => LogLevel::Debug,
        5 => LogLevel::Trace,
        _ => LogLevel::Info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::{CleanupConfig, ConsoleLogConfig, FileLogConfig};

    fn config() -> StdLogConfig {
        StdLogConfig {
            enabled: true,
            level: LogLevel::Info,
            console: ConsoleLogConfig { enabled: true },
            file: FileLogConfig {
                enabled: false,
                directory: PathBuf::from("./logs"),
                file_prefix: "test".to_string(),
            },
            cleanup: CleanupConfig {
                enabled: false,
                max_retention_days: 7,
            },
        }
    }

    fn runtime_config() -> RuntimeLogConfig {
        RuntimeLogConfig {
            enabled: true,
            level: LogLevel::Info,
            console_enabled: true,
            file_enabled: false,
        }
    }

    fn test_handle() -> StdLoggingHandle {
        let state = RuntimeState::from_runtime_config(runtime_config());
        let (writer, guard) = tracing_appender::non_blocking(std::io::sink());

        StdLoggingHandle {
            state,
            file_config: Mutex::new(FileLogConfig {
                enabled: false,
                directory: std::env::temp_dir(),
                file_prefix: "test".to_string(),
            }),
            file_writer: Arc::new(Mutex::new(writer)),
            file_guard: Arc::new(Mutex::new(guard)),
        }
    }

    #[test]
    fn rejects_enabled_without_any_output() {
        let mut cfg = config();
        cfg.console.enabled = false;

        let result = validate_init_config(&cfg);

        assert!(matches!(result, Err(StdLogInstallError::InvalidConfig(_))));
    }

    #[test]
    fn rejects_invalid_cleanup_days() {
        let mut cfg = config();
        cfg.cleanup.enabled = true;
        cfg.cleanup.max_retention_days = 0;

        let result = validate_init_config(&cfg);

        assert!(matches!(result, Err(StdLogInstallError::InvalidConfig(_))));
    }

    #[test]
    fn rejects_empty_file_prefix() {
        let mut cfg = config();
        cfg.file.file_prefix.clear();

        let result = validate_init_config(&cfg);

        assert!(matches!(result, Err(StdLogInstallError::InvalidConfig(_))));
    }

    #[test]
    fn rejects_invalid_runtime_config() {
        let invalid = RuntimeLogConfig {
            enabled: true,
            level: LogLevel::Info,
            console_enabled: false,
            file_enabled: false,
        };

        let result = validate_runtime_config(invalid);

        assert!(matches!(result, Err(StdLogInstallError::InvalidConfig(_))));
    }

    #[test]
    fn current_runtime_config_reflects_updates() {
        let handle = test_handle();
        let updated = RuntimeLogConfig {
            enabled: false,
            level: LogLevel::Debug,
            console_enabled: true,
            file_enabled: false,
        };

        handle
            .apply_runtime_config(updated)
            .expect("runtime config update should succeed");

        assert_eq!(handle.current_runtime_config(), updated);
    }

    #[test]
    fn update_file_directory_replaces_stored_path() {
        let handle = test_handle();
        let next = std::env::temp_dir().join("platform-std-logging-next");

        handle
            .update_file_directory(next.clone())
            .expect("file directory update should succeed");

        let stored = handle
            .file_config
            .lock()
            .expect("logging file config mutex poisoned")
            .directory
            .clone();
        assert_eq!(stored, next);
    }
}
