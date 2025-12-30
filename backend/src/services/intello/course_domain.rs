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

/// Input for creating a new course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCourseInput {
    pub name: String,
    pub description: String,
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
