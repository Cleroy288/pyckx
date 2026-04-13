//! Keywords parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::KeywordsAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::{OptionId, QuestionId};

use super::domain::{Keyword, KeywordQuestion};

/// Parse AI-generated keywords response into domain objects
pub fn parse_keywords_response(
    response: &str,
) -> Result<Vec<KeywordQuestion>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: KeywordsAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as keywords questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let questions: Vec<KeywordQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| KeywordQuestion {
            id: QuestionId::new(),
            statement: q.statement,
            keywords: q
                .keywords
                .into_iter()
                .map(|k| Keyword {
                    id: OptionId::new(),
                    word: k.word,
                    is_correct: k.is_correct,
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
    fn test_parse_keywords_valid_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "statement": "Rust memory management",
                "keywords": [
                    {"word": "ownership", "is_correct": true},
                    {"word": "garbage", "is_correct": false}
                ],
                "explanation": "Rust uses ownership"
            }]
        })
        .to_string();

        // act
        let questions =
            parse_keywords_response(&json).unwrap();

        // assert
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions[0].statement,
            "Rust memory management"
        );
    }

    #[test]
    fn test_parse_keywords_invalid_json_returns_error() {
        // arrange / act
        let result = parse_keywords_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_keywords_empty_returns_empty() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result =
            parse_keywords_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_keywords_assigns_unique_ids() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "statement": "Test",
                "keywords": [
                    {"word": "A", "is_correct": true},
                    {"word": "B", "is_correct": false}
                ],
                "explanation": "E"
            }]
        })
        .to_string();

        // act
        let qs = parse_keywords_response(&json).unwrap();
        let kws = &qs[0].keywords;

        // assert
        assert_ne!(kws[0].id, kws[1].id);
    }

    #[test]
    fn test_parse_keywords_missing_explanation_returns_error() {
        // arrange
        let json = r#"{"questions": [{
            "statement": "S",
            "keywords": [
                {"word": "W", "is_correct": true}
            ]
        }]}"#;

        // act
        let result = parse_keywords_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_keywords_empty_keywords_list_returns_ok() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "statement": "S",
                "keywords": [],
                "explanation": "E"
            }]
        })
        .to_string();

        // act
        let result =
            parse_keywords_response(&json).unwrap();

        // assert
        assert_eq!(result.len(), 1);
        assert!(result[0].keywords.is_empty());
    }

    #[test]
    fn test_parse_keywords_multiple_questions_returns_all() {
        // arrange
        let make_q = |s: &str| serde_json::json!({
            "statement": s,
            "keywords": [
                {"word": "W", "is_correct": true}
            ],
            "explanation": "E"
        });
        let json = serde_json::json!({
            "questions": [
                make_q("S1"),
                make_q("S2"),
                make_q("S3")
            ]
        })
        .to_string();

        // act
        let result =
            parse_keywords_response(&json).unwrap();

        // assert
        assert_eq!(result.len(), 3);
    }
}
