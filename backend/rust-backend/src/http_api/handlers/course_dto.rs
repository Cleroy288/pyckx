//! Course & Session DTO types
//!
//! Request/response types for course and session
//! CRUD endpoints.

use serde::{Deserialize, Serialize};

/// Request body for creating a new course.
#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// Request body for uploading a resource file.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UploadResourceRequest {
    pub filename: String,
    pub content: String,
    #[serde(default)]
    pub token_count: i32,
}

/// Serializable course entity for API responses.
#[derive(Debug, Serialize)]
pub struct Course {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Serializable resource entity for API responses.
#[derive(Debug, Serialize)]
pub struct Resource {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub content: String,
    pub token_count: i32,
    pub created_at: String,
}

/// Response wrapper for a list of courses.
#[derive(Debug, Serialize)]
pub struct CourseListResponse {
    pub success: bool,
    pub courses: Vec<Course>,
}

/// Response wrapper for a single course.
#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub success: bool,
    pub course: Course,
}

/// Response wrapper for a list of resources.
#[derive(Debug, Serialize)]
pub struct ResourceListResponse {
    pub success: bool,
    pub resources: Vec<Resource>,
}

/// Response wrapper for a single resource.
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ResourceResponse {
    pub success: bool,
    pub resource: Resource,
}

/// Request body for creating a new session.
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub topic: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default = "default_language")]
    pub language: String,
}

/// Returns "en" as the default language code.
fn default_language() -> String {
    "en".to_string()
}

/// Serializable session entity for API responses.
#[derive(Debug, Serialize)]
pub struct Session {
    pub id: String,
    pub course_id: String,
    pub topic: String,
    pub instructions: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_content: Option<serde_json::Value>,
    pub created_at: String,
}

/// Response wrapper for a single session.
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub success: bool,
    pub session: Session,
}

/// Response wrapper for a list of sessions.
#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub success: bool,
    pub sessions: Vec<Session>,
}
