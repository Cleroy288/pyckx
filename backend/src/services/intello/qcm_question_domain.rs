//! QCM Question domain entity
//!
//! Pure data structure representing a single multiple choice question.

use serde::{Deserialize, Serialize};
use crate::services::intello::QuestionId;

/// A single QCM question with one correct answer and three wrong answers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QcmQuestion {
    /// Unique identifier for the question
    pub id: QuestionId,
    /// The question text
    pub question: String,
    /// List of 3 wrong answers
    pub wrong_answers: Vec<String>,
    /// The correct answer
    pub right_answer: String,
    /// Explanation of why the answer is correct
    pub explanation: String,
}
