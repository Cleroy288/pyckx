//! Helper functions for intello handlers

use crate::services::intello::document_extractor::{
    extract_pdf, extract_pptx, extract_txt, extract_word,
};
use crate::services::intello::DocumentType;
use crate::shared::{AppError, AppResult};
use actix_multipart::Multipart;
use futures_util::StreamExt;

/// Parse multipart form data with metadata and file uploads
/// Returns (metadata, documents) where documents is Vec<(filename, content, token_count)>
pub async fn parse_multipart<T: serde::de::DeserializeOwned>(
    mut payload: Multipart,
) -> AppResult<(T, Vec<(String, String, u32)>)> {
    let mut metadata: Option<T> = None;
    let mut documents: Vec<(String, String, u32)> = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| AppError::validation("multipart", e.to_string()))?;

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };
        let field_name = content_disposition.get_name().unwrap_or("");

        match field_name {
            "metadata" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk =
                        chunk.map_err(|e| AppError::validation("metadata", e.to_string()))?;
                    bytes.extend_from_slice(&chunk);
                }
                metadata = Some(
                    serde_json::from_slice(&bytes)
                        .map_err(|e| AppError::validation("metadata", e.to_string()))?,
                );
            }
            "files" => {
                let filename = content_disposition
                    .get_filename()
                    .unwrap_or("unknown")
                    .to_string();

                // Determine document type from extension
                let doc_type = get_document_type(&filename)?;

                // Read file bytes
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.map_err(|e| AppError::validation("files", e.to_string()))?;
                    bytes.extend_from_slice(&chunk);
                }

                // Extract text based on document type
                let extracted = match doc_type {
                    DocumentType::Text => extract_txt(&bytes, &filename)?,
                    DocumentType::Pdf => extract_pdf(&bytes, &filename)?,
                    DocumentType::Word => extract_word(&bytes, &filename)?,
                    DocumentType::PowerPoint => extract_pptx(&bytes, &filename)?,
                };

                documents.push((filename, extracted.content, extracted.token_count));
            }
            _ => {}
        }
    }

    let metadata = metadata
        .ok_or_else(|| AppError::validation("metadata", "Missing metadata field in request"))?;

    Ok((metadata, documents))
}

/// Get document type from filename extension
pub fn get_document_type(filename: &str) -> AppResult<DocumentType> {
    if filename.ends_with(".pdf") {
        Ok(DocumentType::Pdf)
    } else if filename.ends_with(".docx") || filename.ends_with(".doc") {
        Ok(DocumentType::Word)
    } else if filename.ends_with(".txt") {
        Ok(DocumentType::Text)
    } else if filename.ends_with(".pptx") || filename.ends_with(".ppt") {
        Ok(DocumentType::PowerPoint)
    } else {
        Err(AppError::validation(
            "files",
            format!(
                "Unsupported file type: {}. Supported: .txt, .pdf, .docx, .pptx",
                filename
            ),
        ))
    }
}

/// Parse multipart form data for file uploads only (no metadata required)
/// Returns documents as Vec<(filename, content, token_count)>
///
/// TODO: Uncomment when course upload endpoint is implemented
#[allow(dead_code)]
pub async fn parse_multipart_files_only(
    mut payload: Multipart,
) -> AppResult<Vec<(String, String, u32)>> {
    let mut documents: Vec<(String, String, u32)> = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| AppError::validation("multipart", e.to_string()))?;

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };
        let field_name = content_disposition.get_name().unwrap_or("");

        if field_name == "files" {
            let filename = content_disposition
                .get_filename()
                .unwrap_or("unknown")
                .to_string();

            // Determine document type from extension
            let doc_type = get_document_type(&filename)?;

            // Read file bytes
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await {
                let chunk = chunk.map_err(|e| AppError::validation("files", e.to_string()))?;
                bytes.extend_from_slice(&chunk);
            }

            // Extract text based on document type
            let extracted = match doc_type {
                DocumentType::Text => extract_txt(&bytes, &filename)?,
                DocumentType::Pdf => extract_pdf(&bytes, &filename)?,
                DocumentType::Word => extract_word(&bytes, &filename)?,
                DocumentType::PowerPoint => extract_pptx(&bytes, &filename)?,
            };

            documents.push((filename, extracted.content, extracted.token_count));
        }
    }

    if documents.is_empty() {
        return Err(AppError::validation(
            "files",
            "No files provided in the upload",
        ));
    }

    Ok(documents)
}
