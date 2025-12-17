//! QCM Set domain entity
//!
//! Pure data structure representing a collection of QCM questions.

use serde::{Deserialize, Serialize};

use super::enums::Level;
use super::qcm_question::QcmQuestion;

/// A set of QCM questions grouped by topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QcmSet {
    /// Unique identifier for the set
    pub id: String,
    /// User ID who owns this set
    pub user_id: String,
    /// Name of the QCM set
    pub name: String,
    /// Description of what this set covers
    pub description: String,
    /// Difficulty level
    pub level: Level,
    /// Language of the QCM set (e.g., "en", "fr", "es")
    pub language: String,
    /// List of subjects this set covers (e.g., "rust", "python", "algorithms")
    pub subjects: Vec<String>,
    /// List of questions in this set
    pub questions: Vec<QcmQuestion>,
}
