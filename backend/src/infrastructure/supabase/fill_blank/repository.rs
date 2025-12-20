//! Supabase Fill Blank Repository - PostgreSQL implementation via REST API
//!
//! Implements FillBlankRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, InsertFillBlankOptionRow, InsertFillBlankQuestionRow,
    InsertFillBlankSetRow, FillBlankOptionRow, FillBlankQuestionRow, FillBlankSetRow,
};
use crate::config::Config;
use crate::domain::intello::{FillBlankOption, FillBlankQuestion, FillBlankSet};
use crate::error::IntelloError;
use crate::infrastructure::repository::GameSetRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE FILL BLANK REPOSITORY
// =============================================================================

/// Supabase implementation of FillBlankRepository
#[derive(Clone, Debug)]
pub struct SupabaseFillBlankRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseFillBlankRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseFillBlankRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_fill_blank_sets", self.url)
    }

    fn questions_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_fill_blank_questions", self.url)
    }

    fn options_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_fill_blank_options", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Insert options for a question
    async fn insert_options(&self, question_id: &str, options: &[FillBlankOption]) -> Result<(), IntelloError> {
        if options.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertFillBlankOptionRow> = options
            .iter()
            .map(|o| InsertFillBlankOptionRow {
                id: o.id.clone(),
                question_id: question_id.to_string(),
                text: o.text.clone(),
                is_correct: o.is_correct,
            })
            .collect();

        let endpoint = self.options_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting fill blank options");

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
            error!(status = %status, body = %body, "Failed to insert options");
            return Err(IntelloError::storage(format!("Insert options failed: {}", status)));
        }

        Ok(())
    }

    /// Get options for a question
    async fn get_options(&self, question_id: &str) -> Result<Vec<FillBlankOption>, IntelloError> {
        let endpoint = format!("{}?question_id=eq.{}", self.options_endpoint(), question_id);
        debug!(endpoint = %endpoint, "Getting fill blank options");

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
            error!(status = %status, body = %body, "Failed to get options");
            return Err(IntelloError::storage(format!("Get options failed: {}", status)));
        }

        let rows: Vec<FillBlankOptionRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let options: Vec<FillBlankOption> = rows
            .into_iter()
            .map(|r| FillBlankOption {
                id: r.id,
                text: r.text,
                is_correct: r.is_correct,
            })
            .collect();

        Ok(options)
    }

    /// Insert questions for a set
    async fn insert_questions(&self, set_id: &str, questions: &[FillBlankQuestion]) -> Result<(), IntelloError> {
        if questions.is_empty() {
            return Ok(());
        }

        // Insert question rows
        let rows: Vec<InsertFillBlankQuestionRow> = questions
            .iter()
            .map(|q| InsertFillBlankQuestionRow {
                id: q.id.clone(),
                set_id: set_id.to_string(),
                phrase: q.phrase.clone(),
                explanation: q.explanation.clone(),
            })
            .collect();

        let endpoint = self.questions_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting fill blank questions");

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
            error!(status = %status, body = %body, "Failed to insert fill blank questions");
            return Err(IntelloError::storage(format!("Insert questions failed: {}", status)));
        }

        // Insert options for each question
        for question in questions {
            self.insert_options(&question.id, &question.options).await?;
        }

        Ok(())
    }

    /// Get questions for a set
    async fn get_questions(&self, set_id: &str) -> Result<Vec<FillBlankQuestion>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting fill blank questions");

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
            error!(status = %status, body = %body, "Failed to get fill blank questions");
            return Err(IntelloError::storage(format!("Get questions failed: {}", status)));
        }

        let rows: Vec<FillBlankQuestionRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut questions = Vec::with_capacity(rows.len());
        for row in rows {
            let options = self.get_options(&row.id).await?;
            questions.push(FillBlankQuestion {
                id: row.id,
                phrase: row.phrase,
                options,
                explanation: row.explanation,
            });
        }

        Ok(questions)
    }

    /// Delete options for a question
    async fn delete_options(&self, question_id: &str) -> Result<(), IntelloError> {
        let endpoint = format!("{}?question_id=eq.{}", self.options_endpoint(), question_id);
        debug!(endpoint = %endpoint, "Deleting fill blank options");

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
            error!(status = %status, body = %body, "Failed to delete options");
            return Err(IntelloError::storage(format!("Delete options failed: {}", status)));
        }

        Ok(())
    }

    /// Delete questions and their options for a set
    async fn delete_questions(&self, set_id: &str) -> Result<(), IntelloError> {
        // Get questions first to delete their options
        let questions = self.get_questions(set_id).await?;
        for question in questions {
            self.delete_options(&question.id).await?;
        }

        // Delete questions
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Deleting fill blank questions");

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
            error!(status = %status, body = %body, "Failed to delete fill blank questions");
            return Err(IntelloError::storage(format!("Delete questions failed: {}", status)));
        }

        Ok(())
    }
}

#[async_trait]
impl GameSetRepository<FillBlankSet> for SupabaseFillBlankRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &FillBlankSet) -> Result<FillBlankSet, IntelloError> {
        // Insert the set
        let row = InsertFillBlankSetRow {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: level_to_db(&set.level),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting fill blank set");

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
            error!(status = %status, body = %body, "Failed to insert fill blank set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert questions
        self.insert_questions(&set.id, &set.questions).await?;

        info!(set_id = %set.id, "Fill blank set inserted");
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<FillBlankSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding fill blank set by ID");

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
            error!(status = %status, body = %body, "Failed to find fill blank set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<FillBlankSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };

        // Get questions
        let questions = self.get_questions(set_id).await?;

        let set = FillBlankSet {
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
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FillBlankSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding fill blank sets by user");

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
            error!(status = %status, body = %body, "Failed to find user fill blank sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<FillBlankSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let questions = self.get_questions(&row.id).await?;
            sets.push(FillBlankSet {
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

        info!(count = sets.len(), user_id = %user_id, "Retrieved user fill blank sets");
        Ok(sets)
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "Fill blank set not found for deletion");
            return Ok(false);
        }

        // Delete questions and options first
        self.delete_questions(set_id).await?;

        // Delete the set
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Deleting fill blank set");

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
            error!(status = %status, body = %body, "Failed to delete fill blank set");
            return Err(IntelloError::storage(format!("Delete set failed: {}", status)));
        }

        info!(set_id = %set_id, "Fill blank set deleted");
        Ok(true)
    }
}
