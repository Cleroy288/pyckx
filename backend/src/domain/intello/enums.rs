//! Intello domain enums
//!
//! Contains enumeration types for the Intello quiz system.

use serde::{Deserialize, Serialize};

/// Difficulty level for a QCM set
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Easy,
    Medium,
    Hard,
}
