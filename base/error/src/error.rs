use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use crate::{ErrorContext, ErrorDescriptor, NativeError};

#[cfg(feature = "std")]
use alloc::boxed::Box;

pub struct BaseError {
    pub desc: &'static ErrorDescriptor,
    pub detail: Option<String>,
    pub native: Option<NativeError>,
    pub context: Vec<ErrorContext>,
    #[cfg(feature = "std")]
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl BaseError {
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

    pub fn native<C, M>(mut self, source: &'static str, code: Option<C>, message: Option<M>) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        self.native = Some(NativeError {
            source,
            code: code.map(Into::into),
            message: message.map(Into::into),
        });
        self
    }

    pub fn context(mut self, key: &'static str, value: impl ToString) -> Self {
        self.context.push(ErrorContext {
            key,
            value: value.to_string(),
        });
        self
    }

    #[cfg(feature = "std")]
    pub fn source<E>(mut self, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(err));
        self
    }
}

impl fmt::Debug for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("BaseError");
        debug
            .field("desc", &self.desc)
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
        if let Some(detail) = &self.detail {
            f.write_str(detail)
        } else {
            f.write_str(self.desc.default_message)
        }
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
