//! Document extractor tests

use crate::services::intello::document_extractor_service::extract_txt;
use crate::services::intello::document_extractor::ExtractedContent;

#[test]
fn test_extract_txt_utf8() {
    let content = "Hello, this is a test document.\nWith multiple lines.";
    let bytes = content.as_bytes();

    let result = extract_txt(bytes, "test.txt").unwrap();
    assert_eq!(result.content, content);
    assert!(result.token_count > 0);
}

#[test]
fn test_extract_txt_empty_fails() {
    let bytes = b"   \n\t  ";
    let result = extract_txt(bytes, "empty.txt");
    assert!(result.is_err());
}

#[test]
fn test_extract_txt_token_estimation() {
    let content = "a".repeat(100);
    let result = extract_txt(content.as_bytes(), "test.txt").unwrap();
    assert_eq!(result.token_count, 25);
}

#[test]
fn test_extracted_content_new() {
    let content = ExtractedContent::new("test content".to_string());
    assert_eq!(content.content, "test content");
    assert_eq!(content.token_count, 3);
}
