//! True or False DTO conversions

use super::response::TrueOrFalseStatementResponse;
use crate::services::TrueOrFalseStatement;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::QuestionId;

    #[test]
    fn test_true_false_response_from_true_statement() {
        // arrange
        let stmt = TrueOrFalseStatement {
            id: QuestionId::from_string("tf-1".into()),
            statement: "The sky is blue".into(),
            answer: true,
            explanation: "Because of light".into(),
        };

        // act
        let resp = TrueOrFalseStatementResponse::from(&stmt);

        // assert
        assert_eq!(resp.id, "tf-1");
        assert_eq!(resp.statement, "The sky is blue");
        assert!(resp.answer);
    }

    #[test]
    fn test_true_false_response_from_false_statement() {
        // arrange
        let stmt = TrueOrFalseStatement {
            id: QuestionId::from_string("tf-2".into()),
            statement: "Water is dry".into(),
            answer: false,
            explanation: "Water is wet".into(),
        };

        // act
        let resp = TrueOrFalseStatementResponse::from(&stmt);

        // assert
        assert!(!resp.answer);
        assert_eq!(resp.explanation, "Water is wet");
    }
}
