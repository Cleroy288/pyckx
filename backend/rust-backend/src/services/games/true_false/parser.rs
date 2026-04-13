//! True or False parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::TrueOrFalseAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::QuestionId;

use super::domain::TrueOrFalseStatement;

/// Parse AI-generated true/false response into domain objects
pub fn parse_true_false_response(
    response: &str,
) -> Result<Vec<TrueOrFalseStatement>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: TrueOrFalseAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as true/false statements: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let statements: Vec<TrueOrFalseStatement> = ai_response
        .statements
        .into_iter()
        .map(|s| TrueOrFalseStatement {
            id: QuestionId::new(),
            statement: s.statement,
            answer: s.answer,
            explanation: s.explanation,
        })
        .collect();

    Ok(statements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_true_false_valid_returns_statements() {
        // arrange
        let json = serde_json::json!({
            "statements": [{
                "statement": "Rust has a GC",
                "answer": false,
                "explanation": "Rust uses ownership"
            }]
        })
        .to_string();

        // act
        let stmts =
            parse_true_false_response(&json).unwrap();

        // assert
        assert_eq!(stmts.len(), 1);
        assert!(!stmts[0].answer);
    }

    #[test]
    fn test_parse_true_false_invalid_json_returns_error() {
        // arrange / act
        let result = parse_true_false_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_true_false_empty_returns_empty() {
        // arrange
        let json = r#"{"statements": []}"#;

        // act
        let result =
            parse_true_false_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_true_false_true_answer_preserved() {
        // arrange
        let json = serde_json::json!({
            "statements": [{
                "statement": "Rust is compiled",
                "answer": true,
                "explanation": "Compiled via LLVM"
            }]
        })
        .to_string();

        // act
        let stmts = parse_true_false_response(&json).unwrap();

        // assert
        assert!(stmts[0].answer);
    }

    #[test]
    fn test_parse_true_false_missing_explanation_returns_error() {
        // arrange
        let json = r#"{"statements": [{
            "statement": "S",
            "answer": true
        }]}"#;

        // act
        let result = parse_true_false_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_true_false_extra_text_before_json_returns_ok() {
        // arrange
        let json = serde_json::json!({
            "statements": [{
                "statement": "S",
                "answer": true,
                "explanation": "E"
            }]
        })
        .to_string();
        let input = format!("Here:\n{}", json);

        // act / assert
        parse_true_false_response(&input).unwrap();
    }

    #[test]
    fn test_parse_true_false_multiple_statements_returns_all() {
        // arrange
        let json = serde_json::json!({
            "statements": [
                {"statement": "A", "answer": true,
                    "explanation": "E1"},
                {"statement": "B", "answer": false,
                    "explanation": "E2"},
                {"statement": "C", "answer": true,
                    "explanation": "E3"}
            ]
        })
        .to_string();

        // act
        let stmts =
            parse_true_false_response(&json).unwrap();

        // assert
        assert_eq!(stmts.len(), 3);
    }
}
