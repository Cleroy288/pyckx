//! Document Extractor Module
//!
//! Provides document text extraction functionality
// ==> This module contains functions for extracting text from various document formats
//
// @ document_extractor_domain : Domain structures for document extraction
// @ document_extractor_service : Main extraction functions for different file formats
// @ document_extractor_utils : Utility functions used across extraction operations

pub mod document_extractor_domain;
pub mod document_extractor_service;
pub mod document_extractor_utils;

pub use document_extractor_service::*;
