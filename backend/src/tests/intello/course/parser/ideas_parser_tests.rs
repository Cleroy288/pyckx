use crate::services::intello::course::parser::ideas_parser::parse_extracted_ideas;

#[test]
fn test_parses_valid_ideas() {
    let json = r#"{
        "core_intent": "Learn async Rust",
        "target_level": "intermediate",
        "mandatory_topics": ["tokio", "futures"],
        "suggested_topics": ["async-std"],
        "constraints": ["2 weeks"]
    }"#;

    let ideas = parse_extracted_ideas(json).unwrap();
    assert_eq!(ideas.core_intent, "Learn async Rust");
    assert_eq!(ideas.mandatory_topics.len(), 2);
}

#[test]
fn test_rejects_empty_core_intent() {
    let json = r#"{
        "core_intent": "",
        "target_level": "intermediate",
        "mandatory_topics": [],
        "suggested_topics": [],
        "constraints": []
    }"#;

    let result = parse_extracted_ideas(json);
    assert!(result.is_err());
}

#[test]
fn test_rejects_invalid_json() {
    let result = parse_extracted_ideas("not json");
    assert!(result.is_err());
}
