use crate::services::intello::study_session_domain::StudySession;
use crate::shared::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait StudySessionRepository: Send + Sync {
    /// Create a study session
    async fn create(&self, session: &StudySession) -> Result<StudySession, AppError>;

    /// List all sessions for a course (ordered by newest first)
    async fn list_by_course(&self, course_id: &str) -> Result<Vec<StudySession>, AppError>;
}
