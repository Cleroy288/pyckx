//! Open Question domain entities
//!
//! Represents a set of open-ended questions where users write their own answers.
//! Unlike QCM (multiple choice), these questions have no predefined answers.

use serde::{Deserialize, Serialize};
use super::enums_domain::Level;
use crate::services::intello::{QuestionId, SetId};
use crate::infra::user::UserId;

/// A single open-ended question with user's written answer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenQuestion {
    /// Unique identifier for the question
    pub id: QuestionId,
    /// The question text
    pub question: String,
    /// User's written answer (empty string if not answered yet)
    pub user_answer: String,
    /// Expected answer for AI-generated sets (for self-grading)
    pub expected_answer: Option<String>,
    /// Optional hint for the user
    pub hint: Option<String>,
}

/// A set of open-ended questions grouped by topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenQuestionSet {
    /// Unique identifier for the set
    pub id: SetId,
    /// User ID who owns this set
    pub user_id: UserId,
    /// Name of the open question set
    pub name: String,
    /// Description of what this set covers
    pub description: String,
    /// Difficulty level
    pub level: Level,
    /// Language of the set (e.g., "en", "fr")
    pub language: String,
    /// List of subjects this set covers
    pub subjects: Vec<String>,
    /// List of open questions in this set
    pub questions: Vec<OpenQuestion>,
}
