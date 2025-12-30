//! QCM Set domain entity
//!
//! Pure data structure representing a collection of QCM questions.

use serde::{Deserialize, Serialize};

use super::enums_domain::Level;
use super::qcm_question_domain::QcmQuestion;
use crate::services::intello::SetId;
use crate::infra::user::UserId;

/// A set of QCM questions grouped by topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QcmSet {
    /// Unique identifier for the set
    pub id: SetId,
    /// User ID who owns this set
    pub user_id: UserId,
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
