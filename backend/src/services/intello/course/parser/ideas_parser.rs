use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::ExtractedIdeas;
use crate::services::intello::error_domain::IntelloError;

// ** parse_extracted_ideas **
// ==> Parses AI response JSON into ExtractedIdeas struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : ExtractedIdeas if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or missing fields
pub fn parse_extracted_ideas(
    ai_response: &str,
) -> Result<ExtractedIdeas, IntelloError> {
    // Step 1: Extract JSON from AI response (remove markdown fences, extra text)
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let ideas: ExtractedIdeas =
        serde_json::from_str(&sanitized).map_err(|err| {
            IntelloError::validation(
                "ideas_json",
                format!("Failed to parse ideas JSON: {}", err),
            )
        })?;

    // Step 2: Validate required fields are not empty
    if ideas.core_intent.is_empty() {
        return Err(IntelloError::validation(
            "core_intent",
            "core_intent cannot be empty",
        ));
    }

    if ideas.target_level.is_empty() {
        return Err(IntelloError::validation(
            "target_level",
            "target_level cannot be empty",
        ));
    }

    // Step 3: Return parsed ideas
    Ok(ideas)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build valid ideas JSON
    fn valid_ideas_json() -> String {
        serde_json::json!({
            "core_intent": "Learn Rust basics",
            "target_level": "beginner",
            "mandatory_topics": ["ownership", "borrowing"],
            "suggested_topics": ["lifetimes"],
            "constraints": []
        })
        .to_string()
    }

    #[test]
    fn test_parse_ideas_valid_returns_ok() {
        // arrange
        let json = valid_ideas_json();

        // act
        let result = parse_extracted_ideas(&json);

        // assert
        assert!(result.is_ok());
        let ideas = result.unwrap();
        assert_eq!(ideas.core_intent, "Learn Rust basics");
    }

    #[test]
    fn test_parse_ideas_invalid_json_returns_error() {
        // arrange / act
        let result = parse_extracted_ideas("not json");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_ideas_empty_core_intent_returns_error() {
        // arrange
        let json = serde_json::json!({
            "core_intent": "",
            "target_level": "beginner",
            "mandatory_topics": [],
            "suggested_topics": [],
            "constraints": []
        })
        .to_string();

        // act
        let result = parse_extracted_ideas(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_ideas_empty_target_level_returns_error() {
        // arrange
        let json = serde_json::json!({
            "core_intent": "Learn Rust",
            "target_level": "",
            "mandatory_topics": [],
            "suggested_topics": [],
            "constraints": []
        })
        .to_string();

        // act
        let result = parse_extracted_ideas(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_ideas_markdown_wrapped_returns_ok() {
        // arrange
        let json = format!(
            "```json\n{}\n```",
            valid_ideas_json()
        );

        // act
        let result = parse_extracted_ideas(&json);

        // assert
        assert!(result.is_ok());
    }
}
