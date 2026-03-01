/*
 * Supabase Keywords Repository
 *
 * CRUD for keyword question sets.
 * Uses shared SupabaseHttpClient.
 *
 * Tables: intello_keyword_sets, intello_keyword_questions, intello_keywords
 */

use super::types::{
    level_from_db, level_to_db, KeywordQuestionRow, KeywordRow, KeywordSetRow,
};
use crate::infra::database::GameSetRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{Keyword, KeywordQuestion, KeywordSet};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, instrument};

const TABLE_SETS: &str = "intello_keyword_sets";
const TABLE_QUESTIONS: &str = "intello_keyword_questions";
const TABLE_KEYWORDS: &str = "intello_keywords";

#[derive(Clone, Debug)]
pub struct SupabaseKeywordsRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseKeywordsRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseKeywordsRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> IntelloError {
        IntelloError::storage(err.to_string())
    }

    async fn get_questions(
        &self,
        set_id: &str,
    ) -> Result<Vec<KeywordQuestion>, IntelloError> {
        let url = self.client.rest_url_with_query(
            TABLE_QUESTIONS,
            &format!("set_id=eq.{}", set_id),
        );
        let q_rows: Vec<KeywordQuestionRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let mut questions = Vec::new();
        for q_row in q_rows {
            let url = self.client.rest_url_with_query(
                TABLE_KEYWORDS,
                &format!("question_id=eq.{}", q_row.id),
            );
            let k_rows: Vec<KeywordRow> =
                self.client.get(&url).await.map_err(Self::map_error)?;

            let keywords = k_rows
                .into_iter()
                .map(|k| Keyword {
                    id: k.id.into(),
                    word: k.word,
                    is_correct: k.is_correct,
                })
                .collect();

            questions.push(KeywordQuestion {
                id: q_row.id.into(),
                statement: q_row.statement,
                explanation: q_row.explanation,
                keywords,
            });
        }
        Ok(questions)
    }

    async fn row_to_domain(
        &self,
        row: KeywordSetRow,
    ) -> Result<KeywordSet, IntelloError> {
        let questions = self.get_questions(&row.id).await?;
        Ok(KeywordSet {
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
impl GameSetRepository<KeywordSet> for SupabaseKeywordsRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(
        &self,
        set: &KeywordSet,
    ) -> Result<KeywordSet, IntelloError> {
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
                    "statement": q.statement,
                    "explanation": q.explanation,
                    "keywords": q.keywords.iter().map(|k| {
                        serde_json::json!({
                            "id": k.id,
                            "word": k.word,
                            "is_correct": k.is_correct
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_keywords_set_atomic");
        let _: serde_json::Value = self
            .client
            .post(&url, &payload)
            .await
            .map_err(Self::map_error)?;
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<KeywordSet>, IntelloError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        let rows: Vec<KeywordSetRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        let mut sets = Vec::new();
        for row in rows {
            sets.push(self.row_to_domain(row).await?);
        }
        Ok(sets)
    }
}
