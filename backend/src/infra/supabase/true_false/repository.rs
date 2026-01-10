/*
 * Supabase True/False Repository
 *
 * CRUD implementation for true/false sets using Supabase.
 * Uses shared SupabaseHttpClient for connection pooling and retry logic.
 *
 * Tables:
 * - intello_true_false_sets: Set metadata
 * - intello_true_false_statements: Individual statements
 */

use super::types::{level_from_db, level_to_db, TrueOrFalseSetRow, TrueOrFalseStatementRow};
use crate::infra::database::GameSetRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{TrueOrFalseSet, TrueOrFalseStatement};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_SETS: &str = "intello_true_false_sets";
const TABLE_STATEMENTS: &str = "intello_true_false_statements";

/* ============================================================================
 * SUPABASE TRUE/FALSE REPOSITORY
 * ============================================================================ */

#[derive(Clone, Debug)]
pub struct SupabaseTrueOrFalseRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseTrueOrFalseRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseTrueOrFalseRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> IntelloError {
        IntelloError::storage(err.to_string())
    }

    async fn get_statements(
        &self,
        set_id: &str,
    ) -> Result<Vec<TrueOrFalseStatement>, IntelloError> {
        let url = self
            .client
            .rest_url_with_query(TABLE_STATEMENTS, &format!("set_id=eq.{}", set_id));
        debug!(url = %url, "Getting true/false statements");

        let rows: Vec<TrueOrFalseStatementRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let statements = rows
            .into_iter()
            .map(|r| TrueOrFalseStatement {
                id: r.id.into(),
                statement: r.statement,
                answer: r.answer,
                explanation: r.explanation,
            })
            .collect();

        Ok(statements)
    }

    async fn row_to_domain(&self, row: TrueOrFalseSetRow) -> Result<TrueOrFalseSet, IntelloError> {
        let statements = self.get_statements(&row.id).await?;

        Ok(TrueOrFalseSet {
            id: row.id.into(),
            user_id: row.user_id.into(),
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            statements,
        })
    }
}

#[async_trait]
impl GameSetRepository<TrueOrFalseSet> for SupabaseTrueOrFalseRepository {
    /* CREATE */
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &TrueOrFalseSet) -> Result<TrueOrFalseSet, IntelloError> {
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
            "p_statements": set.statements.iter().map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "statement": s.statement,
                    "answer": s.answer,
                    "explanation": s.explanation
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_true_false_set_atomic");
        let _: serde_json::Value = self
            .client
            .post(&url, &payload)
            .await
            .map_err(Self::map_error)?;
        Ok(set.clone())
    }

    /* READ: By user */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Finding true/false sets by user");

        let rows: Vec<TrueOrFalseSetRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let set = self.row_to_domain(row).await?;
            sets.push(set);
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user true/false sets");
        Ok(sets)
    }
}
