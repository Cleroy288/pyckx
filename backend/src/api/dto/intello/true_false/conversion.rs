//! True or False DTO conversions

use super::response::TrueOrFalseStatementResponse;
use crate::domain::intello::TrueOrFalseStatement;

impl From<&TrueOrFalseStatement> for TrueOrFalseStatementResponse {
    fn from(statement: &TrueOrFalseStatement) -> Self {
        Self {
            id: statement.id.clone(),
            statement: statement.statement.clone(),
            answer: statement.answer,
            explanation: statement.explanation.clone(),
        }
    }
}
