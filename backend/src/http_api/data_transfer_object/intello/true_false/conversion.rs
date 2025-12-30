//! True or False DTO conversions

use super::response::TrueOrFalseStatementResponse;
use crate::services::intello::TrueOrFalseStatement;

impl From<&TrueOrFalseStatement> for TrueOrFalseStatementResponse {
    fn from(statement: &TrueOrFalseStatement) -> Self {
        Self {
            id: statement.id.to_string(),
            statement: statement.statement.clone(),
            answer: statement.answer,
            explanation: statement.explanation.clone(),
        }
    }
}
