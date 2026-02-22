/*
 * Supabase QCM Repository
 *
 * CRUD implementation for QCM (multiple choice question) sets using Supabase.
 * Uses shared SupabaseHttpClient for connection pooling and retry logic.
 *
 * Tables:
 * - qcm_sets: Set metadata (name, level, subjects)
 * - qcm_questions: Individual questions with answers
 */

use super::types::{
    level_from_db, level_to_db, InsertQcmQuestionRow, QcmQuestionRow,
    QcmSetRow, UpdateQcmSetRow,
};
use crate::infra::database::QcmRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{QcmQuestion, QcmSet};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_SETS: &str = "qcm_sets";
const TABLE_QUESTIONS: &str = "qcm_questions";

/* ============================================================================
 * SUPABASE QCM REPOSITORY
 * ============================================================================ */

/*
 * Repository for QCM set persistence operations.
 *
 * Uses shared HTTP client for all Supabase operations.
 * Implements full CRUD: Create, Read, Update, Delete.
 */
#[derive(Clone, Debug)]
pub struct SupabaseQcmRepository {
    client: Arc<SupabaseHttpClient>,
}

/* ============================================================================
 * CONSTRUCTOR
 * ============================================================================ */

impl SupabaseQcmRepository {
    /*
     * Create repository with shared HTTP client.
     *
     * The client should be created once at app startup and shared
     * across all repository instances via Arc.
     */
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseQcmRepository initialized with shared client");
        Self { client }
    }

    /* ========================================================================
     * HELPER: Error Mapping
     * ======================================================================== */

    /*
     * Convert SupabaseError to IntelloError.
     *
     * Maps HTTP errors to appropriate domain errors.
     */
    fn map_error(err: SupabaseError) -> IntelloError {
        IntelloError::storage(err.to_string())
    }

    /* ========================================================================
     * HELPER: Questions CRUD (Internal)
     * ======================================================================== */

    /*
     * Insert questions for a set.
     *
     * Called internally after set creation/update.
     */
    async fn insert_questions(
        &self,
        set_id: &str,
        questions: &[QcmQuestion],
    ) -> Result<(), IntelloError> {
        if questions.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertQcmQuestionRow> = questions
            .iter()
            .map(|q| InsertQcmQuestionRow {
                id: q.id.to_string(),
                set_id: set_id.to_string(),
                question: q.question.clone(),
                wrong_answers: q.wrong_answers.clone(),
                right_answer: q.right_answer.clone(),
                explanation: q.explanation.clone(),
            })
            .collect();

        let url = self.client.rest_url(TABLE_QUESTIONS);
        debug!(url = %url, count = rows.len(), "Inserting QCM questions");

        let _: Vec<QcmQuestionRow> = self
            .client
            .post(&url, &rows)
            .await
            .map_err(Self::map_error)?;

        Ok(())
    }

    /*
     * Get questions for a set.
     *
     * Called internally when retrieving full set data.
     */
    async fn get_questions(
        &self,
        set_id: &str,
    ) -> Result<Vec<QcmQuestion>, IntelloError> {
        let url = self.client.rest_url_with_query(
            TABLE_QUESTIONS,
            &format!("set_id=eq.{}", set_id),
        );
        debug!(url = %url, "Getting QCM questions");

        let rows: Vec<QcmQuestionRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let questions = rows
            .into_iter()
            .map(|r| QcmQuestion {
                id: r.id.into(),
                question: r.question,
                wrong_answers: r.wrong_answers,
                right_answer: r.right_answer,
                explanation: r.explanation,
            })
            .collect();

        Ok(questions)
    }

    /*
     * Delete questions for a set.
     *
     * Called internally before set deletion or during update.
     */
    async fn delete_questions(&self, set_id: &str) -> Result<(), IntelloError> {
        let url = self.client.rest_url_with_query(
            TABLE_QUESTIONS,
            &format!("set_id=eq.{}", set_id),
        );
        debug!(url = %url, "Deleting QCM questions");

        self.client.delete(&url).await.map_err(Self::map_error)?;
        Ok(())
    }

    /* ========================================================================
     * HELPER: Row to Domain Conversion
     * ======================================================================== */

    /*
     * Convert database row to domain entity.
     *
     * Fetches associated questions and builds complete QcmSet.
     */
    async fn row_to_domain(
        &self,
        row: QcmSetRow,
    ) -> Result<QcmSet, IntelloError> {
        let questions = self.get_questions(&row.id).await?;

        Ok(QcmSet {
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

/* ============================================================================
 * CRUD IMPLEMENTATION
 * ============================================================================ */

#[async_trait]
impl QcmRepository for SupabaseQcmRepository {
    /* ========================================================================
     * CREATE: Insert new QCM set
     * ======================================================================== */

    /*
     * Insert a new QCM set with its questions atomically.
     *
     * Uses PostgreSQL RPC function for atomic transaction.
     * All questions are created with the set in a single database transaction.
     */
    #[instrument(skip(self, qcm_set), fields(set_id = %qcm_set.id))]
    async fn insert(&self, qcm_set: &QcmSet) -> Result<QcmSet, IntelloError> {
        let payload = serde_json::json!({
            "p_set": {
                "id": qcm_set.id,
                "user_id": qcm_set.user_id,
                "name": qcm_set.name,
                "description": qcm_set.description,
                "level": level_to_db(&qcm_set.level),
                "language": qcm_set.language,
                "subjects": qcm_set.subjects
            },
            "p_questions": qcm_set.questions.iter().map(|q| {
                serde_json::json!({
                    "id": q.id,
                    "question": q.question,
                    "wrong_answers": q.wrong_answers,
                    "right_answer": q.right_answer,
                    "explanation": q.explanation
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_qcm_set_atomic");
        debug!(url = %url, "Inserting QCM set atomically");

        let _: serde_json::Value = self
            .client
            .post(&url, &payload)
            .await
            .map_err(Self::map_error)?;

        info!(set_id = %qcm_set.id, "QCM set inserted atomically");
        Ok(qcm_set.clone())
    }

    /* ========================================================================
     * READ: Find by ID
     * ======================================================================== */

    /*
     * Find a QCM set by ID and user ID.
     *
     * Returns None if not found.
     * Includes all associated questions.
     */
    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<QcmSet>, IntelloError> {
        let query = format!("id=eq.{}&user_id=eq.{}", set_id, user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Finding QCM set by ID");

        let rows: Vec<QcmSetRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => {
                info!(set_id = %set_id, "QCM set not found");
                return Ok(None);
            }
        };

        let qcm_set = self.row_to_domain(row).await?;
        info!(set_id = %set_id, "QCM set found");
        Ok(Some(qcm_set))
    }

    /* ========================================================================
     * READ: Find by user
     * ======================================================================== */

    /*
     * Find all QCM sets belonging to a user.
     *
     * Returns empty vector if no sets found.
     * Includes all associated questions for each set.
     */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<QcmSet>, IntelloError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Finding QCM sets by user");

        let rows: Vec<QcmSetRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let qcm_set = self.row_to_domain(row).await?;
            sets.push(qcm_set);
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user QCM sets");
        Ok(sets)
    }

    /* ========================================================================
     * UPDATE: Update existing QCM set
     * ======================================================================== */

    /*
     * Update an existing QCM set.
     *
     * Replaces all questions (deletes old, inserts new).
     * Returns false if set not found.
     */
    #[instrument(skip(self, qcm_set), fields(set_id = %qcm_set.id))]
    async fn update(&self, qcm_set: &QcmSet) -> Result<bool, IntelloError> {
        /* Check existence first */
        let existing = self.find_by_id(&qcm_set.id, &qcm_set.user_id).await?;
        if existing.is_none() {
            info!(set_id = %qcm_set.id, "QCM set not found for update");
            return Ok(false);
        }

        let row = UpdateQcmSetRow {
            name: qcm_set.name.clone(),
            description: qcm_set.description.clone(),
            level: level_to_db(&qcm_set.level),
            language: qcm_set.language.clone(),
            subjects: qcm_set.subjects.clone(),
        };

        let query =
            format!("id=eq.{}&user_id=eq.{}", qcm_set.id, qcm_set.user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Updating QCM set");

        let _: Vec<QcmSetRow> = self
            .client
            .patch(&url, &row)
            .await
            .map_err(Self::map_error)?;

        /* Replace questions: delete old, insert new */
        self.delete_questions(&qcm_set.id).await?;
        self.insert_questions(&qcm_set.id, &qcm_set.questions)
            .await?;

        info!(set_id = %qcm_set.id, "QCM set updated");
        Ok(true)
    }

    /* ========================================================================
     * DELETE: Delete QCM set
     * ======================================================================== */

    /*
     * Delete a QCM set and its questions.
     *
     * Returns false if set not found.
     */
    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<bool, IntelloError> {
        /* Check existence first */
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "QCM set not found for deletion");
            return Ok(false);
        }

        /* Delete questions first */
        self.delete_questions(set_id).await?;

        /* Delete the set */
        let query = format!("id=eq.{}&user_id=eq.{}", set_id, user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Deleting QCM set");

        self.client.delete(&url).await.map_err(Self::map_error)?;

        info!(set_id = %set_id, "QCM set deleted");
        Ok(true)
    }
}

/* ============================================================================
 * LEGACY CONSTRUCTOR (COMMENTED - Preserved for reference)
 * ============================================================================ */

/*
 * Legacy constructor that creates its own HTTP client.
 *
 * DEPRECATED: Use new(Arc<SupabaseHttpClient>) instead.
 * Kept for reference during migration.
 */
// impl SupabaseQcmRepository {
//     pub fn new_legacy(config: &crate::configs::Config) -> Self {
//         let client = Arc::new(SupabaseHttpClient::new(config));
//         Self { client }
//     }
// }
