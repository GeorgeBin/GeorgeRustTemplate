use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use crate::{ErrorContext, ErrorDescriptor, NativeError};

#[cfg(feature = "std")]
use alloc::boxed::Box;

/// Represents one runtime error instance built from a stable descriptor.
pub struct BaseError {
    pub desc: &'static ErrorDescriptor,
    pub detail: Option<String>,
    pub native: Option<NativeError>,
    pub context: Vec<ErrorContext>,
    #[cfg(feature = "std")]
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl BaseError {
    /// Creates a new runtime error instance from a static descriptor.
    pub fn new(desc: &'static ErrorDescriptor) -> Self {
        Self {
            desc,
            detail: None,
            native: None,
            context: Vec::new(),
            #[cfg(feature = "std")]
            source: None,
        }
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Attaches native error information from an underlying system.
    pub fn native<C, M>(mut self, source: &'static str, code: Option<C>, message: Option<M>) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        self.native = Some(NativeError::new(source, code, message));
        self
    }

    /// Attaches one structured context item to the error.
    pub fn context(mut self, key: &'static str, value: impl ToString) -> Self {
        self.context.push(ErrorContext::new(key, value));
        self
    }

    #[cfg(feature = "std")]
    /// Attaches a Rust source error for error chain integration.
    pub fn source<E>(mut self, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(err));
        self
    }

    /// Returns the stable error code from the descriptor.
    pub fn code(&self) -> crate::ErrorCode {
        self.desc.code
    }

    /// Returns the stable descriptor name.
    pub fn name(&self) -> &'static str {
        self.desc.name
    }

    /// Returns the high-level error kind from the descriptor.
    pub fn kind(&self) -> crate::ErrorKind {
        self.desc.kind
    }

    /// Returns the descriptor's default message.
    pub fn default_message(&self) -> &'static str {
        self.desc.default_message
    }
}

impl fmt::Debug for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code().to_string();
        let mut debug = f.debug_struct("BaseError");
        debug
            .field("code", &code)
            .field("name", &self.name())
            .field("kind", &self.kind())
            .field("default_message", &self.default_message())
            .field("detail", &self.detail)
            .field("native", &self.native)
            .field("context", &self.context);

        #[cfg(feature = "std")]
        {
            let has_source = self.source.is_some();
            debug.field("has_source", &has_source);
        }

        debug.finish()
    }
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}][{}] {}",
            self.code(),
            self.name(),
            self.default_message()
        )?;

        if let Some(detail) = &self.detail {
            write!(f, " | {detail}")?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|err| err as &(dyn std::error::Error + 'static))
    }
}
