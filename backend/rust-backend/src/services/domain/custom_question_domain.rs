//! Custom Question domain entity
//!
//! Represents a configuration for AI-generated questions from documents.
//! Users can upload multiple documents (PDF, Word, TXT) and the system
//! will extract text and send it to an AI model to generate questions.

use super::enums_domain::Level;
use serde::{Deserialize, Serialize};

// ** DocumentType **
// ==> Supported document types for content extraction
//
// @ Pdf : Portable Document Format files
// @ Word : Microsoft Word (.docx) files
// @ Text : Plain text (.txt) files
// @ PowerPoint : PowerPoint presentation (.pptx) files
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    Pdf,
    Word,
    Text,
    PowerPoint,
}

// ** CustomQuestionDocument **
// ==> A document attached to a custom question configuration
//
// @ filename : Original filename of the uploaded document
// @ doc_type : Type of document (PDF, Word, Text, PowerPoint)
// @ content : Extracted text content from the document
// @ token_count : Estimated token count for this document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomQuestionDocument {
    pub filename: String,       // original filename
    pub doc_type: DocumentType, // document type
    pub content: String,        // extracted text
    pub token_count: u32,       // token count
}

// ** CustomQuestion **
// ==> Configuration for AI-generated question content from documents
//
// @ id : Unique identifier for this configuration
// @ user_id : Owner user ID
// @ name : Name of the custom question set
// @ description : Description of what this generates
// @ instructions : Specific instructions for the AI model
// @ language : Language for generated questions (e.g., "en", "fr")
// @ level : Difficulty level (Easy, Medium, Hard)
// @ output_game : Target game type (e.g., "qcm", "flashcard")
// @ subjects : Subject tags for the questions (max 3)
// @ num_questions : Number of questions to generate (5, 10, 15, 20, 25, or 30)
// @ documents : Attached documents for content extraction
// @ total_token_count : Total token count across all documents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomQuestion {
    pub id: String,                             // unique identifier
    pub user_id: String,                        // owner
    pub name: String,                           // set name
    pub description: String,                    // description
    pub instructions: String,                   // AI instructions
    pub language: String,                       // language code
    pub level: Level,                           // difficulty level
    pub output_game: String,                    // target game type
    pub subjects: Vec<String>,                  // subject tags
    pub num_questions: u8,                      // question count
    pub documents: Vec<CustomQuestionDocument>, // attached documents
    pub total_token_count: u32,                 // total tokens
}
