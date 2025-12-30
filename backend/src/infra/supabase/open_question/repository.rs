/*
 * Supabase Open Question Repository
 *
 * CRUD implementation for open question sets using Supabase.
 * Uses shared SupabaseHttpClient for connection pooling and retry logic.
 *
 * Tables:
 * - open_question_sets: Set metadata
 * - open_questions: Individual questions
 */

use super::types::{level_from_db, level_to_db, OpenQuestionRow, OpenQuestionSetRow};
use crate::services::intello::{OpenQuestion, OpenQuestionSet};
use crate::services::intello::error_domain::IntelloError;
use crate::infra::database::OpenQuestionRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_SETS: &str = "open_question_sets";
const TABLE_QUESTIONS: &str = "open_questions";

/* ============================================================================
 * SUPABASE OPEN QUESTION REPOSITORY
 * ============================================================================ */

#[derive(Clone, Debug)]
pub struct SupabaseOpenQuestionRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseOpenQuestionRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseOpenQuestionRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> IntelloError {
        IntelloError::storage(err.to_string())
    }

    async fn get_questions(&self, set_id: &str) -> Result<Vec<OpenQuestion>, IntelloError> {
        let url = self
            .client
            .rest_url_with_query(TABLE_QUESTIONS, &format!("set_id=eq.{}", set_id));
        debug!(url = %url, "Getting open questions");

        let rows: Vec<OpenQuestionRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        let questions = rows
            .into_iter()
            .map(|r| OpenQuestion {
                id: r.id.into(),
                question: r.question,
                user_answer: r.user_answer,
                expected_answer: r.expected_answer,
                hint: r.hint,
            })
            .collect();

        Ok(questions)
    }

    async fn row_to_domain(
        &self,
        row: OpenQuestionSetRow,
    ) -> Result<OpenQuestionSet, IntelloError> {
        let questions = self.get_questions(&row.id).await?;

        Ok(OpenQuestionSet {
            id: row.id.into(),
            user_id: row.user_id.into(),
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            questions,
        })
    }
}

#[async_trait]
impl OpenQuestionRepository for SupabaseOpenQuestionRepository {
    /* CREATE */
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &OpenQuestionSet) -> Result<OpenQuestionSet, IntelloError> {
        let payload = serde_json::json!({
            "p_set": {
                "id": set.id,
                "user_id": set.user_id,
                "name": set.name,
                "description": set.description,
                "level": level_to_db(&set.level),
                "language": set.language,
                "subjects": set.subjects
            },
            "p_questions": set.questions.iter().map(|q| {
                serde_json::json!({
                    "id": q.id,
                    "question": q.question,
                    "user_answer": q.user_answer,
                    "expected_answer": q.expected_answer,
                    "hint": q.hint
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_open_question_set_atomic");
        let _: serde_json::Value = self
            .client
            .post(&url, &payload)
            .await
            .map_err(Self::map_error)?;
        Ok(set.clone())
    }

    /* READ: By ID */
    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<OpenQuestionSet>, IntelloError> {
        let query = format!("id=eq.{}&user_id=eq.{}", set_id, user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Finding open question set by ID");

        let rows: Vec<OpenQuestionSetRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => {
                info!(set_id = %set_id, "Open question set not found");
                return Ok(None);
            }
        };

        let set = self.row_to_domain(row).await?;
        info!(set_id = %set_id, "Open question set found");
        Ok(Some(set))
    }

    /* READ: By user */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Finding open question sets by user");

        let rows: Vec<OpenQuestionSetRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let set = self.row_to_domain(row).await?;
            sets.push(set);
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user open question sets");
        Ok(sets)
    }
}
