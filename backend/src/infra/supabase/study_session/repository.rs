/*
 * Supabase Study Session Repository
 *
 * Repository for managing study sessions within courses.
 */

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::services::intello::study_session_domain::StudySession;
use crate::shared::AppError;
use crate::infra::database::StudySessionRepository;
use crate::infra::supabase::shared::SupabaseHttpClient;

const TABLE_SESSIONS: &str = "intello_study_sessions";

/* ============================================================================
 * ROW TYPES
 * ============================================================================ */

#[derive(Debug, Deserialize)]
struct SessionRow {
    id: String,
    course_id: String,
    topic: String,
    instructions: String,
    keywords: Vec<String>,
    language: String,
    status: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct InsertSessionRow {
    id: String,
    course_id: String,
    topic: String,
    instructions: String,
    keywords: Vec<String>,
    language: String,
    status: String,
}

/* ============================================================================
 * REPOSITORY
 * ============================================================================ */

#[derive(Clone, Debug)]
pub struct SupabaseStudySessionRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseStudySessionRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseStudySessionRepository initialized");
        Self { client }
    }

    fn row_to_domain(row: SessionRow) -> StudySession {
        StudySession {
            id: row.id,
            course_id: row.course_id,
            topic: row.topic,
            instructions: row.instructions,
            keywords: row.keywords,
            language: row.language,
            status: row.status,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl StudySessionRepository for SupabaseStudySessionRepository {
    async fn create(&self, session: &StudySession) -> Result<StudySession, AppError> {
        let row = InsertSessionRow {
            id: session.id.clone(),
            course_id: session.course_id.clone(),
            topic: session.topic.clone(),
            instructions: session.instructions.clone(),
            keywords: session.keywords.clone(),
            language: session.language.clone(),
            status: session.status.clone(),
        };

        let url = self.client.rest_url(TABLE_SESSIONS);
        let rows: Vec<SessionRow> = self
            .client
            .post(&url, &row)
            .await
            .map_err(|e| AppError::Internal(crate::http_api::utils::InternalError::new(e.to_string())))?;

        let created = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal(crate::http_api::utils::InternalError::new("No session returned".to_string())))?;

        info!(session_id = %created.id, "Study session created");
        Ok(Self::row_to_domain(created))
    }

    async fn list_by_course(&self, course_id: &str) -> Result<Vec<StudySession>, AppError> {
        let query = format!("course_id=eq.{}&order=created_at.desc", course_id);
        let url = self.client.rest_url_with_query(TABLE_SESSIONS, &query);

        let rows: Vec<SessionRow> = self
            .client
            .get(&url)
            .await
            .map_err(|e| AppError::Internal(crate::http_api::utils::InternalError::new(e.to_string())))?;

        Ok(rows.into_iter().map(Self::row_to_domain).collect())
    }
}
