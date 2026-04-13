use crate::services::course::parser::{
    section_parser::parse_generated_section,
};

#[test]
fn test_parse_generated_section_with_valid_json_title() {
    // Arrange
    let json = r#"{
        "order": 1,
        "title": "Introduction",
        "content_blocks": [
            {"type": "text", "content": "Some content"}
        ],
        "qcm_set": {
            "name": "Quiz",
            "description": "Test",
            "level": "intermediate",
            "subjects": ["topic"],
            "questions": [{
                "question": "What is X?",
                "right_answer": "Correct",
                "wrong_answers": ["W1", "W2", "W3"],
                "explanation": "Because this is the correct answer."
            }]
        }
    }"#;

    // Act
    let section =
        parse_generated_section(json).unwrap();

    // Assert
    assert_eq!(section.title, "Introduction");
}

#[test]
fn test_parse_generated_section_with_valid_json_has_qcm() {
    // Arrange
    let json = r#"{
        "order": 1,
        "title": "Introduction",
        "content_blocks": [
            {"type": "text", "content": "Some content"}
        ],
        "qcm_set": {
            "name": "Quiz",
            "description": "Test",
            "level": "intermediate",
            "subjects": ["topic"],
            "questions": [{
                "question": "What is X?",
                "right_answer": "Correct",
                "wrong_answers": ["W1", "W2", "W3"],
                "explanation": "Because this is the correct answer."
            }]
        }
    }"#;

    // Act
    let section =
        parse_generated_section(json).unwrap();

    // Assert
    assert!(!section.qcm_set.questions.is_empty());
}

#[test]
fn test_parse_generated_section_with_empty_blocks() {
    // Arrange
    let json = r#"{
        "order": 1,
        "title": "Section",
        "content_blocks": [],
        "qcm_set": null
    }"#;

    // Act
    let result = parse_generated_section(json);

    // Assert
    assert!(result.is_err());
}
