//! In-memory stub for StudySessionRepository

use crate::infra::StudySessionRepository;
use crate::services::study_session::{
    study_session_domain::StudySession,
};
use crate::shared::AppError;
use async_trait::async_trait;
use std::sync::Mutex;

/// In-memory stub for StudySessionRepository
pub struct MemSessionRepo {
    /// Stored sessions
    pub sessions: Mutex<Vec<StudySession>>,
}

impl MemSessionRepo {
    /// Empty repo
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(vec![]),
        }
    }

    /// Pre-seed sessions
    pub fn with_sessions(
        sessions: Vec<StudySession>,
    ) -> Self {
        Self {
            sessions: Mutex::new(sessions),
        }
    }
}

#[async_trait]
impl StudySessionRepository for MemSessionRepo {
    async fn create(
        &self,
        session: &StudySession,
    ) -> Result<StudySession, AppError> {
        self.sessions
            .lock()
            .unwrap()
            .push(session.clone());
        Ok(session.clone())
    }

    async fn list_by_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<StudySession>, AppError> {
        let store = self.sessions.lock().unwrap();
        Ok(store
            .iter()
            .filter(|s| s.course_id == course_id)
            .cloned()
            .collect())
    }

    async fn get(
        &self,
        session_id: &str,
    ) -> Result<StudySession, AppError> {
        let store = self.sessions.lock().unwrap();
        store
            .iter()
            .find(|s| s.id == session_id)
            .cloned()
            .ok_or(AppError::Internal(
                crate::http_api::utils::InternalError::new(
                    "Session not found",
                ),
            ))
    }

    async fn save_session_content(
        &self,
        _session_id: &str,
        _content: &serde_json::Value,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn delete_by_course(
        &self,
        course_id: &str,
    ) -> Result<(), AppError> {
        self.sessions
            .lock()
            .unwrap()
            .retain(|s| s.course_id != course_id);
        Ok(())
    }

    async fn delete(
        &self,
        session_id: &str,
    ) -> Result<(), AppError> {
        self.sessions
            .lock()
            .unwrap()
            .retain(|s| s.id != session_id);
        Ok(())
    }
}
