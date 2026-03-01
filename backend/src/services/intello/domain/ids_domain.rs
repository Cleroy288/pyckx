//! Intello ID Types - NewType wrappers for game entity IDs
//!
//! Provides compile-time type safety to prevent accidental argument swapping.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use uuid::Uuid;

/* ============================================================================
 * SET ID - Identifies game sets (QCM, Flashcard, etc.)
 * ============================================================================ */

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SetId(String);

impl SetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SetId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SetId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for SetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for SetId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SetId> for String {
    fn from(id: SetId) -> String {
        id.0
    }
}

/* ============================================================================
 * QUESTION ID - Identifies items within sets
 * ============================================================================ */

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuestionId(String);

impl QuestionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for QuestionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QuestionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for QuestionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for QuestionId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for QuestionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for QuestionId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<QuestionId> for String {
    fn from(id: QuestionId) -> String {
        id.0
    }
}

/* ============================================================================
 * OPTION ID - Identifies answer options within questions
 * ============================================================================ */

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OptionId(String);

impl OptionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for OptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OptionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for OptionId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for OptionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for OptionId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OptionId> for String {
    fn from(id: OptionId) -> String {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- SetId tests --

    #[test]
    fn test_set_id_new_generates_uuid_string() {
        // arrange / act
        let id = SetId::new();

        // assert - UUID v4 format: 8-4-4-4-12 hex chars
        assert_eq!(id.as_str().len(), 36);
    }

    #[test]
    fn test_set_id_from_string_preserves_value() {
        // arrange
        let raw = "my-set-id".to_string();

        // act
        let id = SetId::from_string(raw.clone());

        // assert
        assert_eq!(id.as_str(), "my-set-id");
    }

    #[test]
    fn test_set_id_from_str_trait() {
        // arrange / act
        let id: SetId = "abc-123".into();

        // assert
        assert_eq!(id.as_str(), "abc-123");
    }

    #[test]
    fn test_set_id_display() {
        // arrange
        let id = SetId::from_string("display-test".into());

        // act / assert
        assert_eq!(id.to_string(), "display-test");
    }

    #[test]
    fn test_set_id_into_string() {
        // arrange
        let id = SetId::from_string("convert-me".into());

        // act
        let s: String = id.into();

        // assert
        assert_eq!(s, "convert-me");
    }

    #[test]
    fn test_set_id_deref_to_str() {
        // arrange
        let id = SetId::from_string("deref-test".into());

        // act - deref allows &str methods
        let starts = id.starts_with("deref");

        // assert
        assert!(starts);
    }

    #[test]
    fn test_set_id_equality() {
        // arrange
        let a = SetId::from_string("same".into());
        let b = SetId::from_string("same".into());

        // assert
        assert_eq!(a, b);
    }

    // -- QuestionId tests --

    #[test]
    fn test_question_id_new_generates_uuid_string() {
        // arrange / act
        let id = QuestionId::new();

        // assert
        assert_eq!(id.as_str().len(), 36);
    }

    #[test]
    fn test_question_id_from_string_preserves_value() {
        // arrange / act
        let id = QuestionId::from_string("q-42".into());

        // assert
        assert_eq!(id.as_str(), "q-42");
    }

    #[test]
    fn test_question_id_display() {
        // arrange
        let id = QuestionId::from_string("q-disp".into());

        // act / assert
        assert_eq!(id.to_string(), "q-disp");
    }

    #[test]
    fn test_question_id_into_string() {
        // arrange
        let id = QuestionId::from_string("q-conv".into());

        // act
        let s: String = id.into();

        // assert
        assert_eq!(s, "q-conv");
    }

    // -- OptionId tests --

    #[test]
    fn test_option_id_new_generates_uuid_string() {
        // arrange / act
        let id = OptionId::new();

        // assert
        assert_eq!(id.as_str().len(), 36);
    }

    #[test]
    fn test_option_id_from_string_preserves_value() {
        // arrange / act
        let id = OptionId::from_string("opt-99".into());

        // assert
        assert_eq!(id.as_str(), "opt-99");
    }

    #[test]
    fn test_option_id_display() {
        // arrange
        let id = OptionId::from_string("opt-disp".into());

        // act / assert
        assert_eq!(id.to_string(), "opt-disp");
    }

    #[test]
    fn test_option_id_into_string() {
        // arrange
        let id = OptionId::from_string("opt-conv".into());

        // act
        let s: String = id.into();

        // assert
        assert_eq!(s, "opt-conv");
    }

    // -- Cross-type inequality test --

    #[test]
    fn test_set_id_from_empty_string() {
        // arrange / act
        let id = SetId::from_string(String::new());

        // assert
        assert_eq!(id.as_str(), "");
    }
}
