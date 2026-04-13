/*
 * Supabase Flashcard Repository
 *
 * CRUD implementation for flashcard sets using Supabase.
 * Uses shared SupabaseHttpClient for connection pool

ing and retry logic.
 *
 * Tables:
 * - flashcard_sets: Set metadata (name, level, subjects)
 * - flashcards: Individual cards with front/back
 */

use super::types::{level_from_db, level_to_db, FlashcardRow, FlashcardSetRow};
use crate::infra::database::GameSetRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::error_domain::StudyError;
use crate::services::{Flashcard, FlashcardSet};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_SETS: &str = "flashcard_sets";
const TABLE_CARDS: &str = "flashcards";

/* ============================================================================
 * SUPABASE FLASHCARD REPOSITORY
 * ============================================================================ */

/*
 * Repository for flashcard set persistence operations.
 *
 * Uses shared HTTP client for all Supabase operations.
 * Implements full CRUD: Create, Read, Delete.
 */
#[derive(Clone, Debug)]
pub struct SupabaseFlashcardRepository {
    client: Arc<SupabaseHttpClient>,
}

/* ============================================================================
 * CONSTRUCTOR
 * ============================================================================ */

impl SupabaseFlashcardRepository {
    /*
     * Create repository with shared HTTP client.
     */
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseFlashcardRepository initialized with shared client");
        Self { client }
    }

    /* ========================================================================
     * HELPER: Error Mapping
     * ======================================================================== */

    /*
     * Convert SupabaseError to StudyError.
     */
    fn map_error(err: SupabaseError) -> StudyError {
        StudyError::storage(err.to_string())
    }

    /*
     * Get flashcards for a set.
     */
    async fn get_cards(
        &self,
        set_id: &str,
    ) -> Result<Vec<Flashcard>, StudyError> {
        let url = self
            .client
            .rest_url_with_query(TABLE_CARDS, &format!("set_id=eq.{}", set_id));
        debug!(url = %url, "Getting flashcards");

        let rows: Vec<FlashcardRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let cards = rows
            .into_iter()
            .map(|r| Flashcard {
                id: r.id.into(),
                front: r.front,
                back: r.back,
            })
            .collect();

        Ok(cards)
    }

    /* ========================================================================
     * HELPER: Row to Domain Conversion
     * ======================================================================== */

    /*
     * Convert database row to domain entity.
     */
    async fn row_to_domain(
        &self,
        row: FlashcardSetRow,
    ) -> Result<FlashcardSet, StudyError> {
        let cards = self.get_cards(&row.id).await?;

        Ok(FlashcardSet {
            id: row.id.into(),
            user_id: row.user_id.into(),
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            cards,
        })
    }
}

/* ============================================================================
 * CRUD IMPLEMENTATION
 * ============================================================================ */

#[async_trait]
impl GameSetRepository<FlashcardSet> for SupabaseFlashcardRepository {
    /* ========================================================================
     * CREATE: Insert new flashcard set
     * ======================================================================== */

    /*
     * Insert a new flashcard set with its cards atomically.
     */
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(
        &self,
        set: &FlashcardSet,
    ) -> Result<FlashcardSet, StudyError> {
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
            "p_cards": set.cards.iter().map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "front": c.front,
                    "back": c.back
                })
            }).collect::<Vec<_>>()
        });

        let url = self.client.rpc_url("create_flashcard_set_atomic");
        let _: serde_json::Value = self
            .client
            .post(&url, &payload)
            .await
            .map_err(Self::map_error)?;
        Ok(set.clone())
    }

    /* ========================================================================
     * READ: Find by user
     * ======================================================================== */

    /*
     * Find all flashcard sets belonging to a user.
     */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<FlashcardSet>, StudyError> {
        let query = format!("user_id=eq.{}", user_id);
        let url = self.client.rest_url_with_query(TABLE_SETS, &query);
        debug!(url = %url, "Finding flashcard sets by user");

        let rows: Vec<FlashcardSetRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let set = self.row_to_domain(row).await?;
            sets.push(set);
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user flashcard sets");
        Ok(sets)
    }
}
