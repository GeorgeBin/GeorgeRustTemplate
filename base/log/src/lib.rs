#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod ext;
mod field;
mod level;
mod logger;
mod noop;
mod record;

pub use ext::LoggerExt;
pub use field::LogField;
pub use level::LogLevel;
pub use logger::{Logger, SharedLogger};
pub use noop::NoopLogger;
pub use record::LogRecord;
