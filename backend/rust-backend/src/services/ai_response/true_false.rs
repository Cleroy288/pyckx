//! True/False AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TrueOrFalseAiResponse {
    pub statements: Vec<TrueOrFalseAiStatement>,
}

#[derive(Debug, Deserialize)]
pub struct TrueOrFalseAiStatement {
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
}
