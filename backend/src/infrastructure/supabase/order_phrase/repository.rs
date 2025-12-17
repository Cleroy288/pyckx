//! Supabase Order Phrase Repository - PostgreSQL implementation via REST API
//!
//! Implements OrderPhraseRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, InsertOrderPhraseQuestionRow, InsertOrderPhraseSetRow,
    InsertOrderPhraseWordRow, OrderPhraseQuestionRow, OrderPhraseSetRow, OrderPhraseWordRow,
};
use crate::config::Config;
use crate::domain::intello::{OrderPhraseQuestion, OrderPhraseSet, OrderPhraseWord};
use crate::error::IntelloError;
use crate::infrastructure::repository::OrderPhraseRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE ORDER PHRASE REPOSITORY
// =============================================================================

/// Supabase implementation of OrderPhraseRepository
#[derive(Clone, Debug)]
pub struct SupabaseOrderPhraseRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseOrderPhraseRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseOrderPhraseRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_order_phrase_sets", self.url)
    }

    fn questions_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_order_phrase_questions", self.url)
    }

    fn words_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_order_phrase_words", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Insert words for a question
    async fn insert_words(&self, question_id: &str, words: &[OrderPhraseWord]) -> Result<(), IntelloError> {
        if words.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertOrderPhraseWordRow> = words
            .iter()
            .map(|w| InsertOrderPhraseWordRow {
                id: w.id.clone(),
                question_id: question_id.to_string(),
                word: w.word.clone(),
                position: w.position,
            })
            .collect();

        let endpoint = self.words_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting order phrase words");

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
            error!(status = %status, body = %body, "Failed to insert order phrase words");
            return Err(IntelloError::storage(format!("Insert words failed: {}", status)));
        }

        Ok(())
    }

    /// Get words for a question
    async fn get_words(&self, question_id: &str) -> Result<Vec<OrderPhraseWord>, IntelloError> {
        let endpoint = format!("{}?question_id=eq.{}", self.words_endpoint(), question_id);
        debug!(endpoint = %endpoint, "Getting order phrase words");

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
            error!(status = %status, body = %body, "Failed to get order phrase words");
            return Err(IntelloError::storage(format!("Get words failed: {}", status)));
        }

        let rows: Vec<OrderPhraseWordRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let words: Vec<OrderPhraseWord> = rows
            .into_iter()
            .map(|r| OrderPhraseWord {
                id: r.id,
                word: r.word,
                position: r.position as u8,
            })
            .collect();

        Ok(words)
    }

    /// Insert questions for a set
    async fn insert_questions(&self, set_id: &str, questions: &[OrderPhraseQuestion]) -> Result<(), IntelloError> {
        if questions.is_empty() {
            return Ok(());
        }

        // Insert question rows
        let rows: Vec<InsertOrderPhraseQuestionRow> = questions
            .iter()
            .map(|q| InsertOrderPhraseQuestionRow {
                id: q.id.clone(),
                set_id: set_id.to_string(),
                original_phrase: q.original_phrase.clone(),
                hint: q.hint.clone(),
            })
            .collect();

        let endpoint = self.questions_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting order phrase questions");

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
            error!(status = %status, body = %body, "Failed to insert order phrase questions");
            return Err(IntelloError::storage(format!("Insert questions failed: {}", status)));
        }

        // Insert words for each question
        for question in questions {
            self.insert_words(&question.id, &question.words).await?;
        }

        Ok(())
    }

    /// Get questions for a set
    async fn get_questions(&self, set_id: &str) -> Result<Vec<OrderPhraseQuestion>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting order phrase questions");

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
            error!(status = %status, body = %body, "Failed to get order phrase questions");
            return Err(IntelloError::storage(format!("Get questions failed: {}", status)));
        }

        let rows: Vec<OrderPhraseQuestionRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut questions = Vec::with_capacity(rows.len());
        for row in rows {
            let words = self.get_words(&row.id).await?;
            questions.push(OrderPhraseQuestion {
                id: row.id,
                original_phrase: row.original_phrase,
                words,
                hint: row.hint,
            });
        }

        Ok(questions)
    }

    /// Delete words for a question
    async fn delete_words(&self, question_id: &str) -> Result<(), IntelloError> {
        let endpoint = format!("{}?question_id=eq.{}", self.words_endpoint(), question_id);
        debug!(endpoint = %endpoint, "Deleting order phrase words");

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
            error!(status = %status, body = %body, "Failed to delete order phrase words");
            return Err(IntelloError::storage(format!("Delete words failed: {}", status)));
        }

        Ok(())
    }

    /// Delete questions and their words for a set
    async fn delete_questions(&self, set_id: &str) -> Result<(), IntelloError> {
        // Get questions first to delete their words
        let questions = self.get_questions(set_id).await?;
        for question in questions {
            self.delete_words(&question.id).await?;
        }

        // Delete questions
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Deleting order phrase questions");

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
            error!(status = %status, body = %body, "Failed to delete order phrase questions");
            return Err(IntelloError::storage(format!("Delete questions failed: {}", status)));
        }

        Ok(())
    }
}

#[async_trait]
impl OrderPhraseRepository for SupabaseOrderPhraseRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &OrderPhraseSet) -> Result<OrderPhraseSet, IntelloError> {
        // Insert the set
        let row = InsertOrderPhraseSetRow {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: level_to_db(&set.level),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting order phrase set");

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
            error!(status = %status, body = %body, "Failed to insert order phrase set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert questions
        self.insert_questions(&set.id, &set.questions).await?;

        info!(set_id = %set.id, "Order phrase set inserted");
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<OrderPhraseSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding order phrase set by ID");

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
            error!(status = %status, body = %body, "Failed to find order phrase set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<OrderPhraseSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };

        // Get questions
        let questions = self.get_questions(set_id).await?;

        let set = OrderPhraseSet {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            questions,
        };

        Ok(Some(set))
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding order phrase sets by user");

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
            error!(status = %status, body = %body, "Failed to find user order phrase sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<OrderPhraseSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let questions = self.get_questions(&row.id).await?;
            sets.push(OrderPhraseSet {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                description: row.description,
                level: level_from_db(&row.level),
                language: row.language,
                subjects: row.subjects,
                questions,
            });
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user order phrase sets");
        Ok(sets)
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "Order phrase set not found for deletion");
            return Ok(false);
        }

        // Delete questions and words first
        self.delete_questions(set_id).await?;

        // Delete the set
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Deleting order phrase set");

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
            error!(status = %status, body = %body, "Failed to delete order phrase set");
            return Err(IntelloError::storage(format!("Delete set failed: {}", status)));
        }

        info!(set_id = %set_id, "Order phrase set deleted");
        Ok(true)
    }
}
