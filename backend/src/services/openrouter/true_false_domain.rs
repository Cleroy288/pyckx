//! True/False AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct TrueOrFalseAiResponse {
    pub statements: Vec<TrueOrFalseAiStatement>,
}

#[derive(Debug, Deserialize)]
pub(super) struct TrueOrFalseAiStatement {
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
}
