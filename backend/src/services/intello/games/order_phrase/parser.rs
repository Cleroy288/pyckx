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
    let ai_response: OrderPhraseAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as order phrase questions: {}. Response was: {}",
                e, json_str
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
