use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::ParsedSection;
use crate::services::intello::error_domain::IntelloError;

// ** parse_generated_section **
// ==> Parses AI response JSON into ParsedSection struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : ParsedSection if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or structure is wrong
pub fn parse_generated_section(
    ai_response: &str,
) -> Result<ParsedSection, IntelloError> {
    // Step 1: Extract JSON from AI response
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let section: ParsedSection =
        serde_json::from_str(&sanitized).map_err(|err| {
            IntelloError::validation(
                "section_json",
                format!("Failed to parse section JSON: {}", err),
            )
        })?;

    // Step 3: Validate section structure
    if section.title.is_empty() {
        return Err(IntelloError::validation(
            "title",
            "Section title cannot be empty",
        ));
    }

    if section.content_blocks.is_empty() {
        return Err(IntelloError::validation(
            "content_blocks",
            "Section must have at least one content block",
        ));
    }

    // Step 4: Validate QCM set (now required)
    if section.qcm_set.questions.is_empty() {
        return Err(IntelloError::validation(
            "qcm_questions",
            "QCM set must have at least one question",
        ));
    }

    // Validate each question structure
    for (idx, question) in section.qcm_set.questions.iter().enumerate() {
        if question.question.is_empty() {
            return Err(IntelloError::validation(
                "question_text",
                format!("Question {} text cannot be empty", idx + 1),
            ));
        }

        if question.right_answer.is_empty() {
            return Err(IntelloError::validation(
                "right_answer",
                format!("Question {} right answer cannot be empty", idx + 1),
            ));
        }

        if question.wrong_answers.len() != 3 {
            return Err(IntelloError::validation(
                "wrong_answers",
                format!(
                    "Question {} must have exactly 3 wrong answers",
                    idx + 1
                ),
            ));
        }

        // Validate extensive explanation (at least 50 characters)
        if question.explanation.len() < 30 {
            return Err(IntelloError::validation(
                "explanation",
                format!(
                    "Question {} explanation must be at least 30 characters for good learning",
                    idx + 1
                ),
            ));
        }
    }

    // Step 5: Return validated section
    Ok(section)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a valid section JSON string
    fn valid_section_json() -> String {
        serde_json::json!({
            "order": 1,
            "title": "Introduction",
            "content_blocks": [
                {"type": "text", "content": "Some content"}
            ],
            "qcm_set": {
                "name": "Test QCM",
                "description": "desc",
                "level": "easy",
                "subjects": [],
                "questions": [{
                    "question": "What is Rust?",
                    "right_answer": "A systems language",
                    "wrong_answers": ["A game", "A drink", "A tool"],
                    "explanation": "Rust is a systems programming language focused on safety and performance and concurrency."
                }]
            }
        })
        .to_string()
    }

    #[test]
    fn test_parse_section_valid_json_returns_ok() {
        // arrange
        let json = valid_section_json();

        // act
        let result = parse_generated_section(&json);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Introduction");
    }

    #[test]
    fn test_parse_section_invalid_json_returns_error() {
        // arrange
        let json = "not valid json at all";

        // act
        let result = parse_generated_section(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_section_empty_title_returns_error() {
        // arrange
        let json = serde_json::json!({
            "order": 1,
            "title": "",
            "content_blocks": [{"type": "text", "content": "X"}],
            "qcm_set": {
                "name": "Q", "description": "d",
                "level": "easy", "subjects": [],
                "questions": [{
                    "question": "Q?",
                    "right_answer": "A",
                    "wrong_answers": ["B", "C", "D"],
                    "explanation": "A long explanation with enough characters for validation pass."
                }]
            }
        })
        .to_string();

        // act
        let result = parse_generated_section(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_section_empty_blocks_returns_error() {
        // arrange
        let json = serde_json::json!({
            "order": 1,
            "title": "Valid Title",
            "content_blocks": [],
            "qcm_set": {
                "name": "Q", "description": "d",
                "level": "easy", "subjects": [],
                "questions": [{
                    "question": "Q?",
                    "right_answer": "A",
                    "wrong_answers": ["B", "C", "D"],
                    "explanation": "A long explanation."
                }]
            }
        })
        .to_string();

        // act
        let result = parse_generated_section(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_section_markdown_wrapped_returns_ok() {
        // arrange
        let json = format!("```json\n{}\n```", valid_section_json());

        // act
        let result = parse_generated_section(&json);

        // assert
        assert!(result.is_ok());
    }
}
