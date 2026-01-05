use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::ExtractedIdeas;
use crate::services::intello::error_domain::IntelloError;

// ** parse_extracted_ideas **
// ==> Parses AI response JSON into ExtractedIdeas struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : ExtractedIdeas if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or missing fields
pub fn parse_extracted_ideas(ai_response: &str) -> Result<ExtractedIdeas, IntelloError> {
    // Step 1: Extract JSON from AI response (remove markdown fences, extra text)
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let ideas: ExtractedIdeas = serde_json::from_str(&sanitized)
        .map_err(|e| IntelloError::validation("ideas_json", format!("Failed to parse ideas JSON: {}", e)))?;

    // Step 2: Validate required fields are not empty
    if ideas.core_intent.is_empty() {
        return Err(IntelloError::validation("core_intent", "core_intent cannot be empty"));
    }

    if ideas.target_level.is_empty() {
        return Err(IntelloError::validation("target_level", "target_level cannot be empty"));
    }

    // Step 3: Return parsed ideas
    Ok(ideas)
}
