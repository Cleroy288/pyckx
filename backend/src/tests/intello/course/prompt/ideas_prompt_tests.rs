use crate::services::intello::course::prompt::ideas_prompt::build_ideas_extraction_prompt;

#[test]
fn test_builds_prompt_without_resources() {
    let prompt =
        build_ideas_extraction_prompt("Learn Rust async programming", None);
    assert!(prompt.contains("Learn Rust async programming"));
    assert!(prompt.contains("OUTPUT ONLY THE JSON"));
}

#[test]
fn test_builds_prompt_with_resources() {
    let prompt = build_ideas_extraction_prompt(
        "Learn databases",
        Some("Chapter 1: SQL Basics"),
    );
    assert!(prompt.contains("Learn databases"));
    assert!(prompt.contains("Source Materials"));
    assert!(prompt.contains("Chapter 1: SQL Basics"));
}
