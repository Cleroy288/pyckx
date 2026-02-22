//! Document Extractor Domain
//!
//! Contains domain structures for document extraction functionality
// ==> This file defines the core data structures used in document extraction

use crate::services::intello::error_domain::IntelloError;
use serde::{Deserialize, Serialize};

// ** ExtractedContent **
// Represents extracted content from a document with metadata
// ==> Structure holding extracted text and its token count
// @ content : The extracted text content from the document
// @ token_count : Estimated token count for AI processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// The extracted text content
    pub content: String,
    /// Estimated token count (rough: ~4 chars per token)
    pub token_count: u32,
}

impl ExtractedContent {
    /// Create new extracted content and calculate token count
    /// ==> Creates a new ExtractedContent instance with automatic token estimation
    /// @ content : The text content to store
    /// @ returns : ExtractedContent with content and calculated token count
    /// @ edge cases : Empty content will result in 0 token count
    /// @ error conditions : None (always succeeds)
    pub fn new(content: String) -> Self {
        // Rough token estimation: ~4 characters per token (common for English)
        let token_count = (content.len() as f64 / 4.0).ceil() as u32;
        Self {
            content,
            token_count,
        }
    }
}

// ** ExtractResult **
// Type alias for document extraction results
// ==> Standardized result type for all extraction operations
// @ T : The successful extraction result type
// @ IntelloError : Error type for extraction failures
pub type ExtractResult<T> = Result<T, IntelloError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_calculates_token_count_for_short_text() {
        // arrange
        let text = "Hello world"; // 11 chars

        // act
        let content = ExtractedContent::new(text.to_string());

        // assert - ceil(11 / 4.0) = 3
        assert_eq!(content.token_count, 3);
        assert_eq!(content.content, "Hello world");
    }

    #[test]
    fn test_new_with_empty_string_returns_zero_tokens() {
        // arrange / act
        let content = ExtractedContent::new(String::new());

        // assert
        assert_eq!(content.token_count, 0);
        assert_eq!(content.content, "");
    }

    #[test]
    fn test_new_token_count_rounds_up() {
        // arrange - 5 chars => ceil(5/4) = 2
        let text = "abcde";

        // act
        let content = ExtractedContent::new(text.to_string());

        // assert
        assert_eq!(content.token_count, 2);
    }

    #[test]
    fn test_new_exact_multiple_of_four() {
        // arrange - 8 chars => ceil(8/4) = 2
        let text = "abcdefgh";

        // act
        let content = ExtractedContent::new(text.to_string());

        // assert
        assert_eq!(content.token_count, 2);
    }

    #[test]
    fn test_new_preserves_content() {
        // arrange
        let text = "Paragraph with\nnewlines and spaces.";

        // act
        let content = ExtractedContent::new(text.to_string());

        // assert
        assert_eq!(content.content, text);
    }
}
