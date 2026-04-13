//! Document Extractor Service
//!
//! Extraction functions for plain text, PDF, and Word
//! documents.

use crate::services::document_extractor::{
    document_extractor_domain::{ExtractResult, ExtractedContent},
    document_extractor_utils::collect_docx_text,
};
use crate::services::error_domain::StudyError;

/// Validates that content is non-empty, returning an error
/// with the filename context when it is blank.
pub(crate) fn require_non_empty(
    content: &str,
    filename: &str,
    label: &str,
) -> Result<(), StudyError> {
    if content.trim().is_empty() {
        return Err(StudyError::validation(
            "document",
            format!("{} '{}' is empty or has no text", label, filename),
        ));
    }
    Ok(())
}

/// Extracts text from a `.txt` file (UTF-8 or Latin-1).
pub fn extract_txt(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    let content = match std::str::from_utf8(bytes) {
        Ok(text) => text.to_string(),
        Err(_) => bytes.iter().map(|&b| b as char).collect(),
    };
    require_non_empty(&content, filename, "File")?;
    Ok(ExtractedContent::new(content))
}

/// Extracts text from a PDF file.
pub fn extract_pdf(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    let content = pdf_extract::extract_text_from_mem(bytes)
        .map_err(|err| {
            StudyError::validation(
                "document",
                format!(
                    "Failed to extract PDF '{}': {}",
                    filename, err
                ),
            )
        })?;
    require_non_empty(&content, filename, "PDF")?;
    Ok(ExtractedContent::new(content))
}

/// Extracts text from a Word (.docx) file.
pub fn extract_word(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    let docx = docx_rs::read_docx(bytes).map_err(|err| {
        StudyError::validation(
            "document",
            format!(
                "Failed to read Word '{}': {}",
                filename, err
            ),
        )
    })?;
    let content = collect_docx_text(docx);
    require_non_empty(&content, filename, "Word document")?;
    Ok(ExtractedContent::new(content))
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- extract_txt tests --

    #[test]
    fn test_extract_txt_valid_utf8_returns_content() {
        // arrange
        let bytes = b"Hello, world!";

        // act
        let extracted = extract_txt(bytes, "test.txt").unwrap();

        // assert
        assert_eq!(extracted.content, "Hello, world!");
    }

    #[test]
    fn test_extract_txt_empty_bytes_returns_error() {
        // arrange
        let bytes = b"";

        // act
        let result = extract_txt(bytes, "empty.txt");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_txt_whitespace_only_returns_error() {
        // arrange
        let bytes = b"   \n  \t  ";

        // act
        let result = extract_txt(bytes, "blank.txt");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_txt_latin1_fallback_returns_content() {
        // arrange - bytes 0xE9 is 'e' with accent in Latin-1
        let bytes: &[u8] = &[0xC0, 0xE9, 0x6C, 0x6F];

        // act
        let extracted =
            extract_txt(bytes, "latin.txt").unwrap();

        // assert
        assert!(!extracted.content.is_empty());
    }

    // -- extract_pdf tests (error case only) --

    #[test]
    fn test_extract_pdf_invalid_bytes_returns_error() {
        // arrange
        let bytes = b"not a real pdf";

        // act
        let result = extract_pdf(bytes, "bad.pdf");

        // assert
        assert!(result.is_err());
    }

    // -- extract_word tests (error case only) --

    #[test]
    fn test_extract_word_invalid_bytes_returns_error() {
        // arrange
        let bytes = b"not a real docx";

        // act
        let result = extract_word(bytes, "bad.docx");

        // assert
        assert!(result.is_err());
    }

    // -- extract_txt edge-case tests --

    #[test]
    fn test_extract_txt_unicode_content_returns_ok() {
        // arrange
        let bytes = "Héllo café résumé".as_bytes();

        // act
        let result = extract_txt(bytes, "unicode.txt");

        // assert
        let extracted = result.unwrap();
        assert_eq!(extracted.content, "Héllo café résumé");
    }

    #[test]
    fn test_extract_txt_multiline_content_returns_ok() {
        // arrange
        let bytes = "Line 1\nLine 2\nLine 3".as_bytes();

        // act
        let result = extract_txt(bytes, "multi.txt");

        // assert
        let extracted = result.unwrap();
        assert!(extracted.content.contains("Line 1"));
        assert!(extracted.content.contains("Line 2"));
        assert!(extracted.content.contains("Line 3"));
    }

    #[test]
    fn test_extract_txt_calculates_token_count() {
        // arrange — "Hello world test" = 16 chars
        let bytes = b"Hello world test";

        // act
        let result = extract_txt(bytes, "tokens.txt");

        // assert — ceil(16 / 4) = 4
        let extracted = result.unwrap();
        assert_eq!(extracted.token_count, 4);
    }

    #[test]
    fn test_extract_txt_single_char_returns_ok() {
        // arrange
        let bytes = b"A";

        // act
        let result = extract_txt(bytes, "single.txt");

        // assert
        let extracted = result.unwrap();
        assert_eq!(extracted.content, "A");
    }

    #[test]
    fn test_extract_txt_newlines_only_returns_error() {
        // arrange
        let bytes = b"\n\n\n";

        // act
        let result = extract_txt(bytes, "newlines.txt");

        // assert
        assert!(result.is_err());
    }

    // -- extract_pdf edge-case tests --

    #[test]
    fn test_extract_pdf_empty_bytes_returns_error() {
        // arrange
        let bytes = b"";

        // act
        let result = extract_pdf(bytes, "empty.pdf");

        // assert
        assert!(result.is_err());
    }
}
