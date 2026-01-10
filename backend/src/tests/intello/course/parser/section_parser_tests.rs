use crate::services::intello::course::parser::section_parser::parse_generated_section;

#[test]
fn test_parses_valid_section() {
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
                "wrong_answers": ["Wrong1", "Wrong2", "Wrong3"],
                "explanation": "Because this is the correct answer for the question."
            }]
        }
    }"#;

    let section = parse_generated_section(json).unwrap();
    assert_eq!(section.title, "Introduction");
    // qcm_set is now required (not Option), verify it has questions
    assert!(!section.qcm_set.questions.is_empty());
}

#[test]
fn test_rejects_empty_content_blocks() {
    let json = r#"{
        "order": 1,
        "title": "Section",
        "content_blocks": [],
        "qcm_set": null
    }"#;

    let result = parse_generated_section(json);
    assert!(result.is_err());
}
