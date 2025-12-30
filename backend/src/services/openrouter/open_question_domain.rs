//! Open Question AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct OpenQuestionAiResponse {
    pub questions: Vec<OpenQuestionAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub(super) struct OpenQuestionAiQuestion {
    pub question: String,
    pub expected_answer: String,
    pub hint: String,
}
