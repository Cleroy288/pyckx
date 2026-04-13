use crate::services::course::parser::synthesis_parser::parse_generated_synthesis;

#[test]
fn test_parses_valid_synthesis() {
    let json = r#"{
        "summary_text": "This course covered async Rust in depth...",
        "key_takeaways": [
            "Tokio is the main async runtime",
            "Futures represent async computations"
        ],
        "final_qcm": {
            "name": "Final Assessment",
            "description": "Test all concepts",
            "level": "intermediate",
            "subjects": ["async", "tokio"],
            "questions": [{
                "question": "What is tokio?",
                "right_answer": "An async runtime",
                "wrong_answers": ["A sync library", "A compiler", "A framework"],
                "explanation": "Tokio is Rust's main async runtime"
            }]
        }
    }"#;

    let synthesis = parse_generated_synthesis(json).unwrap();
    assert!(!synthesis.summary_text.is_empty());
    assert_eq!(synthesis.key_takeaways.len(), 2);
    assert_eq!(synthesis.final_qcm.questions.len(), 1);
}

#[test]
fn test_rejects_empty_summary() {
    let json = r#"{
        "summary_text": "",
        "key_takeaways": ["Something"],
        "final_qcm": {
            "name": "Quiz",
            "description": "Test",
            "level": "easy",
            "subjects": [],
            "questions": []
        }
    }"#;

    let result = parse_generated_synthesis(json);
    assert!(result.is_err());
}

#[test]
fn test_rejects_empty_qcm() {
    let json = r#"{
        "summary_text": "Summary",
        "key_takeaways": ["Takeaway"],
        "final_qcm": {
            "name": "Quiz",
            "description": "Test",
            "level": "easy",
            "subjects": [],
            "questions": []
        }
    }"#;

    let result = parse_generated_synthesis(json);
    assert!(result.is_err());
}
