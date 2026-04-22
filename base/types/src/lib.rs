#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod id;
mod text;

pub use error::{EmptyStringError, InvalidIdError};
pub use id::{CorrelationId, HandleId, RequestId};
pub use text::NonEmptyString;
