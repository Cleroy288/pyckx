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
