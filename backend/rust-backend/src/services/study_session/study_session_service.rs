//! Study session operations

use crate::services::error_domain::StudyError;
use crate::services::study_session::study_session_domain::{
    CreateStudySessionInput, StudySession,
};
use crate::services::StudyService;
use tracing::{info, instrument};
use uuid::Uuid;

impl StudyService {
    // ** create_study_session **
    // ==> Creates a new study session for a specific course
    //
    // @ user_id : The user creating the session
    // @ course_id : The course this session belongs to
    // @ input : Session creation parameters (topic, instructions, etc.)
    // @ returns : The created StudySession
    // @ errors : ValidationFailed if user_id invalid, StorageError if creation fails
    #[instrument(skip(self, input), fields(user_id = %user_id, course_id = %course_id, topic = %input.topic))]
    pub async fn create_study_session(
        &self,
        user_id: &str,
        course_id: &str,
        input: CreateStudySessionInput,
    ) -> Result<StudySession, StudyError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;
        info!("Creating study session");

        // Step 2: Build study session object
        let session = StudySession {
            id: Uuid::new_v4().to_string(),
            course_id: course_id.to_string(),
            topic: input.topic,
            instructions: input.instructions,
            keywords: input.keywords,
            language: input.language,
            status: "in_progress".to_string(),
            generated_content: None,
            extracted_knowledge: None,
            educational_content: None,
            expanded_knowledge: None,
            created_at: String::new(), // Set by database
        };

