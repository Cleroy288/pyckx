use serde::{Deserialize, Serialize};

/// Course entity
/// 
/// A course is a container for learning resources and study sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Course {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Resource entity
/// 
/// A resource is a document (PDF, Text, etc.) attached to a course.
/// It contains the raw content and token count for AI processing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resource {
    pub id: String,
    pub course_id: String,
    pub filename: String,
    pub content: String,
    pub token_count: i32,
    pub created_at: String,
}

/// Input for creating a new course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCourseInput {
    pub name: String,
    pub description: String,
}

/// Input for adding a resource to a course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddResourceInput {
    pub filename: String,
    pub content: String,
    pub token_count: i32,
}

/// Summary of a resource (without content for list views)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceSummary {
    pub id: String,
    pub filename: String,
    pub token_count: i32,
    pub created_at: String,
}

/// User-scoped resource (not tied to a specific course)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserResource {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub content: String,
    pub token_count: i32,
    pub created_at: String,
}
