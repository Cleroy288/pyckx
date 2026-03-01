//! Order Phrase parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::OrderPhraseAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{OptionId, QuestionId};

use super::domain::{OrderPhraseQuestion, OrderPhraseWord};

// ** parse_order_phrase_response **
// ==> Parses AI-generated order phrase response into domain question objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed OrderPhraseQuestion objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_order_phrase_response(
    response: &str,
) -> Result<Vec<OrderPhraseQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: OrderPhraseAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as order phrase questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
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

    // Step 4: Return parsed order phrase questions
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
        let result = parse_order_phrase_response(&json);

        // assert
        assert!(result.is_ok());
        let questions = result.unwrap();
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
        let result = parse_order_phrase_response(json);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
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
}
