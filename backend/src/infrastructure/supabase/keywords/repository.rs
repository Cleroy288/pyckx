//! Supabase Keywords Repository - PostgreSQL implementation via REST API
//!
//! Implements KeywordsRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, InsertKeywordQuestionRow, InsertKeywordRow, InsertKeywordSetRow,
    KeywordQuestionRow, KeywordRow, KeywordSetRow,
};
use crate::config::Config;
use crate::domain::intello::{Keyword, KeywordQuestion, KeywordSet};
use crate::error::IntelloError;
use crate::infrastructure::repository::KeywordsRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE KEYWORDS REPOSITORY
// =============================================================================

/// Supabase implementation of KeywordsRepository
#[derive(Clone, Debug)]
pub struct SupabaseKeywordsRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseKeywordsRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseKeywordsRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_keyword_sets", self.url)
    }

    fn questions_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_keyword_questions", self.url)
    }

    fn keywords_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_keywords", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Insert keywords for a question
    async fn insert_keywords(&self, question_id: &str, keywords: &[Keyword]) -> Result<(), IntelloError> {
        if keywords.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertKeywordRow> = keywords
            .iter()
            .map(|k| InsertKeywordRow {
                id: k.id.clone(),
                question_id: question_id.to_string(),
                word: k.word.clone(),
                is_correct: k.is_correct,
            })
            .collect();

        let endpoint = self.keywords_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting keywords");

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
            error!(status = %status, body = %body, "Failed to insert keywords");
            return Err(IntelloError::storage(format!("Insert keywords failed: {}", status)));
        }

        Ok(())
    }

    /// Get keywords for a question
    async fn get_keywords(&self, question_id: &str) -> Result<Vec<Keyword>, IntelloError> {
        let endpoint = format!("{}?question_id=eq.{}", self.keywords_endpoint(), question_id);
        debug!(endpoint = %endpoint, "Getting keywords");

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
            error!(status = %status, body = %body, "Failed to get keywords");
            return Err(IntelloError::storage(format!("Get keywords failed: {}", status)));
        }

        let rows: Vec<KeywordRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let keywords: Vec<Keyword> = rows
            .into_iter()
            .map(|r| Keyword {
                id: r.id,
                word: r.word,
                is_correct: r.is_correct,
            })
            .collect();

        Ok(keywords)
    }

    /// Insert questions for a set
    async fn insert_questions(&self, set_id: &str, questions: &[KeywordQuestion]) -> Result<(), IntelloError> {
        if questions.is_empty() {
            return Ok(());
        }

        // Insert question rows
        let rows: Vec<InsertKeywordQuestionRow> = questions
            .iter()
            .map(|q| InsertKeywordQuestionRow {
                id: q.id.clone(),
                set_id: set_id.to_string(),
                statement: q.statement.clone(),
                explanation: q.explanation.clone(),
            })
            .collect();

        let endpoint = self.questions_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting keyword questions");

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
            error!(status = %status, body = %body, "Failed to insert keyword questions");
            return Err(IntelloError::storage(format!("Insert questions failed: {}", status)));
        }

        // Insert keywords for each question
        for question in questions {
            self.insert_keywords(&question.id, &question.keywords).await?;
        }

        Ok(())
    }

    /// Get questions for a set
    async fn get_questions(&self, set_id: &str) -> Result<Vec<KeywordQuestion>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting keyword questions");

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
            error!(status = %status, body = %body, "Failed to get keyword questions");
            return Err(IntelloError::storage(format!("Get questions failed: {}", status)));
        }

        let rows: Vec<KeywordQuestionRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut questions = Vec::with_capacity(rows.len());
        for row in rows {
            let keywords = self.get_keywords(&row.id).await?;
            questions.push(KeywordQuestion {
                id: row.id,
                statement: row.statement,
                keywords,
                explanation: row.explanation,
            });
        }

        Ok(questions)
    }

    /// Delete keywords for a question
    async fn delete_keywords(&self, question_id: &str) -> Result<(), IntelloError> {
        let endpoint = format!("{}?question_id=eq.{}", self.keywords_endpoint(), question_id);
        debug!(endpoint = %endpoint, "Deleting keywords");

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
            error!(status = %status, body = %body, "Failed to delete keywords");
            return Err(IntelloError::storage(format!("Delete keywords failed: {}", status)));
        }

        Ok(())
    }

    /// Delete questions and their keywords for a set
    async fn delete_questions(&self, set_id: &str) -> Result<(), IntelloError> {
        // Get questions first to delete their keywords
        let questions = self.get_questions(set_id).await?;
        for question in questions {
            self.delete_keywords(&question.id).await?;
        }

        // Delete questions
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Deleting keyword questions");

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
            error!(status = %status, body = %body, "Failed to delete keyword questions");
            return Err(IntelloError::storage(format!("Delete questions failed: {}", status)));
        }

        Ok(())
    }
}

#[async_trait]
impl KeywordsRepository for SupabaseKeywordsRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &KeywordSet) -> Result<KeywordSet, IntelloError> {
        // Insert the set
        let row = InsertKeywordSetRow {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: level_to_db(&set.level),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting keyword set");

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
            error!(status = %status, body = %body, "Failed to insert keyword set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert questions
        self.insert_questions(&set.id, &set.questions).await?;

        info!(set_id = %set.id, "Keyword set inserted");
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<KeywordSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding keyword set by ID");

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
            error!(status = %status, body = %body, "Failed to find keyword set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<KeywordSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };

        // Get questions
        let questions = self.get_questions(set_id).await?;

        let set = KeywordSet {
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
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding keyword sets by user");

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
            error!(status = %status, body = %body, "Failed to find user keyword sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<KeywordSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let questions = self.get_questions(&row.id).await?;
            sets.push(KeywordSet {
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

        info!(count = sets.len(), user_id = %user_id, "Retrieved user keyword sets");
        Ok(sets)
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "Keyword set not found for deletion");
            return Ok(false);
        }

        // Delete questions and keywords first
        self.delete_questions(set_id).await?;

        // Delete the set
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Deleting keyword set");

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
            error!(status = %status, body = %body, "Failed to delete keyword set");
            return Err(IntelloError::storage(format!("Delete set failed: {}", status)));
        }

        info!(set_id = %set_id, "Keyword set deleted");
        Ok(true)
    }
}
