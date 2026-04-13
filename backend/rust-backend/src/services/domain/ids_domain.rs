//! Study ID Types - NewType wrappers for game entity IDs
//!
//! Provides compile-time type safety to prevent
//! accidental argument swapping.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use uuid::Uuid;

/// Generate a newtype ID wrapper with all standard
/// trait implementations.
///
/// Produces: `new()`, `from_string()`, `as_str()`,
/// `Default`, `Display`, `From<String>`, `From<&str>`,
/// `AsRef<str>`, `Deref<Target=str>`, `Into<String>`.
macro_rules! newtype_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash,
            Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Create a new random UUID-based ID
            pub fn new() -> Self {
                Self(Uuid::new_v4().to_string())
            }

            /// Create from an owned string
            pub fn from_string(s: String) -> Self {
                Self(s)
            }

            /// Borrow the inner string slice
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> String {
                id.0
            }
        }
    };
}

newtype_id!(SetId, "Identifies game sets (QCM, Flashcard, etc.)");
newtype_id!(QuestionId, "Identifies items within sets");
newtype_id!(OptionId, "Identifies answer options within questions");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_id_new_generates_uuid_string() {
        let id = SetId::new();
        assert_eq!(id.as_str().len(), 36);
    }

    #[test]
    fn test_set_id_from_string_preserves_value() {
        let id = SetId::from_string("my-set-id".into());
        assert_eq!(id.as_str(), "my-set-id");
    }

    #[test]
    fn test_set_id_from_str_trait() {
        let id: SetId = "abc-123".into();
        assert_eq!(id.as_str(), "abc-123");
    }

    #[test]
    fn test_set_id_display() {
        let id = SetId::from_string("display-test".into());
        assert_eq!(id.to_string(), "display-test");
    }

    #[test]
    fn test_set_id_into_string() {
        let id = SetId::from_string("convert-me".into());
        let s: String = id.into();
        assert_eq!(s, "convert-me");
    }

    #[test]
    fn test_set_id_deref_to_str() {
        let id = SetId::from_string("deref-test".into());
        assert!(id.starts_with("deref"));
    }

    #[test]
    fn test_set_id_equality() {
        let a = SetId::from_string("same".into());
        let b = SetId::from_string("same".into());
        assert_eq!(a, b);
    }

    #[test]
    fn test_question_id_new_generates_uuid_string() {
        let id = QuestionId::new();
        assert_eq!(id.as_str().len(), 36);
    }

    #[test]
    fn test_question_id_from_string_preserves_value() {
        let id = QuestionId::from_string("q-42".into());
        assert_eq!(id.as_str(), "q-42");
    }

    #[test]
    fn test_question_id_display() {
        let id = QuestionId::from_string("q-disp".into());
        assert_eq!(id.to_string(), "q-disp");
    }

    #[test]
    fn test_question_id_into_string() {
        let id = QuestionId::from_string("q-conv".into());
        let s: String = id.into();
        assert_eq!(s, "q-conv");
    }

    #[test]
    fn test_option_id_new_generates_uuid_string() {
        let id = OptionId::new();
        assert_eq!(id.as_str().len(), 36);
    }

    #[test]
    fn test_option_id_from_string_preserves_value() {
        let id = OptionId::from_string("opt-99".into());
        assert_eq!(id.as_str(), "opt-99");
    }

    #[test]
    fn test_option_id_display() {
        let id = OptionId::from_string("opt-disp".into());
        assert_eq!(id.to_string(), "opt-disp");
    }

    #[test]
    fn test_option_id_into_string() {
        let id = OptionId::from_string("opt-conv".into());
        let s: String = id.into();
        assert_eq!(s, "opt-conv");
    }

    #[test]
    fn test_set_id_from_empty_string() {
        let id = SetId::from_string(String::new());
        assert_eq!(id.as_str(), "");
    }
}
