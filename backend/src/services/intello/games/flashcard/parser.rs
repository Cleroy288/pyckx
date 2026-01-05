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
pub fn parse_flashcard_response(response: &str) -> Result<Vec<Flashcard>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: FlashcardAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as flashcards: {}. Response was: {}",
                e, json_str
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
