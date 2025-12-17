//! Supabase QCM Repository - PostgreSQL implementation via REST API
//!
//! Implements QcmRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, InsertQcmQuestionRow, InsertQcmSetRow, QcmQuestionRow, QcmSetRow,
    UpdateQcmSetRow,
};
use crate::config::Config;
use crate::domain::intello::{QcmQuestion, QcmSet};
use crate::error::IntelloError;
use crate::infrastructure::repository::QcmRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE QCM REPOSITORY
// =============================================================================

/// Supabase implementation of QcmRepository
#[derive(Clone, Debug)]
pub struct SupabaseQcmRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseQcmRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseQcmRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/qcm_sets", self.url)
    }

    fn questions_endpoint(&self) -> String {
        format!("{}/rest/v1/qcm_questions", self.url)
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
        questions: &[QcmQuestion],
    ) -> Result<(), IntelloError> {
        if questions.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertQcmQuestionRow> = questions
            .iter()
            .map(|q| InsertQcmQuestionRow {
                id: q.id.clone(),
                set_id: set_id.to_string(),
                question: q.question.clone(),
                wrong_answers: q.wrong_answers.clone(),
                right_answer: q.right_answer.clone(),
                explanation: q.explanation.clone(),
            })
            .collect();

        let endpoint = self.questions_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting QCM questions");

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
            error!(status = %status, body = %body, "Failed to insert questions");
            return Err(IntelloError::storage(format!("Insert questions failed: {}", status)));
        }

        Ok(())
    }

    /// Get questions for a set
    async fn get_questions(&self, set_id: &str) -> Result<Vec<QcmQuestion>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting QCM questions");

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
            error!(status = %status, body = %body, "Failed to get questions");
            return Err(IntelloError::storage(format!("Get questions failed: {}", status)));
        }

        let rows: Vec<QcmQuestionRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let questions: Vec<QcmQuestion> = rows
            .into_iter()
            .map(|r| QcmQuestion {
                id: r.id,
                question: r.question,
                wrong_answers: r.wrong_answers,
                right_answer: r.right_answer,
                explanation: r.explanation,
            })
            .collect();

        Ok(questions)
    }

    /// Delete questions for a set
    async fn delete_questions(&self, set_id: &str) -> Result<(), IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.questions_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Deleting QCM questions");

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
            error!(status = %status, body = %body, "Failed to delete questions");
            return Err(IntelloError::storage(format!("Delete questions failed: {}", status)));
        }

        Ok(())
    }
}

#[async_trait]
impl QcmRepository for SupabaseQcmRepository {
    #[instrument(skip(self, qcm_set), fields(set_id = %qcm_set.id))]
    async fn insert(&self, qcm_set: &QcmSet) -> Result<QcmSet, IntelloError> {
        // Insert the set
        let row = InsertQcmSetRow {
            id: qcm_set.id.clone(),
            user_id: qcm_set.user_id.clone(),
            name: qcm_set.name.clone(),
            description: qcm_set.description.clone(),
            level: level_to_db(&qcm_set.level),
            language: qcm_set.language.clone(),
            subjects: qcm_set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting QCM set");

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
            error!(status = %status, body = %body, "Failed to insert QCM set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert questions
        self.insert_questions(&qcm_set.id, &qcm_set.questions).await?;

        info!(set_id = %qcm_set.id, "QCM set inserted");
        Ok(qcm_set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<QcmSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding QCM set by ID");

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
            error!(status = %status, body = %body, "Failed to find QCM set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<QcmSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => {
                info!(set_id = %set_id, "QCM set not found");
                return Ok(None);
            }
        };

        // Get questions
        let questions = self.get_questions(set_id).await?;

        let qcm_set = QcmSet {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            questions,
        };

        info!(set_id = %set_id, "QCM set found");
        Ok(Some(qcm_set))
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<QcmSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding QCM sets by user");

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
            error!(status = %status, body = %body, "Failed to find user QCM sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<QcmSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let questions = self.get_questions(&row.id).await?;
            sets.push(QcmSet {
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

        info!(count = sets.len(), user_id = %user_id, "Retrieved user QCM sets");
        Ok(sets)
    }

    #[instrument(skip(self, qcm_set), fields(set_id = %qcm_set.id))]
    async fn update(&self, qcm_set: &QcmSet) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(&qcm_set.id, &qcm_set.user_id).await?;
        if existing.is_none() {
            info!(set_id = %qcm_set.id, "QCM set not found for update");
            return Ok(false);
        }

        // Update the set
        let row = UpdateQcmSetRow {
            name: qcm_set.name.clone(),
            description: qcm_set.description.clone(),
            level: level_to_db(&qcm_set.level),
            language: qcm_set.language.clone(),
            subjects: qcm_set.subjects.clone(),
        };

        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            qcm_set.id,
            qcm_set.user_id
        );
        debug!(endpoint = %endpoint, "Updating QCM set");

        let mut request = self.client.patch(&endpoint);
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
            error!(status = %status, body = %body, "Failed to update QCM set");
            return Err(IntelloError::storage(format!("Update set failed: {}", status)));
        }

        // Delete old questions and insert new ones
        self.delete_questions(&qcm_set.id).await?;
        self.insert_questions(&qcm_set.id, &qcm_set.questions).await?;

        info!(set_id = %qcm_set.id, "QCM set updated");
        Ok(true)
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "QCM set not found for deletion");
            return Ok(false);
        }

        // Delete questions first (cascade should handle this, but explicit is safer)
        self.delete_questions(set_id).await?;

        // Delete the set
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Deleting QCM set");

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
            error!(status = %status, body = %body, "Failed to delete QCM set");
            return Err(IntelloError::storage(format!("Delete set failed: {}", status)));
        }

        info!(set_id = %set_id, "QCM set deleted");
        Ok(true)
    }
}
