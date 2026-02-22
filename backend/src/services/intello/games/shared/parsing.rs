#![allow(dead_code)]
//! Common parsing utilities for game responses

use crate::infra::openrouter::extract_json_from_response as extract_json;

/// Extract and prepare JSON from AI response
///
/// This is a thin wrapper around the infrastructure function
pub fn extract_and_prepare_json(response: &str) -> String {
    extract_json(response)
}

/// Convert AnswerGrade from parsing to domain types
///
/// Helper for converting between our internal AnswerGrade and string representations
pub fn parse_answer_grade(grade_str: &str) -> super::types::AnswerGrade {
    match grade_str.to_lowercase().as_str() {
        "right" => super::types::AnswerGrade::Right,
        "medium" => super::types::AnswerGrade::Medium,
        _ => super::types::AnswerGrade::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::AnswerGrade;

    // -- extract_and_prepare_json tests --

    #[test]
    fn test_extract_json_raw_object_returns_as_is() {
        // arrange
        let input = r#"{"key": "val"}"#;

        // act
        let result = extract_and_prepare_json(input);

        // assert
        assert_eq!(result, r#"{"key": "val"}"#);
    }

    #[test]
    fn test_extract_json_markdown_block_extracts() {
        // arrange
        let input = "```json\n{\"a\": 1}\n```";

        // act
        let result = extract_and_prepare_json(input);

        // assert
        assert_eq!(result, r#"{"a": 1}"#);
    }

    // -- parse_answer_grade tests --

    #[test]
    fn test_parse_grade_scenarios() {
        // arrange
        let cases = vec![
            ("right", AnswerGrade::Right),
            ("Right", AnswerGrade::Right),
            ("RIGHT", AnswerGrade::Right),
            ("medium", AnswerGrade::Medium),
            ("Medium", AnswerGrade::Medium),
            ("error", AnswerGrade::Error),
            ("wrong", AnswerGrade::Error),
            ("unknown", AnswerGrade::Error),
            ("", AnswerGrade::Error),
        ];

        for (input, expected) in cases {
            // act
            let result = parse_answer_grade(input);

            // assert
            assert_eq!(
                result, expected,
                "parse_answer_grade('{}') failed",
                input
            );
        }
    }
}
