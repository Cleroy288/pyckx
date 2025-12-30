//! Study session operations

use super::types_domain::IntelloService;
use crate::services::intello::study_session_domain::{CreateStudySessionInput, StudySession};
use crate::services::intello::error_domain::IntelloError;
use tracing::{info, instrument};
use uuid::Uuid;

impl IntelloService {
    /// Create a new study session for a course
    #[instrument(skip(self, input), fields(user_id = %user_id, course_id = %course_id, topic = %input.topic))]
    pub async fn create_study_session(
        &self,
        user_id: &str,
        course_id: &str,
        input: CreateStudySessionInput,
    ) -> Result<StudySession, IntelloError> {
        self.validate_user_id(user_id)?;

        info!("Creating study session");

        let session = StudySession {
            id: Uuid::new_v4().to_string(),
            course_id: course_id.to_string(),
            topic: input.topic,
            instructions: input.instructions,
            keywords: input.keywords,
            language: input.language,
            status: "in_progress".to_string(),
            created_at: String::new(), // Set by database
        };

        let created = self
            .study_session_repo
            .create(&session)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        info!(session_id = %created.id, "Study session created");
        Ok(created)
    }

    /// List all study sessions for a course
    #[instrument(skip(self), fields(user_id = %user_id, course_id = %course_id))]
    pub async fn list_course_sessions(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<Vec<StudySession>, IntelloError> {
        self.validate_user_id(user_id)?;

        info!("Listing course sessions");

        let sessions = self
            .study_session_repo
            .list_by_course(course_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        info!(count = sessions.len(), "Course sessions listed");
        Ok(sessions)
    }
}
