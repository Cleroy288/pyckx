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
