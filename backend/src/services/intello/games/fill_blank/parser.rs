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
pub fn parse_fill_blank_response(
    response: &str,
) -> Result<Vec<FillBlankQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: FillBlankAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as fill blank questions: {}. Response was: {}",
                err, json_str
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fill_blank_valid_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "phrase": "Rust is a ___ language",
                "options": [
                    {"text": "systems", "is_correct": true},
                    {"text": "scripting", "is_correct": false}
                ],
                "explanation": "Rust is systems-level"
            }]
        })
        .to_string();

        // act
        let result = parse_fill_blank_response(&json);

        // assert
        assert!(result.is_ok());
        let questions = result.unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions[0].phrase,
            "Rust is a ___ language"
        );
    }

    #[test]
    fn test_parse_fill_blank_invalid_json_returns_error() {
        // arrange / act
        let result = parse_fill_blank_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fill_blank_empty_returns_empty() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result = parse_fill_blank_response(json);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_parse_fill_blank_assigns_option_ids() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "phrase": "Test ___",
                "options": [
                    {"text": "A", "is_correct": true},
                    {"text": "B", "is_correct": false}
                ],
                "explanation": "Because A"
            }]
        })
        .to_string();

        // act
        let questions = parse_fill_blank_response(&json).unwrap();
        let opts = &questions[0].options;

        // assert
        assert_ne!(opts[0].id, opts[1].id);
    }
}
