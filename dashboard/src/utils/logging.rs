use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{
    fmt,
    prelude::*,
    layer::{Layer, Filter},
};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::io::{Write, Result};
use tracing::{Metadata, Level, Subscriber};
use std::path::Path;

pub struct SwapWriter {
    inner: Arc<Mutex<NonBlocking>>,
}

impl Write for SwapWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.lock().unwrap().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.inner.lock().unwrap().flush()
    }
}

impl<'a> fmt::MakeWriter<'a> for SwapWriter {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        Self { inner: self.inner.clone() }
    }
}

// Custom dynamic log level filter
pub struct DynamicLevelFilter {
    current_level: Arc<AtomicU8>,
}

impl DynamicLevelFilter {
    fn get_level(&self) -> Level {
        map_level(self.current_level.load(Ordering::Relaxed))
    }
}

impl<S: Subscriber> Filter<S> for DynamicLevelFilter {
    fn enabled(&self, metadata: &Metadata<'_>, _ctx: &tracing_subscriber::layer::Context<'_, S>) -> bool {
        metadata.level() <= &self.get_level()
    }
}

pub struct LoggingSystem {
    pub writer: SwapWriter,
    pub guard: Arc<Mutex<WorkerGuard>>,
    pub level: Arc<AtomicU8>,
}

pub fn init_logging(log_dir: &str, level_num: u8) -> LoggingSystem {
    let level_atomic = Arc::new(AtomicU8::new(level_num));
    
    // Terminal output
    let stdout_layer = fmt::layer()
        .with_target(false)
        .with_filter(DynamicLevelFilter { current_level: level_atomic.clone() });
    
    // File output (fall back to temp on permission errors)
    let (non_blocking, guard) = match build_file_appender(log_dir) {
        Ok(appender) => tracing_appender::non_blocking(appender),
        Err(primary_err) => {
            let fallback_dir = std::env::temp_dir()
                .join("dashboard")
                .join("logs");
            match build_file_appender(fallback_dir.to_string_lossy().as_ref()) {
                Ok(appender) => tracing_appender::non_blocking(appender),
                Err(fallback_err) => {
                    eprintln!(
                        "Logging disabled: failed to create log directory. primary: {}; fallback: {}",
                        primary_err, fallback_err
                    );
                    tracing_appender::non_blocking(std::io::sink())
                }
            }
        }
    };
    
    let swap_writer = SwapWriter { inner: Arc::new(Mutex::new(non_blocking)) };
    let guard_holder = Arc::new(Mutex::new(guard));

    let file_layer = fmt::layer()
        .with_writer(swap_writer.clone())
        .with_ansi(false)
        .with_target(true)
        .with_filter(DynamicLevelFilter { current_level: level_atomic.clone() });

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    LoggingSystem { 
        writer: swap_writer, 
        guard: guard_holder,
        level: level_atomic,
    }
}

fn map_level(level_num: u8) -> Level {
    match level_num {
        1 => Level::ERROR,
        2 => Level::WARN,
        3 => Level::INFO,
        4 => Level::DEBUG,
        5 => Level::TRACE,
        _ => Level::INFO,
    }
}

impl LoggingSystem {
    pub fn update_path(&self, log_dir: &str) {
        let file_appender = match build_file_appender(log_dir) {
            Ok(appender) => appender,
            Err(err) => {
                tracing::error!("Failed to update log directory '{}': {}", log_dir, err);
                return;
            }
        };
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        
        *self.writer.inner.lock().unwrap() = non_blocking;
        *self.guard.lock().unwrap() = guard;
        tracing::info!("Log directory updated to: {}", log_dir);
    }

    pub fn update_level(&self, level_num: u8) {
        self.level.store(level_num, Ordering::Relaxed);
        let level = map_level(level_num);
        tracing::info!("Log level updated to: {:?}", level);
    }
}

impl Clone for SwapWriter {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

fn build_file_appender(log_dir: &str) -> std::io::Result<RollingFileAppender> {
    std::fs::create_dir_all(log_dir)?;
    RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("dashboard")
        .filename_suffix("log")
        .build(log_dir)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn cleanup_expired_logs(log_dir: &str, log_days: u8) {
    let log_path = Path::new(log_dir);
    if !log_path.exists() || !log_path.is_dir() {
        return;
    }

    let now = chrono::Local::now().date_naive();
    let expiration_date = now - chrono::TimeDelta::days(log_days as i64);

    if let Ok(entries) = std::fs::read_dir(log_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            if !file_name.starts_with("dashboard.") || !file_name.ends_with(".log") {
                continue;
            }

            let date_part = file_name
                .trim_start_matches("dashboard.")
                .trim_end_matches(".log");

            if let Ok(file_date) = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                if file_date <= expiration_date {
                    tracing::info!("Deleting expired log file: {:?}", file_name);
                    let _ = std::fs::remove_file(path);
                }
            }
        }
    }
}
