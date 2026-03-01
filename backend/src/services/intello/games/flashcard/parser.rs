//! Flashcard parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::FlashcardAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::QuestionId;

use super::domain::Flashcard;

// ** parse_flashcard_response **
// ==> Parses AI-generated flashcard response into domain objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed Flashcard objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_flashcard_response(
    response: &str,
) -> Result<Vec<Flashcard>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: FlashcardAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as flashcards: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
    let flashcards: Vec<Flashcard> = ai_response
        .cards
        .into_iter()
        .map(|c| Flashcard {
            id: QuestionId::new(),
            front: c.front,
            back: c.back,
        })
        .collect();

    // Step 4: Return parsed flashcards
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
        let result = parse_flashcard_response(&json);

        // assert
        assert!(result.is_ok());
        let cards = result.unwrap();
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
        let result = parse_flashcard_response(json);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
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

        // act
        let result = parse_flashcard_response(&json);

        // assert
        assert!(result.is_ok());
    }
}
