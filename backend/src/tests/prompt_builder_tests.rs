use crate::domain::intello::{CustomQuestion, CustomQuestionDocument, DocumentType, Level};
use crate::shared::prompt_builder::*;

#[test]
fn test_build_prompt_basic() {
    let question = CustomQuestion {
        id: "test-id".to_string(),
        user_id: "user-123".to_string(),
        name: "History Quiz".to_string(),
        description: "A quiz about World War II".to_string(),
        instructions: "Focus on European theater".to_string(),
        language: "en".to_string(),
        level: Level::Medium,
        output_game: "qcm".to_string(),
        subjects: vec!["History".to_string(), "WWII".to_string()],
        num_questions: 10,
        documents: vec![CustomQuestionDocument {
            filename: "wwii_notes.txt".to_string(),
            doc_type: DocumentType::Text,
            content: "World War II began in 1939...".to_string(),
            token_count: 100,
        }],
        total_token_count: 100,
    };

    let prompt = build_prompt(&question);

    assert!(prompt.contains("History Quiz"));
    assert!(prompt.contains("A quiz about World War II"));
    assert!(prompt.contains("Focus on European theater"));
    assert!(prompt.contains("English"));
    assert!(prompt.contains("MEDIUM")); // Level is uppercase in prompt
    assert!(prompt.contains("10"));
    assert!(prompt.contains("History, WWII"));
    assert!(prompt.contains("wwii_notes.txt"));
}

#[test]
fn test_get_game_format() {
    let qcm = get_game_format("qcm");
    assert!(qcm.is_some());
    assert_eq!(qcm.unwrap().game_id, "qcm");

    let open_question = get_game_format("open_question");
    assert!(open_question.is_some());
    assert_eq!(open_question.unwrap().game_id, "open_question");

    let unknown = get_game_format("unknown_game");
    assert!(unknown.is_none());
}

#[test]
fn test_level_to_string() {
    assert!(level_to_string(&Level::Easy).contains("EASY"));
    assert!(level_to_string(&Level::Medium).contains("MEDIUM"));
    assert!(level_to_string(&Level::Hard).contains("HARD"));
}

#[test]
fn test_build_open_question_prompt() {
    let input = OpenQuestionPromptInput {
        name: "Science Quiz".to_string(),
        description: "Open questions about biology".to_string(),
        instructions: "Focus on cell structure".to_string(),
        language: "en".to_string(),
        level: Level::Medium,
        subjects: vec!["Biology".to_string(), "Cells".to_string()],
        num_questions: 5,
        documents: vec![(
            "biology_notes.txt".to_string(),
            "Cells are the basic unit of life...".to_string(),
        )],
    };

    let prompt = build_open_question_prompt(&input);

    assert!(prompt.contains("Science Quiz"));
    assert!(prompt.contains("Open questions about biology"));
    assert!(prompt.contains("Focus on cell structure"));
    assert!(prompt.contains("English"));
    assert!(prompt.contains("Medium"));
    assert!(prompt.contains("5"));
    assert!(prompt.contains("Biology, Cells"));
    assert!(prompt.contains("biology_notes.txt"));
    assert!(prompt.contains("Open-Ended Questions"));
    assert!(prompt.contains("expected_answer"));
    assert!(prompt.contains("hint"));
}
