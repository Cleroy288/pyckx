//! Document Extractor Module
//!
//! Text extraction from various document formats (TXT,
//! PDF, DOCX, PPTX).

pub mod document_extractor_domain;
pub mod document_extractor_service;
pub mod document_extractor_utils;
pub mod extractor_pptx;

pub use document_extractor_service::{
    extract_pdf, extract_txt, extract_word,
};
pub use extractor_pptx::extract_pptx;
