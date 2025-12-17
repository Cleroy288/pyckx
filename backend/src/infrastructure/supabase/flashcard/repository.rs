//! Supabase Flashcard Repository - PostgreSQL implementation via REST API
//!
//! Implements FlashcardRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, FlashcardRow, FlashcardSetRow, InsertFlashcardRow,
    InsertFlashcardSetRow,
};
use crate::config::Config;
use crate::domain::intello::{Flashcard, FlashcardSet};
use crate::error::IntelloError;
use crate::infrastructure::repository::FlashcardRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE FLASHCARD REPOSITORY
// =============================================================================

/// Supabase implementation of FlashcardRepository
#[derive(Clone, Debug)]
pub struct SupabaseFlashcardRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseFlashcardRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseFlashcardRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/flashcard_sets", self.url)
    }

    fn cards_endpoint(&self) -> String {
        format!("{}/rest/v1/flashcards", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Insert cards for a set
    async fn insert_cards(&self, set_id: &str, cards: &[Flashcard]) -> Result<(), IntelloError> {
        if cards.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertFlashcardRow> = cards
            .iter()
            .map(|c| InsertFlashcardRow {
                id: c.id.clone(),
                set_id: set_id.to_string(),
                front: c.front.clone(),
                back: c.back.clone(),
            })
            .collect();

        let endpoint = self.cards_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting flashcards");

        let mut request = self.client.post(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .json(&rows)
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to insert flashcards");
            return Err(IntelloError::storage(format!("Insert cards failed: {}", status)));
        }

        Ok(())
    }

    /// Get cards for a set
    async fn get_cards(&self, set_id: &str) -> Result<Vec<Flashcard>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.cards_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting flashcards");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to get flashcards");
            return Err(IntelloError::storage(format!("Get cards failed: {}", status)));
        }

        let rows: Vec<FlashcardRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let cards: Vec<Flashcard> = rows
            .into_iter()
            .map(|r| Flashcard {
                id: r.id,
                front: r.front,
                back: r.back,
            })
            .collect();

        Ok(cards)
    }

    /// Delete cards for a set
    async fn delete_cards(&self, set_id: &str) -> Result<(), IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.cards_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Deleting flashcards");

        let mut request = self.client.delete(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to delete flashcards");
            return Err(IntelloError::storage(format!("Delete cards failed: {}", status)));
        }

        Ok(())
    }
}

#[async_trait]
impl FlashcardRepository for SupabaseFlashcardRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &FlashcardSet) -> Result<FlashcardSet, IntelloError> {
        // Insert the set
        let row = InsertFlashcardSetRow {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: level_to_db(&set.level),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting flashcard set");

        let mut request = self.client.post(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .json(&row)
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to insert flashcard set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert cards
        self.insert_cards(&set.id, &set.cards).await?;

        info!(set_id = %set.id, "Flashcard set inserted");
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<FlashcardSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding flashcard set by ID");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to find flashcard set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<FlashcardSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };

        // Get cards
        let cards = self.get_cards(set_id).await?;

        let set = FlashcardSet {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            cards,
        };

        Ok(Some(set))
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FlashcardSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding flashcard sets by user");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to find user flashcard sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<FlashcardSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let cards = self.get_cards(&row.id).await?;
            sets.push(FlashcardSet {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                description: row.description,
                level: level_from_db(&row.level),
                language: row.language,
                subjects: row.subjects,
                cards,
            });
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user flashcard sets");
        Ok(sets)
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "Flashcard set not found for deletion");
            return Ok(false);
        }

        // Delete cards first
        self.delete_cards(set_id).await?;

        // Delete the set
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Deleting flashcard set");

        let mut request = self.client.delete(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| IntelloError::storage(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to delete flashcard set");
            return Err(IntelloError::storage(format!("Delete set failed: {}", status)));
        }

        info!(set_id = %set_id, "Flashcard set deleted");
        Ok(true)
    }
}
