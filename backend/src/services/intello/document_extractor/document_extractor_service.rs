//! Document Extractor Service
//!
//! Contains document extraction functions for various file formats
// ==> This file provides the main service functions for extracting text from documents

use crate::services::intello::document_extractor::document_extractor_domain::{
    ExtractResult, ExtractedContent,
};
use crate::services::intello::document_extractor::document_extractor_utils::extract_text_from_xml;
use crate::services::intello::error_domain::IntelloError;

// ** extract_txt **
// Extracts text content from a .txt file
// ==> Handles plain text files with UTF-8 and Latin-1 encoding
// @ bytes : Raw file bytes received from HTTP request
// @ filename : Original filename (for error messages)
// @ returns : ExtractedContent with the text and estimated token count
// @ edge cases : Empty files, different text encodings
// @ error conditions : Empty or whitespace-only files
pub fn extract_txt(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    // Step 1: Try UTF-8 decoding first (most common)
    match std::str::from_utf8(bytes) {
        Ok(text) => {
            // Step 2: Validate content is not empty
            let content = text.to_string();
            if content.trim().is_empty() {
                return Err(IntelloError::validation(
                    "document",
                    format!(
                        "File '{}' is empty or contains only whitespace",
                        filename
                    ),
                ));
            }
            // Step 3: Return extracted content
            Ok(ExtractedContent::new(content))
        }
        Err(_) => {
            // Step 4: Fallback to Latin-1 (ISO-8859-1) for older documents
            let content: String = bytes.iter().map(|&b| b as char).collect();
            if content.trim().is_empty() {
                return Err(IntelloError::validation(
                    "document",
                    format!(
                        "File '{}' is empty or contains only whitespace",
                        filename
                    ),
                ));
            }
            Ok(ExtractedContent::new(content))
        }
    }
}

// ** extract_pdf **
// Extracts text content from a PDF file
// ==> Uses pdf_extract crate to extract text from PDF documents
// @ bytes : Raw PDF file bytes received from HTTP request
// @ filename : Original filename (for error messages)
// @ returns : ExtractedContent with the extracted text and estimated token count
// @ edge cases : PDFs with no text content, encrypted PDFs
// @ error conditions : Invalid PDF format, extraction failures
pub fn extract_pdf(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    // Step 1: Extract text from PDF using pdf_extract crate
    let content = pdf_extract::extract_text_from_mem(bytes).map_err(|err| {
        IntelloError::validation(
            "document",
            format!("Failed to extract text from PDF '{}': {}", filename, err),
        )
    })?;

    // Step 2: Validate extracted content is not empty
    if content.trim().is_empty() {
        return Err(IntelloError::validation(
            "document",
            format!("PDF '{}' contains no extractable text", filename),
        ));
    }

    // Step 3: Return extracted content with token estimate
    Ok(ExtractedContent::new(content))
}

// ** extract_word **
// Extracts text content from a Word (.docx) file
// ==> Uses docx_rs crate to parse DOCX documents
// @ bytes : Raw DOCX file bytes received from HTTP request
// @ filename : Original filename (for error messages)
// @ returns : ExtractedContent with the extracted text and estimated token count
// @ edge cases : Documents with no text content, complex formatting
// @ error conditions : Invalid DOCX format, parsing failures
#[allow(clippy::excessive_nesting)]
pub fn extract_word(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    // Step 1: Parse DOCX file using docx_rs
    let docx = docx_rs::read_docx(bytes).map_err(|err| {
        IntelloError::validation(
            "document",
            format!("Failed to read Word document '{}': {}", filename, err),
        )
    })?;

    // Step 2: Extract text from all paragraphs
    let mut content = String::new();
    for child in docx.document.children {
        if let docx_rs::DocumentChild::Paragraph(para) = child {
            for run_child in para.children {
                if let docx_rs::ParagraphChild::Run(run) = run_child {
                    for run_content in run.children {
                        if let docx_rs::RunChild::Text(text) = run_content {
                            content.push_str(&text.text);
                        }
                    }
                }
            }
            content.push('\n');
        }
    }

    // Step 3: Validate extracted content is not empty
    if content.trim().is_empty() {
        return Err(IntelloError::validation(
            "document",
            format!(
                "Word document '{}' contains no extractable text",
                filename
            ),
        ));
    }

    // Step 4: Return extracted content with token estimate
    Ok(ExtractedContent::new(content))
}

// ** extract_pptx **
// Extracts text content from a PowerPoint (.pptx) file
// ==> PPTX files are ZIP archives containing XML files with text in <a:t> tags
// @ bytes : Raw PPTX file bytes received from HTTP request
// @ filename : Original filename (for error messages)
// @ returns : ExtractedContent with the extracted text and estimated token count
// @ edge cases : Presentations with no text content, complex slide layouts
// @ error conditions : Invalid PPTX format, ZIP extraction failures
pub fn extract_pptx(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    use std::io::{Cursor, Read};

    // Step 1: Open PPTX as ZIP archive
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor).map_err(|err| {
        IntelloError::validation(
            "document",
            format!("Failed to read PowerPoint '{}': {}", filename, err),
        )
    })?;

    let mut content = String::new();
    let mut slide_num = 1;

    // Step 2: Collect and sort slide file names
    let mut slide_files: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            archive.by_index(i).ok().and_then(|f| {
                let name = f.name().to_string();
                if name.starts_with("ppt/slides/slide")
                    && name.ends_with(".xml")
                {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect();
    slide_files.sort();

    // Step 3: Extract text from each slide XML
    for slide_path in slide_files {
        let mut file = archive.by_name(&slide_path).map_err(|err| {
            IntelloError::validation(
                "document",
                format!("Failed to read slide in '{}': {}", filename, err),
            )
        })?;

        let mut xml_content = String::new();
        file.read_to_string(&mut xml_content).map_err(|err| {
            IntelloError::validation(
                "document",
                format!(
                    "Failed to read slide content in '{}': {}",
                    filename, err
                ),
            )
        })?;

        // Extract text from <a:t> tags (PowerPoint text elements)
        let slide_text = extract_text_from_xml(&xml_content);
        if !slide_text.trim().is_empty() {
            content.push_str(&format!("--- Slide {} ---\n", slide_num));
            content.push_str(&slide_text);
            content.push_str("\n\n");
        }
        slide_num += 1;
    }

    // Step 4: Validate extracted content is not empty
    if content.trim().is_empty() {
        return Err(IntelloError::validation(
            "document",
            format!("PowerPoint '{}' contains no extractable text", filename),
        ));
    }

    // Step 5: Return extracted content with token estimate
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
        let result = extract_txt(bytes, "test.txt");

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().content, "Hello, world!");
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
        let result = extract_txt(bytes, "latin.txt");

        // assert
        assert!(result.is_ok());
        assert!(!result.unwrap().content.is_empty());
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

    // -- extract_pptx tests (error case only) --

    #[test]
    fn test_extract_pptx_invalid_bytes_returns_error() {
        // arrange
        let bytes = b"not a real pptx";

        // act
        let result = extract_pptx(bytes, "bad.pptx");

        // assert
        assert!(result.is_err());
    }
}
