use core::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InvalidIdError {
    Zero,
}

impl fmt::Display for InvalidIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("id must not be zero"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidIdError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EmptyStringError {
    EmptyOrBlank,
}

impl fmt::Display for EmptyStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOrBlank => f.write_str("string must not be empty or blank"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EmptyStringError {}

#[cfg(test)]
mod tests {
    use super::{EmptyStringError, InvalidIdError};

    #[test]
    fn invalid_id_error_display_is_stable() {
        assert_eq!(InvalidIdError::Zero.to_string(), "id must not be zero");
    }

    #[test]
    fn empty_string_error_display_is_stable() {
        assert_eq!(
            EmptyStringError::EmptyOrBlank.to_string(),
            "string must not be empty or blank"
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn errors_implement_std_error() {
        fn assert_error<E: std::error::Error>() {}

        assert_error::<InvalidIdError>();
        assert_error::<EmptyStringError>();
    }
}
