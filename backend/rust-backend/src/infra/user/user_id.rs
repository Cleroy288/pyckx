//! UserId - Identifies users from Supabase Auth

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(String);

impl UserId {
    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for UserId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> String {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_preserves_value() {
        // arrange
        let raw = "user-abc-123".to_string();

        // act
        let id = UserId::from_string(raw);

        // assert
        assert_eq!(id.as_str(), "user-abc-123");
    }

    #[test]
    fn test_as_str_returns_inner() {
        // arrange
        let id = UserId::from_string("uid".into());

        // act / assert
        assert_eq!(id.as_str(), "uid");
    }

    #[test]
    fn test_display() {
        // arrange
        let id = UserId::from_string("display-uid".into());

        // act / assert
        assert_eq!(id.to_string(), "display-uid");
    }

    #[test]
    fn test_from_str_trait() {
        // arrange / act
        let id: UserId = "from-str".into();

        // assert
        assert_eq!(id.as_str(), "from-str");
    }

    #[test]
    fn test_into_string() {
        // arrange
        let id = UserId::from_string("convert".into());

        // act
        let s: String = id.into();

        // assert
        assert_eq!(s, "convert");
    }

    #[test]
    fn test_deref_to_str() {
        // arrange
        let id = UserId::from_string("deref-uid".into());

        // act - deref coercion to &str
        let contains = id.contains("deref");

        // assert
        assert!(contains);
    }

    #[test]
    fn test_equality() {
        // arrange
        let a = UserId::from_string("same".into());
        let b = UserId::from_string("same".into());

        // assert
        assert_eq!(a, b);
    }

    #[test]
    fn test_empty_string() {
        // arrange / act
        let id = UserId::from_string(String::new());

        // assert
        assert_eq!(id.as_str(), "");
    }
}
