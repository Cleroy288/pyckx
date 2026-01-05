//! True or False parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::TrueOrFalseAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::QuestionId;

use super::domain::TrueOrFalseStatement;

// ** parse_true_false_response **
// ==> Parses AI-generated true/false response into domain statement objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed TrueOrFalseStatement objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_true_false_response(
    response: &str,
) -> Result<Vec<TrueOrFalseStatement>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: TrueOrFalseAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as true/false statements: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
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

    // Step 4: Return parsed true/false statements
    Ok(statements)
}
