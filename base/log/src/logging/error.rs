use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogInitError {
    #[error("logging has already been initialized")]
    AlreadyInitialized,

    #[error("invalid log config: {0}")]
    InvalidConfig(String),

    #[error("failed to create log directory: {0}")]
    CreateDirectory(#[source] std::io::Error),

    #[error("failed to build file appender: {0}")]
    BuildFileAppender(#[source] std::io::Error),

    #[error("failed to install global subscriber: {0}")]
    SetGlobalSubscriber(String),
}
