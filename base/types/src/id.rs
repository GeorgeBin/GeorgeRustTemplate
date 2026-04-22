use core::fmt;
use core::num::NonZeroU64;

use crate::InvalidIdError;

const fn ensure_non_zero(raw: u64) -> Result<NonZeroU64, InvalidIdError> {
    match NonZeroU64::new(raw) {
        Some(value) => Ok(value),
        None => Err(InvalidIdError::Zero),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HandleId(NonZeroU64);

impl HandleId {
    pub const fn new(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl From<NonZeroU64> for HandleId {
    fn from(value: NonZeroU64) -> Self {
        Self::new(value)
    }
}

impl From<HandleId> for u64 {
    fn from(value: HandleId) -> Self {
        value.get()
    }
}

impl TryFrom<u64> for HandleId {
    type Error = InvalidIdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::new(ensure_non_zero(value)?))
    }
}

impl fmt::Display for HandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RequestId(NonZeroU64);

impl RequestId {
    pub const fn new(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl From<NonZeroU64> for RequestId {
    fn from(value: NonZeroU64) -> Self {
        Self::new(value)
    }
}

impl From<RequestId> for u64 {
    fn from(value: RequestId) -> Self {
        value.get()
    }
}

impl TryFrom<u64> for RequestId {
    type Error = InvalidIdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::new(ensure_non_zero(value)?))
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InternalCorrelationId(NonZeroU64);

impl InternalCorrelationId {
    pub const fn new(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl From<NonZeroU64> for InternalCorrelationId {
    fn from(value: NonZeroU64) -> Self {
        Self::new(value)
    }
}

impl From<InternalCorrelationId> for u64 {
    fn from(value: InternalCorrelationId) -> Self {
        value.get()
    }
}

impl TryFrom<u64> for InternalCorrelationId {
    type Error = InvalidIdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::new(ensure_non_zero(value)?))
    }
}

impl fmt::Display for InternalCorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroU64;

    use super::{HandleId, InternalCorrelationId, RequestId};
    use crate::InvalidIdError;

    #[test]
    fn handle_id_try_from_non_zero_succeeds() {
        let id = HandleId::try_from(1).expect("handle id should be valid");

        assert_eq!(id.get(), 1);
    }

    #[test]
    fn handle_id_try_from_zero_fails() {
        let err = HandleId::try_from(0).expect_err("zero should be rejected");

        assert_eq!(err, InvalidIdError::Zero);
    }

    #[test]
    fn request_id_and_internal_correlation_id_get_return_raw_value() {
        let request_id = RequestId::try_from(42).expect("request id should be valid");
        let correlation_id =
            InternalCorrelationId::try_from(7).expect("correlation id should be valid");

        assert_eq!(request_id.get(), 42);
        assert_eq!(correlation_id.get(), 7);
    }

    #[test]
    fn display_outputs_plain_number() {
        let handle_id = HandleId::try_from(99).expect("handle id should be valid");
        let request_id = RequestId::try_from(100).expect("request id should be valid");
        let correlation_id =
            InternalCorrelationId::try_from(101).expect("correlation id should be valid");

        assert_eq!(handle_id.to_string(), "99");
        assert_eq!(request_id.to_string(), "100");
        assert_eq!(correlation_id.to_string(), "101");
    }

    #[test]
    fn from_non_zero_u64_constructs_all_ids() {
        let raw = NonZeroU64::new(8).expect("8 must be non-zero");

        let handle_id = HandleId::from(raw);
        let request_id = RequestId::from(raw);
        let correlation_id = InternalCorrelationId::from(raw);

        assert_eq!(handle_id.get(), 8);
        assert_eq!(request_id.get(), 8);
        assert_eq!(correlation_id.get(), 8);
    }

    #[test]
    fn ids_convert_back_to_u64() {
        let handle_id = HandleId::try_from(11).expect("handle id should be valid");
        let request_id = RequestId::try_from(12).expect("request id should be valid");
        let correlation_id =
            InternalCorrelationId::try_from(13).expect("correlation id should be valid");

        assert_eq!(u64::from(handle_id), 11);
        assert_eq!(u64::from(request_id), 12);
        assert_eq!(u64::from(correlation_id), 13);
    }
}
