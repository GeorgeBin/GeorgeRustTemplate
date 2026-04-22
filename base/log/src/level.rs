use core::fmt;

/// Represents the severity of one log record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Returns the stable lowercase string form of this level.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::LogLevel;

    #[test]
    fn as_str_matches_expected_values() {
        assert_eq!(LogLevel::Error.as_str(), "error");
        assert_eq!(LogLevel::Warn.as_str(), "warn");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Trace.as_str(), "trace");
    }

    #[test]
    fn display_matches_as_str() {
        assert_eq!(LogLevel::Info.to_string(), LogLevel::Info.as_str());
    }
}
