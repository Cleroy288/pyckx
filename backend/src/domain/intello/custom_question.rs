//! Custom Question domain entity
//!
//! Represents a configuration for AI-generated questions from documents.
//! Users can upload multiple documents (PDF, Word, TXT) and the system
//! will extract text and send it to an AI model to generate questions.

use serde::{Deserialize, Serialize};
use super::enums::Level;

/// Supported document types for content extraction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    Pdf,
    Word,
    Text,
    PowerPoint,
}

/// A document attached to a custom question
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomQuestionDocument {
    /// Original filename
    pub filename: String,
    /// Document type
    pub doc_type: DocumentType,
    /// Extracted text content
    pub content: String,
    /// Token count for this document
    pub token_count: u32,
}

/// A custom question configuration for AI-generated content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomQuestion {
    /// Unique identifier
    pub id: String,
    /// User ID who owns this configuration
    pub user_id: String,
    /// Name of the custom question set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated questions (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Output game ID (references AVAILABLE_GAMES, e.g., "qcm")
    pub output_game: String,
    /// Subjects/topics for the questions (max 3)
    pub subjects: Vec<String>,
    /// Number of questions to generate (5, 10, 15, 20, 25, or 30)
    pub num_questions: u8,
    /// Attached documents (multiple files, max 800k tokens total)
    pub documents: Vec<CustomQuestionDocument>,
    /// Total token count across all documents
    pub total_token_count: u32,
}


