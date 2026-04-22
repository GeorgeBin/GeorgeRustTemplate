#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod code;
mod context;
mod descriptor;
mod error;
mod kind;
mod native;
mod result;

pub mod catalog;

#[cfg(test)]
mod tests;

pub use code::ErrorCode;
pub use context::ErrorContext;
pub use descriptor::ErrorDescriptor;
pub use error::BaseError;
pub use kind::ErrorKind;
pub use native::NativeError;
pub use result::Result;
