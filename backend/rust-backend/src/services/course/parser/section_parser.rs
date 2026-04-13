use crate::infra::openrouter::{
    extract_json_from_response, sanitize_ai_json,
};
use crate::services::course::domain::ParsedSection;
use crate::services::error_domain::StudyError;

/// Minimum explanation length for QCM questions
const MIN_EXPLANATION_LEN: usize = 30;
/// Required number of wrong answers per question
const REQUIRED_WRONG_ANSWERS: usize = 3;

/// Parses AI response JSON into a ParsedSection struct
pub fn parse_generated_section(
    ai_response: &str,
) -> Result<ParsedSection, StudyError> {
    let extracted =
        extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    let section: ParsedSection =
        serde_json::from_str(&sanitized).map_err(|err| {
            StudyError::validation(
                "section_json",
                format!("Failed to parse: {}", err),
            )
        })?;

    validate_section_structure(&section)?;
    validate_section_questions(&section)?;
    Ok(section)
}

/// Validate section title and content blocks
fn validate_section_structure(
    section: &ParsedSection,
) -> Result<(), StudyError> {
    if section.title.is_empty() {
        return Err(StudyError::validation(
            "title",
            "Section title cannot be empty",
        ));
    }
    if section.content_blocks.is_empty() {
        return Err(StudyError::validation(
            "content_blocks",
            "Section must have at least one block",
        ));
    }
    if section.qcm_set.questions.is_empty() {
        return Err(StudyError::validation(
            "qcm_questions",
            "QCM set must have at least one question",
        ));
    }
    Ok(())
}

/// Validate each question in the QCM set
fn validate_section_questions(
    section: &ParsedSection,
) -> Result<(), StudyError> {
    for (idx, q) in
        section.qcm_set.questions.iter().enumerate()
    {
        validate_single_question(q, idx)?;
    }
    Ok(())
}

/// Validate a single QCM question
fn validate_single_question(
    q: &crate::http_api::data_transfer_object::course::QcmQuestionPayload,
    idx: usize,
) -> Result<(), StudyError> {
    if q.question.is_empty() {
        return Err(StudyError::validation(
            "question_text",
            format!("Question {} text empty", idx + 1),
        ));
    }
    if q.right_answer.is_empty() {
        return Err(StudyError::validation(
            "right_answer",
            format!("Question {} answer empty", idx + 1),
        ));
    }
    if q.wrong_answers.len() != REQUIRED_WRONG_ANSWERS {
        return Err(StudyError::validation(
            "wrong_answers",
            format!(
                "Question {} needs {} wrong answers",
                idx + 1,
                REQUIRED_WRONG_ANSWERS
            ),
        ));
    }
    if q.explanation.len() < MIN_EXPLANATION_LEN {
        return Err(StudyError::validation(
            "explanation",
            format!(
                "Question {} explanation too short",
                idx + 1
            ),
        ));
    }
    Ok(())
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
        let json = valid_section_json();
        let result = parse_generated_section(&json);
        assert_eq!(
            result.unwrap().title,
            "Introduction"
        );
    }

    #[test]
    fn test_parse_section_invalid_json_returns_error() {
        let result =
            parse_generated_section("not valid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_section_empty_title_returns_error() {
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
        let result = parse_generated_section(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_section_empty_blocks_returns_error() {
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
        let result = parse_generated_section(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_section_markdown_wrapped_returns_ok() {
        let json = format!(
            "```json\n{}\n```",
            valid_section_json()
        );
        let result = parse_generated_section(&json);
        assert!(result.is_ok());
    }
}