        // Step 3: Persist session to database
        let created = self
            .study_session_repo
            .create(&session)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // Step 4: Log success and return created session
        info!(session_id = %created.id, "Study session created");
        Ok(created)
    }

    // ** list_course_sessions **
    // ==> Lists all study sessions for a specific course
    //
    // @ user_id : The user requesting the list
    // @ course_id : The course to list sessions for
    // @ returns : Vector of all StudySessions for the course
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id, course_id = %course_id))]
    pub async fn list_course_sessions(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<Vec<StudySession>, StudyError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;
        info!("Listing course sessions");

        // Step 2: Query repository for course sessions
        let sessions = self
            .study_session_repo
            .list_by_course(course_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // Step 3: Log count and return sessions
        info!(count = sessions.len(), "Course sessions listed");
        Ok(sessions)
    }

    // ** get_study_session **
    // ==> Retrieves a specific study session by ID
    //
    // @ user_id : The user requesting the session
    // @ session_id : The session ID to retrieve
    // @ returns : The requested StudySession
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id, session_id = %session_id))]
    pub async fn get_study_session(
        &self,
        user_id: &str,
        session_id: &str,
    ) -> Result<StudySession, StudyError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;

        // Step 2: Query repository for session
        let session = self
            .study_session_repo
            .get(session_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // Step 3: Return session
        Ok(session)
    }

    /// Deletes a study session by ID
    #[instrument(skip(self), fields(user_id = %user_id, session_id = %session_id))]
    pub async fn delete_study_session(
        &self,
        user_id: &str,
        session_id: &str,
    ) -> Result<(), StudyError> {
        self.validate_user_id(user_id)?;
        info!("Deleting study session");

        self.study_session_repo
            .delete(session_id)
            .await
            .map_err(|err| {
                StudyError::storage(err.to_string())
            })?;

        info!("Study session deleted");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::test_support::{
        build_test_service, make_session,
        MemCourseRepo, MemSessionRepo,
    };
    use std::sync::Arc;

    // ── Helpers ──────────────────────────────────

    /// Build service with pre-seeded sessions
    fn svc_with_sessions(
        sessions: Vec<StudySession>,
    ) -> (StudyService, Arc<MemSessionRepo>) {
        let repo = Arc::new(
            MemSessionRepo::with_sessions(sessions),
        );
        let svc = build_test_service(
            Arc::new(MemCourseRepo::new()),
            repo.clone(),
        );
        (svc, repo)
    }

    /// Create a valid input for session creation
    fn valid_input() -> CreateStudySessionInput {
        CreateStudySessionInput {
            topic: "Algebra".into(),
            instructions: "Solve equations".into(),
            keywords: vec!["math".into()],
            language: "en".into(),
        }
    }

    // ── create_study_session ─────────────────────

    #[tokio::test]
    async fn test_create_study_session_success() {
        let (svc, _) = svc_with_sessions(vec![]);

        let result = svc
            .create_study_session(
                "u1", "c1", valid_input(),
            )
            .await
            .unwrap();

        assert_eq!(result.topic, "Algebra");
    }

    #[tokio::test]
    async fn test_create_study_session_sets_course_id() {
        let (svc, _) = svc_with_sessions(vec![]);

        let result = svc
            .create_study_session(
                "u1", "c99", valid_input(),
            )
            .await
            .unwrap();

        assert_eq!(result.course_id, "c99");
    }

    #[tokio::test]
    async fn test_create_study_session_empty_user_id() {
        let (svc, _) = svc_with_sessions(vec![]);

        let err = svc
            .create_study_session(
                "", "c1", valid_input(),
            )
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            StudyError::ValidationFailed { .. }
        ));
    }

    // ── list_course_sessions ─────────────────────

    #[tokio::test]
    async fn test_list_course_sessions_empty() {
        let (svc, _) = svc_with_sessions(vec![]);

        let result = svc
            .list_course_sessions("u1", "c1")
            .await
            .unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_list_course_sessions_returns_matching() {
        let s1 = make_session("s1", "c1");
        let s2 = make_session("s2", "c2");
        let (svc, _) = svc_with_sessions(vec![s1, s2]);

        let result = svc
            .list_course_sessions("u1", "c1")
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_list_course_sessions_empty_user_id() {
        let (svc, _) = svc_with_sessions(vec![]);

        let err = svc
            .list_course_sessions("", "c1")
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            StudyError::ValidationFailed { .. }
        ));
    }

    // ── get_study_session ────────────────────────

    #[tokio::test]
    async fn test_get_study_session_found() {
        let s = make_session("s1", "c1");
        let (svc, _) = svc_with_sessions(vec![s]);

        let result = svc
            .get_study_session("u1", "s1")
            .await
            .unwrap();

        assert_eq!(result.id, "s1");
    }

    #[tokio::test]
    async fn test_get_study_session_not_found() {
        let (svc, _) = svc_with_sessions(vec![]);

        let err = svc
            .get_study_session("u1", "missing")
            .await;

        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_get_study_session_empty_user_id() {
        let (svc, _) = svc_with_sessions(vec![]);

        let err = svc
            .get_study_session("", "s1")
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            StudyError::ValidationFailed { .. }
        ));
    }

    // ── delete_study_session ─────────────────────

    #[tokio::test]
    async fn test_delete_study_session_success() {
        let s = make_session("s1", "c1");
        let (svc, repo) = svc_with_sessions(vec![s]);

        svc.delete_study_session("u1", "s1")
            .await
            .unwrap();

        let remaining = repo.sessions.lock().unwrap();
        assert!(remaining.is_empty());
    }

    #[tokio::test]
    async fn test_delete_study_session_empty_user_id() {
        let (svc, _) = svc_with_sessions(vec![]);

        let err = svc
            .delete_study_session("", "s1")
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            StudyError::ValidationFailed { .. }
        ));
    }

    // ── Ownership gap documentation ──────────────
    // NOTE: get_study_session and delete_study_session
    // do NOT verify that the session belongs to the
    // requesting user. Any valid user_id can access or
    // delete any session. This is a known authorization
    // gap — the service only validates user_id is
    // non-empty but never checks ownership against
    // the session's course owner.

    #[tokio::test]
    async fn test_get_session_no_ownership_check() {
        let s = make_session("s1", "c1");
        let (svc, _) = svc_with_sessions(vec![s]);

        // A different user can access the session
        let result = svc
            .get_study_session("other-user", "s1")
            .await;

        // This SUCCEEDS — ownership is NOT enforced
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_session_no_ownership_check() {
        let s = make_session("s1", "c1");
        let (svc, repo) = svc_with_sessions(vec![s]);

        // A different user can delete the session
        svc.delete_study_session("other-user", "s1")
            .await
            .unwrap();

        let remaining = repo.sessions.lock().unwrap();
        // This SUCCEEDS — ownership is NOT enforced
        assert!(remaining.is_empty());
    }
}
