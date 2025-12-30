//! QCM AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct QcmAiResponse {
    pub questions: Vec<QcmAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub(super) struct QcmAiQuestion {
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}
