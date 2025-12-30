//! True or False Response DTOs

use serde::Serialize;

/// Response for a single true/false statement
#[derive(Debug, Serialize)]
pub struct TrueOrFalseStatementResponse {
    pub id: String,
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
}

/// Response for creating true/false statements (AI generation)
#[derive(Debug, Serialize)]
pub struct CreateTrueOrFalseResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub statements: Vec<TrueOrFalseStatementResponse>,
}

/// Response for listing true/false sets (with full statements for playing)
#[derive(Debug, Serialize)]
pub struct TrueOrFalseSetListResponse {
    pub sets: Vec<TrueOrFalseSetWithStatementsResponse>,
    pub count: usize,
}

/// Response for a true/false set with full statements (for playing)
#[derive(Debug, Serialize)]
pub struct TrueOrFalseSetWithStatementsResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub statements: Vec<TrueOrFalseStatementResponse>,
}

