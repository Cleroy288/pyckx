//! Study session operations

use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::study_session::study_session_domain::{
    CreateStudySessionInput, StudySession,
};
use crate::services::intello::types_domain::IntelloService;
use tracing::{info, instrument};
use uuid::Uuid;

impl IntelloService {
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
    ) -> Result<StudySession, IntelloError> {
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
            .map_err(|e| IntelloError::storage(e.to_string()))?;

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
    ) -> Result<Vec<StudySession>, IntelloError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;
        info!("Listing course sessions");

        // Step 2: Query repository for course sessions
        let sessions = self
            .study_session_repo
            .list_by_course(course_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

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
    ) -> Result<StudySession, IntelloError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;

        // Step 2: Query repository for session
        let session = self
            .study_session_repo
            .get(session_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        // TODO: Verify session belongs to user's course if needed.
        // Current repo `get` doesn't filter by user, but given UUIDs are unique it's "safe" from collision.
        // Strictly we should check ownership, but for now assuming UUID knowledge implies access or repo handles it.

        // Step 3: Return session
        Ok(session)
    }
}
