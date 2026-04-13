use crate::services::course::parser::{
    ideas_parser::parse_extracted_ideas,
};

#[test]
fn test_parse_extracted_ideas_with_valid_json() {
    // Arrange
    let json = r#"{
        "core_intent": "Learn async Rust",
        "target_level": "intermediate",
        "mandatory_topics": ["tokio", "futures"],
        "suggested_topics": ["async-std"],
        "constraints": ["2 weeks"]
    }"#;

    // Act
    let ideas = parse_extracted_ideas(json).unwrap();

    // Assert
    assert_eq!(ideas.core_intent, "Learn async Rust");
}

#[test]
fn test_parse_extracted_ideas_with_valid_json_topics() {
    // Arrange
    let json = r#"{
        "core_intent": "Learn async Rust",
        "target_level": "intermediate",
        "mandatory_topics": ["tokio", "futures"],
        "suggested_topics": ["async-std"],
        "constraints": ["2 weeks"]
    }"#;

    // Act
    let ideas = parse_extracted_ideas(json).unwrap();

    // Assert
    assert_eq!(ideas.mandatory_topics.len(), 2);
}

#[test]
fn test_parse_extracted_ideas_with_empty_core_intent() {
    // Arrange
    let json = r#"{
        "core_intent": "",
        "target_level": "intermediate",
        "mandatory_topics": [],
        "suggested_topics": [],
        "constraints": []
    }"#;

    // Act
    let result = parse_extracted_ideas(json);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_parse_extracted_ideas_with_invalid_json() {
    // Arrange
    let input = "not json";

    // Act
    let result = parse_extracted_ideas(input);

    // Assert
    assert!(result.is_err());
}
