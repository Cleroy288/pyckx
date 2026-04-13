//! Order Phrase parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::OrderPhraseAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::{OptionId, QuestionId};

use super::domain::{OrderPhraseQuestion, OrderPhraseWord};

/// Parse AI-generated order phrase response into domain objects
pub fn parse_order_phrase_response(
    response: &str,
) -> Result<Vec<OrderPhraseQuestion>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: OrderPhraseAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as order phrase questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let questions: Vec<OrderPhraseQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| OrderPhraseQuestion {
            id: QuestionId::new(),
            original_phrase: q.original_phrase,
            words: q
                .words
                .into_iter()
                .map(|w| OrderPhraseWord {
                    id: OptionId::new(),
                    word: w.word,
                    position: w.position,
                })
                .collect(),
            hint: q.hint,
        })
        .collect();

    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_order_phrase_valid_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "original_phrase": "The cat sat",
                "words": [
                    {"word": "The", "position": 0},
                    {"word": "cat", "position": 1},
                    {"word": "sat", "position": 2}
                ],
                "hint": "Animal on furniture"
            }]
        })
        .to_string();

        // act
        let questions =
            parse_order_phrase_response(&json).unwrap();

        // assert
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions[0].original_phrase,
            "The cat sat"
        );
    }

    #[test]
    fn test_parse_order_phrase_invalid_json_returns_error() {
        // arrange / act
        let result = parse_order_phrase_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_order_phrase_empty_returns_empty() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result =
            parse_order_phrase_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_order_phrase_preserves_positions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "original_phrase": "Hello world",
                "words": [
                    {"word": "Hello", "position": 0},
                    {"word": "world", "position": 1}
                ],
                "hint": "Greeting"
            }]
        })
        .to_string();

        // act
        let qs = parse_order_phrase_response(&json).unwrap();

        // assert
        assert_eq!(qs[0].words[0].position, 0);
        assert_eq!(qs[0].words[1].position, 1);
    }

    #[test]
    fn test_parse_order_phrase_missing_hint_returns_error() {
        // arrange
        let json = r#"{"questions": [{
            "original_phrase": "Hello",
            "words": [
                {"word": "Hello", "position": 0}
            ]
        }]}"#;

        // act
        let result = parse_order_phrase_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_order_phrase_extra_text_before_json_returns_ok() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "original_phrase": "Hello",
                "words": [
                    {"word": "Hello", "position": 0}
                ],
                "hint": "Greeting"
            }]
        })
        .to_string();
        let input = format!("Answer:\n{}", json);

        // act / assert
        parse_order_phrase_response(&input).unwrap();
    }

    #[test]
    fn test_parse_order_phrase_multiple_questions_returns_all() {
        // arrange
        let json = serde_json::json!({
            "questions": [
                {"original_phrase": "A B",
                    "words": [], "hint": "H1"},
                {"original_phrase": "C D",
                    "words": [], "hint": "H2"},
                {"original_phrase": "E F",
                    "words": [], "hint": "H3"}
            ]
        })
        .to_string();

        // act
        let qs = parse_order_phrase_response(&json).unwrap();

        // assert
        assert_eq!(qs.len(), 3);
    }
}
