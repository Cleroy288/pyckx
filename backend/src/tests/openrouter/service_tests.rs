//! OpenRouter service tests
//!
//! Tests for JSON extraction, sanitization, and course parsing

use crate::infra::openrouter::{
    extract_json_from_response, 
    sanitize_json_duplicates,
};
use crate::http_api::data_transfer_object::intello::course::GeneratedCourse;

// =============================================================================
// JSON EXTRACTION TESTS
// =============================================================================

#[test]
fn test_extract_json_from_markdown() {
    let response = "```json\n{\"test\": 1}\n```";
    assert_eq!(extract_json_from_response(response), "{\"test\": 1}");
}

#[test]
fn test_extract_json_from_generic_code_block() {
    let response = "```\n{\"test\": 2}\n```";
    assert_eq!(extract_json_from_response(response), "{\"test\": 2}");
}

#[test]
fn test_extract_json_from_raw() {
    let response = "{\"test\": 3}";
    assert_eq!(extract_json_from_response(response), "{\"test\": 3}");
}

#[test]
fn test_extract_json_with_surrounding_text() {
    let response = "Here is the result: {\"test\": 4} Thanks!";
    assert_eq!(extract_json_from_response(response), "{\"test\": 4}");
}

// =============================================================================
// JSON SANITIZATION TESTS
// =============================================================================

#[test]
fn test_sanitize_json_no_changes_needed() {
    let json = r#"{"question": "test", "answer": "value"}"#;
    assert_eq!(sanitize_json_duplicates(json), json);
}

#[test]
fn test_sanitize_json_trailing_comma_object() {
    let json = r#"{"question": "test", "answer": "value",}"#;
    let result = sanitize_json_duplicates(json);
    assert_eq!(result, r#"{"question": "test", "answer": "value"}"#);
}

#[test]
fn test_sanitize_json_trailing_comma_array() {
    let json = r#"{"items": ["a", "b", "c",]}"#;
    let result = sanitize_json_duplicates(json);
    assert_eq!(result, r#"{"items": ["a", "b", "c"]}"#);
}

#[test]
fn test_sanitize_json_with_emojis() {
    // This should NOT panic - UTF-8 safety test
    let json = r#"{"title": "📚 Course", "emoji": "🔧", "value": "test–dash"}"#;
    let result = sanitize_json_duplicates(json);
    assert!(result.contains("📚"));
    assert!(result.contains("🔧"));
}

// =============================================================================
// COURSE PARSING TESTS
// =============================================================================

/// Mock JSON response that simulates AI output for a GeneratedCourse
const MOCK_COURSE_JSON: &str = r#"{
    "course_metadata": {
        "title": "Introduction to Rust",
        "description": "Learn Rust basics",
        "level": "beginner"
    },
    "modules": [
        {
            "title": "Getting Started",
            "blocks": [
                { "type": "title", "content": "What is Rust?" },
                { "type": "text", "content": "Rust is a systems language..." },
                { "type": "schema", "language": "mermaid", "content": "graph TD\n A-->B" },
                { "type": "qcm_set", "data": {
                    "name": "Quiz 1",
                    "description": "Test basics",
                    "level": "beginner",
                    "subjects": ["rust"],
                    "questions": [
                        {
                            "question": "Safe?",
                            "right_answer": "Yes",
                            "wrong_answers": ["No"],
                            "explanation": "Memory safe"
                        }
                    ]
                }}
            ]
        }
    ],
    "synthesis": {
        "title": "Course Summary",
        "blocks": [
            { "type": "text", "content": "In this course, we learned about Rust basics." }
        ]
    }
}"#;

#[test]
fn test_parse_generated_course_from_mock_json() {
    let course: GeneratedCourse = serde_json::from_str(MOCK_COURSE_JSON)
        .expect("Should parse mock course JSON");

    assert_eq!(course.course_metadata.title, "Introduction to Rust");
    assert_eq!(course.course_metadata.level, "beginner");
    assert_eq!(course.modules.len(), 1);
    assert_eq!(course.modules[0].title, "Getting Started");
}

#[test]
fn test_course_module_has_blocks() {
    let course: GeneratedCourse = serde_json::from_str(MOCK_COURSE_JSON)
        .expect("Should parse mock course JSON");

    let blocks = &course.modules[0].blocks;
    assert_eq!(blocks.len(), 4, "Should have 4 content blocks");
}

#[test]
fn test_minimal_course_json() {
    let minimal_json = r#"{
        "course_metadata": {
            "title": "Minimal",
            "description": "Desc",
            "level": "easy"
        },
        "modules": [],
        "synthesis": {
            "title": "Summary",
            "blocks": []
        }
    }"#;

    let course: GeneratedCourse = serde_json::from_str(minimal_json)
        .expect("Should parse minimal course JSON");

    assert_eq!(course.course_metadata.title, "Minimal");
    assert!(course.modules.is_empty());
}

#[test]
fn test_invalid_json_fails_gracefully() {
    let invalid_json = r#"{"title": "No other fields"}"#;
    let result: Result<GeneratedCourse, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err(), "Invalid JSON should fail to parse");
}
