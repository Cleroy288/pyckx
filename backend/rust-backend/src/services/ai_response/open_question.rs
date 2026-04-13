//! Open Question AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenQuestionAiResponse {
    pub questions: Vec<OpenQuestionAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub struct OpenQuestionAiQuestion {
    pub question: String,
    pub expected_answer: String,
    pub hint: String,
}
