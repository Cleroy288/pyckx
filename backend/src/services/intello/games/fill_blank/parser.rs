//! Fill Blank parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::FillBlankAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{OptionId, QuestionId};

use super::domain::{FillBlankOption, FillBlankQuestion};

// ** parse_fill_blank_response **
// ==> Parses AI-generated fill-in-the-blank response into domain question objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed FillBlankQuestion objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_fill_blank_response(response: &str) -> Result<Vec<FillBlankQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: FillBlankAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as fill blank questions: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
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

    // Step 4: Return parsed fill-in-the-blank questions
    Ok(questions)
}
