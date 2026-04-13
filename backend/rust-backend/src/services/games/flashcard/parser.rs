//! Flashcard parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::FlashcardAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::QuestionId;

use super::domain::Flashcard;

/// Parse AI-generated flashcard response into domain objects
pub fn parse_flashcard_response(
    response: &str,
) -> Result<Vec<Flashcard>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: FlashcardAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as flashcards: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let flashcards: Vec<Flashcard> = ai_response
        .cards
        .into_iter()
        .map(|c| Flashcard {
            id: QuestionId::new(),
            front: c.front,
            back: c.back,
        })
        .collect();

    Ok(flashcards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flashcard_valid_returns_cards() {
        // arrange
        let json = serde_json::json!({
            "cards": [{
                "front": "What is Rust?",
                "back": "A systems language"
            }]
        })
        .to_string();

        // act
        let cards =
            parse_flashcard_response(&json).unwrap();

        // assert
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].front, "What is Rust?");
    }

    #[test]
    fn test_parse_flashcard_invalid_json_returns_error() {
        // arrange / act
        let result = parse_flashcard_response("bad json");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_flashcard_empty_cards_returns_empty() {
        // arrange
        let json = r#"{"cards": []}"#;

        // act
        let result =
            parse_flashcard_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_flashcard_markdown_wrapped_returns_ok() {
        // arrange
        let json = format!(
            "```json\n{}\n```",
            serde_json::json!({
                "cards": [{
                    "front": "F",
                    "back": "B"
                }]
            })
        );

        // act / assert
        parse_flashcard_response(&json).unwrap();
    }

    #[test]
    fn test_parse_flashcard_missing_back_returns_error() {
        // arrange
        let json = r#"{"cards": [{"front": "Q?"}]}"#;

        // act
        let result = parse_flashcard_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_flashcard_extra_text_before_json_returns_ok() {
        // arrange
        let json = format!(
            "Sure!\n{}",
            serde_json::json!({
                "cards": [{
                    "front": "F",
                    "back": "B"
                }]
            })
        );

        // act / assert
        parse_flashcard_response(&json).unwrap();
    }

    #[test]
    fn test_parse_flashcard_multiple_cards_returns_all() {
        // arrange
        let json = serde_json::json!({
            "cards": [
                {"front": "F1", "back": "B1"},
                {"front": "F2", "back": "B2"},
                {"front": "F3", "back": "B3"}
            ]
        })
        .to_string();

        // act
        let result =
            parse_flashcard_response(&json).unwrap();

        // assert
        assert_eq!(result.len(), 3);
    }
}
