/*
 * Supabase Fill Blank Repository
 *
 * CRUD for fill-in-the-blank question sets.
 * Uses shared SupabaseHttpClient.
 *
 * Tables: intello_fill_blank_sets, intello_fill_blanks, intello_fill_blank_options
 */

use super::types::{
    level_from_db, level_to_db, FillBlankOptionRow, FillBlankQuestionRow, FillBlankSetRow,
};
use crate::infra::database::GameSetRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{FillBlankOption, FillBlankQuestion, FillBlankSet};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, instrument};

const TABLE_SETS: &str = "intello_fill_blank_sets";
const TABLE_QUESTIONS: &str = "intello_fill_blanks";
const TABLE_OPTIONS: &str = "intello_fill_blank_options";

#[derive(Clone, Debug)]
pub struct SupabaseFillBlankRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseFillBlankRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseFillBlankRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> IntelloError {
        IntelloError::storage(err.to_string())
    }

    async fn get_questions(&self, set_id: &str) -> Result<Vec<FillBlankQuestion>, IntelloError> {
        let url = self
            .client
            .rest_url_with_query(TABLE_QUESTIONS, &format!("set_id=eq.{}", set_id));
        let q_rows: Vec<FillBlankQuestionRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let mut questions = Vec::new();
        for q_row in q_rows {
            let url = self
                .client
                .rest_url_with_query(TABLE_OPTIONS, &format!("question_id=eq.{}", q_row.id));
            let w_rows: Vec<FillBlankOptionRow> =
                self.client.get(&url).await.map_err(Self::map_error)?;

            let options = w_rows
                .into_iter()
                .map(|o| FillBlankOption {
                    id: o.id.into(),
                    text: o.text,
                    is_correct: o.is_correct,
                })
                .collect();

            questions.push(FillBlankQuestion {
                id: q_row.id.into(),
                phrase: q_row.phrase,
                explanation: q_row.explanation,
                options,
            });
        }
        Ok(questions)
    }

    async fn row_to_domain(&self, row: FillBlankSetRow) -> Result<FillBlankSet, IntelloError> {
        let questions = self.get_questions(&row.id).await?;
        Ok(FillBlankSet {
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
impl GameSetRepository<FillBlankSet> for SupabaseFillBlankRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &FillBlankSet) -> Result<FillBlankSet, IntelloError> {
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
                    "phrase": q.phrase,
                    "explanation": q.explanation,
                    "options": q.options.iter().map(|o| {
                        serde_json::json!({
                            "id": o.id,
                            "text": o.text,
                            "is_correct": o.is_correct
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_fill_blank_set_atomic");
        let _: serde_json::Value = self
            .client
            .post(&url, &payload)
            .await
            .map_err(Self::map_error)?;
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FillBlankSet>, IntelloError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        let rows: Vec<FillBlankSetRow> = self.client.get(&url).await.map_err(Self::map_error)?;
        let mut sets = Vec::new();
        for row in rows {
            sets.push(self.row_to_domain(row).await?);
        }
        Ok(sets)
    }
}
