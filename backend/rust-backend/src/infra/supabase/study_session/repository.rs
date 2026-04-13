/*
 * Supabase Study Session Repository
 *
 * Repository for managing study sessions within courses.
 */

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::infra::database::StudySessionRepository;
use crate::infra::supabase::shared::SupabaseHttpClient;
use crate::services::study_session::study_session_domain::StudySession;
use crate::shared::AppError;

const TABLE_SESSIONS: &str = "intello_study_sessions";

/* ============================================================================
 * ROW TYPES
 * ============================================================================ */

#[derive(Debug, Deserialize)]
struct SessionRow {
    pub id: String,
    pub course_id: String,
    pub topic: String,
    pub instructions: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub status: String,
    pub generated_content: Option<serde_json::Value>,
    pub extracted_knowledge: Option<serde_json::Value>,
    pub educational_content: Option<serde_json::Value>,
    pub expanded_knowledge: Option<String>,
    pub created_at: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    generated_content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extracted_knowledge: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    educational_content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expanded_knowledge: Option<String>,
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
            generated_content: row.generated_content,
            extracted_knowledge: row.extracted_knowledge,
            educational_content: row.educational_content,
            expanded_knowledge: row.expanded_knowledge,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl StudySessionRepository for SupabaseStudySessionRepository {
    async fn create(
        &self,
        session: &StudySession,
    ) -> Result<StudySession, AppError> {
        let row = InsertSessionRow {
            id: session.id.clone(),
            course_id: session.course_id.clone(),
            topic: session.topic.clone(),
            instructions: session.instructions.clone(),
            keywords: session.keywords.clone(),
            language: session.language.clone(),
            status: session.status.clone(),
            generated_content: session.generated_content.clone(),
            extracted_knowledge: session.extracted_knowledge.clone(),
            educational_content: session.educational_content.clone(),
            expanded_knowledge: session.expanded_knowledge.clone(),
        };

        let url = self.client.rest_url(TABLE_SESSIONS);
        let rows: Vec<SessionRow> =
            self.client.post(&url, &row).await.map_err(|err| {
                AppError::Internal(crate::http_api::utils::InternalError::new(
                    err.to_string(),
                ))
            })?;

        let created = rows.into_iter().next().ok_or_else(|| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                "No session returned".to_string(),
            ))
        })?;

        info!(session_id = %created.id, "Study session created");
        Ok(Self::row_to_domain(created))
    }

    async fn list_by_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<StudySession>, AppError> {
        let query = format!("course_id=eq.{}&order=created_at.desc", course_id);
        let url = self.client.rest_url_with_query(TABLE_SESSIONS, &query);

        let rows: Vec<SessionRow> =
            self.client.get(&url).await.map_err(|err| {
                AppError::Internal(crate::http_api::utils::InternalError::new(
                    err.to_string(),
                ))
            })?;

        Ok(rows.into_iter().map(Self::row_to_domain).collect())
    }

    async fn get(&self, session_id: &str) -> Result<StudySession, AppError> {
        let query = format!("id=eq.{}", session_id);
        let url = self.client.rest_url_with_query(TABLE_SESSIONS, &query);

        let rows: Vec<SessionRow> =
            self.client.get(&url).await.map_err(|err| {
                AppError::Internal(crate::http_api::utils::InternalError::new(
                    err.to_string(),
                ))
            })?;

        let session = rows.into_iter().next().ok_or_else(|| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                "Session not found".to_string(),
            ))
        })?;

        Ok(Self::row_to_domain(session))
    }

    async fn save_session_content(
        &self,
        session_id: &str,
        content: &serde_json::Value,
    ) -> Result<(), AppError> {
        let url = self.client.rest_url_with_query(
            TABLE_SESSIONS,
            &format!("id=eq.{}", session_id),
        );

        use crate::http_api::data_transfer_object::course::CourseGenerationResult;

        let update = if let Ok(full_result) =
            serde_json::from_value::<CourseGenerationResult>(
                content.clone(),
            )
        {
            serde_json::json!({
                "generated_content": full_result.course,
                "extracted_knowledge": full_result.extracted_knowledge,
                "educational_content": full_result.educational_content,
                "expanded_knowledge": full_result.expanded_knowledge,
                "status": "completed"
            })
        } else {
            serde_json::json!({
                "generated_content": content,
                "status": "completed"
            })
        };

        self.client
            .patch::<serde_json::Value, _>(&url, &update)
            .await
            .map_err(|err| {
                AppError::Internal(crate::http_api::utils::InternalError::new(
                    err.to_string(),
                ))
            })?;

        info!(session_id = %session_id, "Saved generated content and artifacts to session");
        Ok(())
    }

    async fn delete_by_course(&self, course_id: &str) -> Result<(), AppError> {
        let query = format!("course_id=eq.{}", course_id);
        let url = self.client.rest_url_with_query(TABLE_SESSIONS, &query);

        self.client.delete(&url).await.map_err(|err| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                err.to_string(),
            ))
        })?;

        info!(course_id = %course_id, "Deleted all study sessions for course");
        Ok(())
    }

    async fn delete(&self, session_id: &str) -> Result<(), AppError> {
        let query = format!("id=eq.{}", session_id);
        let url = self.client.rest_url_with_query(TABLE_SESSIONS, &query);

        self.client.delete(&url).await.map_err(|err| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                err.to_string(),
            ))
        })?;

        info!(session_id = %session_id, "Deleted study session");
        Ok(())
    }
}
