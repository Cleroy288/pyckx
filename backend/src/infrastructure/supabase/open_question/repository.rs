//! Supabase Open Question Repository - PostgreSQL implementation via REST API
//!
//! Implements OpenQuestionRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, InsertOpenQuestionRow, InsertOpenQuestionSetRow,
    OpenQuestionRow, OpenQuestionSetRow,
};
use crate::config::Config;
use crate::domain::intello::{OpenQuestion, OpenQuestionSet};
use crate::error::IntelloError;
use crate::infrastructure::repository::OpenQuestionRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE OPEN QUESTION REPOSITORY
// =============================================================================

/// Supabase implementation of OpenQuestionRepository
#[derive(Clone, Debug)]
pub struct SupabaseOpenQuestionRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseOpenQuestionRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseOpenQuestionRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/open_question_sets", self.url)
    }

    fn questions_endpoint(&self) -> String {
        format!("{}/rest/v1/open_questions", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Insert questions for a set
    async fn insert_questions(
        &self,
        set_id: &str,
        questions: &[OpenQuestion],
    ) -> Result<(), IntelloError> {
        if questions.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertOpenQuestionRow> = questions
            .iter()
            .map(|q| InsertOpenQuestionRow {
                id: q.id.clone(),
                set_id: set_id.to_string(),
                question: q.question.clone(),
                user_answer: q.user_answer.clone(),
                expected_answer: q.expected_answer.clone(),
                hint: q.hint.clone(),
            })
            .collect();

        let endpoint = self.questions_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting open questions");

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
            error!(status = %status, body = %body, "Failed to insert open questions");
            return Err(IntelloError::storage(format!("Insert questions failed: {}", status)));
        }

        Ok(())
    }

    /// Get questions for a set
    async fn get_questions(&self, set_id: &str) -> Result<Vec<OpenQuestion>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting open questions");

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
            error!(status = %status, body = %body, "Failed to get open questions");
            return Err(IntelloError::storage(format!("Get questions failed: {}", status)));
        }

        let rows: Vec<OpenQuestionRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let questions: Vec<OpenQuestion> = rows
            .into_iter()
            .map(|r| OpenQuestion {
                id: r.id,
                question: r.question,
                user_answer: r.user_answer,
                expected_answer: r.expected_answer,
                hint: r.hint,
            })
            .collect();

        Ok(questions)
    }
}

#[async_trait]
impl OpenQuestionRepository for SupabaseOpenQuestionRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &OpenQuestionSet) -> Result<OpenQuestionSet, IntelloError> {
        // Insert the set
        let row = InsertOpenQuestionSetRow {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: level_to_db(&set.level),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting open question set");

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
            error!(status = %status, body = %body, "Failed to insert open question set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert questions
        self.insert_questions(&set.id, &set.questions).await?;

        info!(set_id = %set.id, "Open question set inserted");
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<OpenQuestionSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding open question set by ID");

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
            error!(status = %status, body = %body, "Failed to find open question set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<OpenQuestionSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => {
                info!(set_id = %set_id, "Open question set not found");
                return Ok(None);
            }
        };

        // Get questions
        let questions = self.get_questions(set_id).await?;

        let set = OpenQuestionSet {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            questions,
        };

        info!(set_id = %set_id, "Open question set found");
        Ok(Some(set))
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding open question sets by user");

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
            error!(status = %status, body = %body, "Failed to find user open question sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<OpenQuestionSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let questions = self.get_questions(&row.id).await?;
            sets.push(OpenQuestionSet {
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

        info!(count = sets.len(), user_id = %user_id, "Retrieved user open question sets");
        Ok(sets)
    }
}
