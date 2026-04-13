//! Test data factories for Study domain types

use crate::services::course::domain::{
    Course, UserResource,
};
use crate::services::study_session::{
    study_session_domain::StudySession,
};

/// Create a test Course
pub fn make_course(
    id: &str,
    user_id: &str,
) -> Course {
    Course {
        id: id.to_string(),
        user_id: user_id.to_string(),
        name: format!("Course {id}"),
        description: "Test course".to_string(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    }
}

/// Create a test UserResource
pub fn make_resource(
    id: &str,
    user_id: &str,
    filename: &str,
) -> UserResource {
    UserResource {
        id: id.to_string(),
        user_id: user_id.to_string(),
        filename: filename.to_string(),
        content: "test content".to_string(),
        token_count: 100,
        created_at: "2024-01-01T00:00:00Z".to_string(),
    }
}

/// Create a test StudySession
pub fn make_session(
    id: &str,
    course_id: &str,
) -> StudySession {
    StudySession {
        id: id.to_string(),
        course_id: course_id.to_string(),
        topic: "Test topic".to_string(),
        instructions: "Test instructions".to_string(),
        keywords: vec!["test".to_string()],
        language: "en".to_string(),
        status: "in_progress".to_string(),
        generated_content: None,
        extracted_knowledge: None,
        educational_content: None,
        expanded_knowledge: None,
        created_at: "2024-01-01T00:00:00Z".to_string(),
    }
}
