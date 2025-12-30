use serde::{Deserialize, Serialize};

/// Study session within a course
///
/// Represents a learning session tied to a course, containing
/// user-defined topics, keywords, and instructions for AI-generated content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StudySession {
    pub id: String,
    pub course_id: String,
    pub topic: String,
    pub instructions: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub status: String,
    pub created_at: String,
}

/// Input for creating a study session
#[derive(Debug, Clone)]
pub struct CreateStudySessionInput {
    pub topic: String,
    pub instructions: String,
    pub keywords: Vec<String>,
    pub language: String,
}
