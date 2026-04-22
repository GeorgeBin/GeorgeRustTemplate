use alloc::string::String;
use core::fmt;

use crate::EmptyStringError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(value: impl Into<String>) -> Result<Self, EmptyStringError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EmptyStringError::EmptyOrBlank);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = EmptyStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = EmptyStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.into_string()
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::NonEmptyString;
    use crate::EmptyStringError;

    #[test]
    fn try_from_str_succeeds_for_non_empty_text() {
        let value = NonEmptyString::try_from("demo").expect("demo should be valid");

        assert_eq!(value.as_str(), "demo");
    }

    #[test]
    fn try_from_empty_string_fails() {
        let err = NonEmptyString::try_from("").expect_err("empty text should fail");

        assert_eq!(err, EmptyStringError::EmptyOrBlank);
    }

    #[test]
    fn try_from_blank_string_fails() {
        let err = NonEmptyString::try_from("   ").expect_err("blank text should fail");

        assert_eq!(err, EmptyStringError::EmptyOrBlank);
    }

    #[test]
    fn as_str_returns_original_text() {
        let value = NonEmptyString::try_from("  demo  ").expect("text should be valid");

        assert_eq!(value.as_str(), "  demo  ");
    }

    #[test]
    fn into_string_returns_original_text() {
        let value = NonEmptyString::try_from("  demo  ").expect("text should be valid");

        assert_eq!(value.into_string(), "  demo  ");
    }

    #[test]
    fn display_outputs_original_text() {
        let value = NonEmptyString::try_from("demo").expect("text should be valid");

        assert_eq!(value.to_string(), "demo");
    }

    #[test]
    fn surrounding_whitespace_is_preserved() {
        let value = NonEmptyString::try_from("  demo  ").expect("text should be valid");

        assert_eq!(value.as_str(), "  demo  ");
    }

    #[test]
    fn as_ref_returns_original_text() {
        let value = NonEmptyString::try_from("  demo  ").expect("text should be valid");

        assert_eq!(value.as_ref(), "  demo  ");
    }

    #[test]
    fn from_non_empty_string_for_string_returns_original_text() {
        let value = NonEmptyString::try_from("  demo  ").expect("text should be valid");

        assert_eq!(String::from(value), "  demo  ");
    }
}
