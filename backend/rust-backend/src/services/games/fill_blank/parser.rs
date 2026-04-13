//! Fill Blank parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::FillBlankAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::{OptionId, QuestionId};

use super::domain::{FillBlankOption, FillBlankQuestion};

/// Parse AI-generated fill-blank response into domain objects
pub fn parse_fill_blank_response(
    response: &str,
) -> Result<Vec<FillBlankQuestion>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: FillBlankAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as fill blank questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let questions: Vec<FillBlankQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| FillBlankQuestion {
            id: QuestionId::new(),
            phrase: q.phrase,
            options: q
                .options
                .into_iter()
                .map(|o| FillBlankOption {
                    id: OptionId::new(),
                    text: o.text,
                    is_correct: o.is_correct,
                })
                .collect(),
            explanation: q.explanation,
        })
        .collect();

    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fill_blank_valid_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "phrase": "Rust is a ___ language",
                "options": [
                    {"text": "systems", "is_correct": true},
                    {"text": "scripting", "is_correct": false}
                ],
                "explanation": "Rust is systems-level"
            }]
        })
        .to_string();

        // act
        let questions =
            parse_fill_blank_response(&json).unwrap();

        // assert
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions[0].phrase,
            "Rust is a ___ language"
        );
    }

    #[test]
    fn test_parse_fill_blank_invalid_json_returns_error() {
        // arrange / act
        let result = parse_fill_blank_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fill_blank_empty_returns_empty() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result =
            parse_fill_blank_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_fill_blank_assigns_option_ids() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "phrase": "Test ___",
                "options": [
                    {"text": "A", "is_correct": true},
                    {"text": "B", "is_correct": false}
                ],
                "explanation": "Because A"
            }]
        })
        .to_string();

        // act
        let questions = parse_fill_blank_response(&json).unwrap();
        let opts = &questions[0].options;

        // assert
        assert_ne!(opts[0].id, opts[1].id);
    }

    #[test]
    fn test_parse_fill_blank_missing_explanation_returns_error() {
        // arrange
        let json = r#"{"questions": [{
            "phrase": "___",
            "options": [
                {"text": "A", "is_correct": true}
            ]
        }]}"#;

        // act
        let result = parse_fill_blank_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fill_blank_extra_text_before_json_returns_ok() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "phrase": "___",
                "options": [
                    {"text": "A", "is_correct": true}
                ],
                "explanation": "Because A"
            }]
        })
        .to_string();
        let input = format!("Result:\n{}", json);

        // act / assert
        parse_fill_blank_response(&input).unwrap();
    }

    #[test]
    fn test_parse_fill_blank_multiple_questions_returns_all() {
        // arrange
        let json = serde_json::json!({
            "questions": [
                {"phrase": "A ___", "options": [],
                    "explanation": "E1"},
                {"phrase": "B ___", "options": [],
                    "explanation": "E2"},
                {"phrase": "C ___", "options": [],
                    "explanation": "E3"}
            ]
        })
        .to_string();

        // act
        let qs = parse_fill_blank_response(&json).unwrap();

        // assert
        assert_eq!(qs.len(), 3);
    }
}
