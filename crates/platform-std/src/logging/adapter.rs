use george_base_log::{LogLevel, LogRecord, Logger, SharedLogger};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU8, Ordering},
    },
};
use tracing::{
    Event, Level, Metadata, callsite,
    field::{FieldSet, Value},
    metadata::Kind,
    subscriber::Interest,
};

// Dynamic tracing targets require runtime-created metadata/callsites instead of
// the usual macro-generated static callsites. We keep one leaked callsite per
// `(level, target, module_path, file, line)` combination for the process
// lifetime so events preserve their real tracing target. This intentionally
// trades bounded process memory growth for correct target-based filtering. In
// normal application code these values are usually `'static` and finite, so the
// cache should remain small and stable.
static CALLSITE_CACHE: OnceLock<Mutex<HashMap<CallsiteKey, &'static DynamicEventCallsite>>> =
    OnceLock::new();
static FIELD_NAMES: &[&str] = &["message", "module_path", "file", "line", "fields"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CallsiteKey {
    level: LogLevel,
    target: &'static str,
    module_path: Option<&'static str>,
    file: Option<&'static str>,
    line: Option<u32>,
}

struct DynamicEventCallsite {
    interest: AtomicU8,
    metadata: OnceLock<&'static Metadata<'static>>,
}

impl DynamicEventCallsite {
    const NEVER: u8 = 0;
    const SOMETIMES: u8 = 1;
    const ALWAYS: u8 = 2;

    fn new() -> Self {
        Self {
            interest: AtomicU8::new(Self::SOMETIMES),
            metadata: OnceLock::new(),
        }
    }

    fn metadata_ref(&self) -> &'static Metadata<'static> {
        self.metadata
            .get()
            .copied()
            .expect("dynamic event callsite metadata must be initialized")
    }
}

impl callsite::Callsite for DynamicEventCallsite {
    fn set_interest(&self, interest: Interest) {
        let value = if interest.is_never() {
            Self::NEVER
        } else if interest.is_always() {
            Self::ALWAYS
        } else {
            Self::SOMETIMES
        };

        self.interest.store(value, Ordering::Relaxed);
    }

    fn metadata(&self) -> &Metadata<'_> {
        self.metadata_ref()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TracingForwardLogger;

impl Logger for TracingForwardLogger {
    fn enabled(&self, _level: LogLevel, _target: &str) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        let fields = format_fields(record);
        let callsite = cached_callsite(record);
        let metadata = callsite.metadata_ref();
        let field_set = metadata.fields();

        let message_field = field_set.field("message").expect("message field");
        let module_path_field = field_set.field("module_path").expect("module_path field");
        let file_field = field_set.field("file").expect("file field");
        let line_field = field_set.field("line").expect("line field");
        let fields_field = field_set.field("fields").expect("fields field");

        let module_path = record.module_path.unwrap_or("");
        let file = record.file.unwrap_or("");
        let line = record.line.unwrap_or(0);

        let values = [
            (&message_field, Some(&record.message as &dyn Value)),
            (&module_path_field, Some(&module_path as &dyn Value)),
            (&file_field, Some(&file as &dyn Value)),
            (&line_field, Some(&line as &dyn Value)),
            (&fields_field, Some(&fields as &dyn Value)),
        ];
        let value_set = metadata.fields().value_set(&values);

        Event::dispatch(metadata, &value_set);
    }
}

pub fn shared_tracing_logger() -> SharedLogger {
    Arc::new(TracingForwardLogger)
}

fn cached_callsite(record: &LogRecord) -> &'static DynamicEventCallsite {
    let key = CallsiteKey {
        level: record.level,
        target: record.target,
        module_path: record.module_path,
        file: record.file,
        line: record.line,
    };

    let cache = CALLSITE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache = cache.lock().expect("callsite cache mutex poisoned");
    if let Some(callsite) = cache.get(&key) {
        return callsite;
    }

    let callsite = Box::leak(Box::new(DynamicEventCallsite::new()));
    let metadata = Box::leak(Box::new(Metadata::new(
        "george_base_log.record",
        record.target,
        tracing_level(record.level),
        record.file,
        record.line,
        record.module_path,
        FieldSet::new(FIELD_NAMES, callsite::Identifier(callsite)),
        Kind::EVENT,
    )));
    callsite
        .metadata
        .set(metadata)
        .expect("dynamic event callsite metadata set only once");
    callsite::register(callsite);
    cache.insert(key, callsite);
    callsite
}

fn tracing_level(level: LogLevel) -> Level {
    match level {
        LogLevel::Error => Level::ERROR,
        LogLevel::Warn => Level::WARN,
        LogLevel::Info => Level::INFO,
        LogLevel::Debug => Level::DEBUG,
        LogLevel::Trace => Level::TRACE,
    }
}

fn format_fields(record: &LogRecord) -> String {
    if record.fields.is_empty() {
        return String::new();
    }

    record
        .fields
        .iter()
        .map(|field| format!("{}={}", field.key, field.value))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::{TracingForwardLogger, shared_tracing_logger};
    use george_base_log::{LogLevel, LogRecord, Logger};

    #[test]
    fn tracing_forward_logger_log_with_real_target_does_not_panic() {
        let logger = TracingForwardLogger;
        let record =
            LogRecord::new(LogLevel::Info, "demo.runtime", "started").with_field("attempt", 1);

        logger.log(&record);
    }

    #[test]
    fn shared_tracing_logger_returns_trait_object() {
        let logger = shared_tracing_logger();
        let record = LogRecord::new(LogLevel::Warn, "demo", "warning");

        logger.log(&record);
    }
}
