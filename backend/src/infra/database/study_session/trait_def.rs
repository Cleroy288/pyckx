use crate::services::intello::study_session::study_session_domain::StudySession;
use crate::shared::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait StudySessionRepository: Send + Sync {
    /// Create a study session
    async fn create(&self, session: &StudySession) -> Result<StudySession, AppError>;

    /// List all sessions for a course (ordered by newest first)
    async fn list_by_course(&self, course_id: &str) -> Result<Vec<StudySession>, AppError>;

    /// Get a session by ID
    async fn get(&self, session_id: &str) -> Result<StudySession, AppError>;

    /// Save generated content to a session
    async fn save_session_content(&self, session_id: &str, content: &serde_json::Value) -> Result<(), AppError>;

    /// Delete all sessions for a course
    async fn delete_by_course(&self, course_id: &str) -> Result<(), AppError>;

    /// Delete a single session by ID
    async fn delete(&self, session_id: &str) -> Result<(), AppError>;
}
