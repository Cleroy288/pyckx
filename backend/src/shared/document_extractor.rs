//! Document text extraction utilities
//!
//! Extracts text content from various document formats (TXT, PDF, Word).
//! Designed to work with file bytes received via HTTP multipart requests.

use crate::error::IntelloError;

/// Result type for document extraction operations
pub type ExtractResult<T> = Result<T, IntelloError>;

/// Extracted document content with metadata
#[derive(Debug, Clone)]
pub struct ExtractedContent {
    /// The extracted text content
    pub content: String,
    /// Estimated token count (rough: ~4 chars per token)
    pub token_count: u32,
}

impl ExtractedContent {
    /// Create new extracted content and calculate token count
    pub fn new(content: String) -> Self {
        // Rough token estimation: ~4 characters per token (common for English)
        let token_count = (content.len() as f64 / 4.0).ceil() as u32;
        Self { content, token_count }
    }
}

/// Extract text content from a .txt file
///
/// # Arguments
/// * `bytes` - Raw file bytes received from HTTP request
/// * `filename` - Original filename (for error messages)
///
/// # Returns
/// * `ExtractedContent` with the text and estimated token count
pub fn extract_txt(bytes: &[u8], filename: &str) -> ExtractResult<ExtractedContent> {
    // Try UTF-8 first
    match std::str::from_utf8(bytes) {
        Ok(text) => {
            let content = text.to_string();
            if content.trim().is_empty() {
                return Err(IntelloError::validation(
                    "document",
                    format!("File '{}' is empty or contains only whitespace", filename),
                ));
            }
            Ok(ExtractedContent::new(content))
        }
        Err(_) => {
            // Try Latin-1 (ISO-8859-1) as fallback - common for older documents
            let content: String = bytes.iter().map(|&b| b as char).collect();
            if content.trim().is_empty() {
                return Err(IntelloError::validation(
                    "document",
                    format!("File '{}' is empty or contains only whitespace", filename),
                ));
            }
            Ok(ExtractedContent::new(content))
        }
    }
}

/// Extract text content from a PDF file
///
/// # Arguments
/// * `bytes` - Raw PDF file bytes received from HTTP request
/// * `filename` - Original filename (for error messages)
///
/// # Returns
/// * `ExtractedContent` with the extracted text and estimated token count
pub fn extract_pdf(bytes: &[u8], filename: &str) -> ExtractResult<ExtractedContent> {
    let content = pdf_extract::extract_text_from_mem(bytes).map_err(|e| {
        IntelloError::validation(
            "document",
            format!("Failed to extract text from PDF '{}': {}", filename, e),
        )
    })?;

    if content.trim().is_empty() {
        return Err(IntelloError::validation(
            "document",
            format!("PDF '{}' contains no extractable text", filename),
        ));
    }

    Ok(ExtractedContent::new(content))
}

/// Extract text content from a Word (.docx) file
///
/// # Arguments
/// * `bytes` - Raw DOCX file bytes received from HTTP request
/// * `filename` - Original filename (for error messages)
///
/// # Returns
/// * `ExtractedContent` with the extracted text and estimated token count
pub fn extract_word(bytes: &[u8], filename: &str) -> ExtractResult<ExtractedContent> {
    let docx = docx_rs::read_docx(bytes).map_err(|e| {
        IntelloError::validation(
            "document",
            format!("Failed to read Word document '{}': {}", filename, e),
        )
    })?;

    // Extract text from all paragraphs
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

    if content.trim().is_empty() {
        return Err(IntelloError::validation(
            "document",
            format!("Word document '{}' contains no extractable text", filename),
        ));
    }

    Ok(ExtractedContent::new(content))
}

/// Extract text content from a PowerPoint (.pptx) file
///
/// PPTX files are ZIP archives containing XML files.
/// Text is stored in ppt/slides/slide*.xml files within <a:t> tags.
///
/// # Arguments
/// * `bytes` - Raw PPTX file bytes received from HTTP request
/// * `filename` - Original filename (for error messages)
///
/// # Returns
/// * `ExtractedContent` with the extracted text and estimated token count
pub fn extract_pptx(bytes: &[u8], filename: &str) -> ExtractResult<ExtractedContent> {
    use std::io::{Cursor, Read};

    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor).map_err(|e| {
        IntelloError::validation(
            "document",
            format!("Failed to read PowerPoint '{}': {}", filename, e),
        )
    })?;

    let mut content = String::new();
    let mut slide_num = 1;

    // Collect slide file names and sort them
    let mut slide_files: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            archive.by_index(i).ok().and_then(|f| {
                let name = f.name().to_string();
                if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect();
    slide_files.sort();

    // Extract text from each slide
    for slide_path in slide_files {
        let mut file = archive.by_name(&slide_path).map_err(|e| {
            IntelloError::validation(
                "document",
                format!("Failed to read slide in '{}': {}", filename, e),
            )
        })?;

        let mut xml_content = String::new();
        file.read_to_string(&mut xml_content).map_err(|e| {
            IntelloError::validation(
                "document",
                format!("Failed to read slide content in '{}': {}", filename, e),
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

    if content.trim().is_empty() {
        return Err(IntelloError::validation(
            "document",
            format!("PowerPoint '{}' contains no extractable text", filename),
        ));
    }

    Ok(ExtractedContent::new(content))
}

/// Extract text content from XML by finding all <a:t> tags
/// This is a simple parser that extracts text between <a:t> and </a:t> tags
fn extract_text_from_xml(xml: &str) -> String {
    let mut result = String::new();
    let mut in_text_tag = false;
    let mut chars = xml.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            // Check if this is <a:t> or </a:t>
            let mut tag = String::new();
            while let Some(&next) = chars.peek() {
                if next == '>' {
                    chars.next();
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            if tag == "a:t" {
                in_text_tag = true;
            } else if tag == "/a:t" {
                in_text_tag = false;
                result.push(' '); // Add space between text elements
            } else if tag == "a:p" || tag == "/a:p" {
                // Paragraph boundary - add newline
                if !result.ends_with('\n') && !result.is_empty() {
                    result.push('\n');
                }
            }
        } else if in_text_tag {
            result.push(c);
        }
    }

    result
}
