/*
 * Supabase Order Phrase Repository
 *
 * CRUD for phrase ordering question sets.
 * Uses shared SupabaseHttpClient.
 *
 * Tables: study_order_phrase_sets, study_order_phrase_questions, study_order_phrase_words
 */

use super::types::{
    level_from_db, level_to_db, OrderPhraseQuestionRow, OrderPhraseSetRow,
    OrderPhraseWordRow,
};
use crate::infra::database::GameSetRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::error_domain::StudyError;
use crate::services::{
    OrderPhraseQuestion, OrderPhraseSet, OrderPhraseWord,
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, instrument};

const TABLE_SETS: &str = "intello_order_phrase_sets";
const TABLE_QUESTIONS: &str = "intello_order_phrase_questions";
const TABLE_WORDS: &str = "intello_order_phrase_words";

#[derive(Clone, Debug)]
pub struct SupabaseOrderPhraseRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseOrderPhraseRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseOrderPhraseRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> StudyError {
        StudyError::storage(err.to_string())
    }

    async fn get_questions(
        &self,
        set_id: &str,
    ) -> Result<Vec<OrderPhraseQuestion>, StudyError> {
        let url = self.client.rest_url_with_query(
            TABLE_QUESTIONS,
            &format!("set_id=eq.{}", set_id),
        );
        let q_rows: Vec<OrderPhraseQuestionRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let mut questions = Vec::new();
        for q_row in q_rows {
            let url = self.client.rest_url_with_query(
                TABLE_WORDS,
                &format!("question_id=eq.{}", q_row.id),
            );
            let w_rows: Vec<OrderPhraseWordRow> =
                self.client.get(&url).await.map_err(Self::map_error)?;

            let words = w_rows
                .into_iter()
                .map(|w| OrderPhraseWord {
                    id: w.id.into(),
                    word: w.word,
                    position: w.position as u8,
                })
                .collect();

            questions.push(OrderPhraseQuestion {
                id: q_row.id.into(),
                original_phrase: q_row.original_phrase,
                hint: q_row.hint,
                words,
            });
        }
        Ok(questions)
    }

    async fn row_to_domain(
        &self,
        row: OrderPhraseSetRow,
    ) -> Result<OrderPhraseSet, StudyError> {
        let questions = self.get_questions(&row.id).await?;
        Ok(OrderPhraseSet {
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
impl GameSetRepository<OrderPhraseSet> for SupabaseOrderPhraseRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(
        &self,
        set: &OrderPhraseSet,
    ) -> Result<OrderPhraseSet, StudyError> {
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
                    "original_phrase": q.original_phrase,
                    "hint": q.hint,
                    "words": q.words.iter().map(|w| {
                        serde_json::json!({
                            "id": w.id,
                            "word": w.word,
                            "position": w.position
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_order_phrase_set_atomic");
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
    ) -> Result<Vec<OrderPhraseSet>, StudyError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        let rows: Vec<OrderPhraseSetRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        let mut sets = Vec::new();
        for row in rows {
            sets.push(self.row_to_domain(row).await?);
        }
        Ok(sets)
    }
}
