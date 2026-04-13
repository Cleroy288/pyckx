//! Document Extractor Domain
//!
//! Core data structures for document extraction.

use crate::services::error_domain::StudyError;
use serde::{Deserialize, Serialize};

/// Average characters per token for English text.
const CHARS_PER_TOKEN: f64 = 4.0;

/// Extracted content from a document with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// The extracted text content
    pub content: String,
    /// Estimated token count for AI processing
    pub token_count: u32,
}

impl ExtractedContent {
    /// Creates extracted content with automatic token
    /// estimation based on character count.
    pub fn new(content: String) -> Self {
        let token_count =
            (content.len() as f64 / CHARS_PER_TOKEN)
                .ceil() as u32;
        Self {
            content,
            token_count,
        }
    }
}

/// Result type alias for extraction operations.
pub type ExtractResult<T> = Result<T, StudyError>;

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

    #[test]
    fn test_new_single_char_returns_one_token() {
        // arrange / act
        let content =
            ExtractedContent::new("a".to_string());

        // assert — ceil(1/4) = 1
        assert_eq!(content.token_count, 1);
    }

    #[test]
    fn test_new_long_text_calculates_correctly() {
        // arrange
        let text = "x".repeat(100);

        // act
        let content = ExtractedContent::new(text);

        // assert — ceil(100/4) = 25
        assert_eq!(content.token_count, 25);
    }

    #[test]
    fn test_new_unicode_counts_bytes_not_chars() {
        // arrange — "e\u{0301}" is 2 bytes per char
        let text = "éé".to_string();
        let byte_len = text.len(); // 4 bytes

        // act
        let content = ExtractedContent::new(text);

        // assert — ceil(4/4) = 1
        assert_eq!(byte_len, 4);
        assert_eq!(content.token_count, 1);
    }

    #[test]
    fn test_new_whitespace_preserves_content() {
        // arrange
        let text = "  spaces  "; // 10 bytes

        // act
        let content =
            ExtractedContent::new(text.to_string());

        // assert — ceil(10/4) = 3
        assert_eq!(content.content, "  spaces  ");
        assert_eq!(content.token_count, 3);
    }
}
