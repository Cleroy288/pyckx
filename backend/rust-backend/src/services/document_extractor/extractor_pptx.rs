//! PowerPoint (.pptx) Extractor
//!
//! Extracts text from PPTX files by reading slide XML
//! entries from the ZIP archive.

use std::io::{Cursor, Read};

use crate::services::document_extractor::{
    document_extractor_domain::{ExtractResult, ExtractedContent},
    document_extractor_service::require_non_empty,
    document_extractor_utils::extract_text_from_xml,
};
use crate::services::error_domain::StudyError;

/// ZIP archive over an in-memory byte slice.
type PptxArchive<'a> = zip::ZipArchive<Cursor<&'a [u8]>>;

/// Extracts text from a PowerPoint (.pptx) file.
pub fn extract_pptx(
    bytes: &[u8],
    filename: &str,
) -> ExtractResult<ExtractedContent> {
    let cursor = Cursor::new(bytes);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|err| {
            StudyError::validation(
                "document",
                format!(
                    "Failed to read PowerPoint '{}': {}",
                    filename, err
                ),
            )
        })?;

    let slide_files = collect_slide_paths(&mut archive);
    let content =
        extract_slides_text(&mut archive, &slide_files, filename)?;

    require_non_empty(&content, filename, "PowerPoint")?;
    Ok(ExtractedContent::new(content))
}

/// Collects and sorts slide XML paths from the archive.
fn collect_slide_paths(
    archive: &mut PptxArchive<'_>,
) -> Vec<String> {
    let mut paths: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            archive.by_index(i).ok().and_then(|f| {
                let n = f.name().to_string();
                if n.starts_with("ppt/slides/slide")
                    && n.ends_with(".xml")
                {
                    Some(n)
                } else {
                    None
                }
            })
        })
        .collect();
    paths.sort();
    paths
}

/// Reads and concatenates text from each slide XML.
fn extract_slides_text(
    archive: &mut PptxArchive<'_>,
    slide_files: &[String],
    filename: &str,
) -> ExtractResult<String> {
    let mut content = String::new();

    for (idx, path) in slide_files.iter().enumerate() {
        let xml = read_slide_xml(archive, path, filename)?;
        let text = extract_text_from_xml(&xml);

        if !text.trim().is_empty() {
            content.push_str(
                &format!("--- Slide {} ---\n", idx + 1),
            );
            content.push_str(&text);
            content.push_str("\n\n");
        }
    }

    Ok(content)
}

/// Reads a single slide XML entry from the ZIP archive.
fn read_slide_xml(
    archive: &mut PptxArchive<'_>,
    path: &str,
    filename: &str,
) -> ExtractResult<String> {
    let mut file =
        archive.by_name(path).map_err(|err| {
            StudyError::validation(
                "document",
                format!(
                    "Failed to read slide in '{}': {}",
                    filename, err
                ),
            )
        })?;

    let mut xml = String::new();
    file.read_to_string(&mut xml).map_err(|err| {
        StudyError::validation(
            "document",
            format!(
                "Failed to read slide content in '{}': {}",
                filename, err
            ),
        )
    })?;

    Ok(xml)
}

#[cfg(test)]
mod tests {
    use super::*;

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
