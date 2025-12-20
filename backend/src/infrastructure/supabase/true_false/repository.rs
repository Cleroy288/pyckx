//! Supabase True or False Repository - PostgreSQL implementation via REST API
//!
//! Implements TrueOrFalseRepository trait using Supabase's PostgREST API.

use super::types::{
    level_from_db, level_to_db, InsertTrueOrFalseSetRow, InsertTrueOrFalseStatementRow,
    TrueOrFalseSetRow, TrueOrFalseStatementRow,
};
use crate::config::Config;
use crate::domain::intello::{TrueOrFalseSet, TrueOrFalseStatement};
use crate::error::IntelloError;
use crate::infrastructure::repository::GameSetRepository;
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, error, info, instrument};

// =============================================================================
// SUPABASE TRUE OR FALSE REPOSITORY
// =============================================================================

/// Supabase implementation of TrueOrFalseRepository
#[derive(Clone, Debug)]
pub struct SupabaseTrueOrFalseRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseTrueOrFalseRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseTrueOrFalseRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn sets_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_true_false_sets", self.url)
    }

    fn statements_endpoint(&self) -> String {
        format!("{}/rest/v1/intello_true_false_statements", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Insert statements for a set
    async fn insert_statements(&self, set_id: &str, statements: &[TrueOrFalseStatement]) -> Result<(), IntelloError> {
        if statements.is_empty() {
            return Ok(());
        }

        let rows: Vec<InsertTrueOrFalseStatementRow> = statements
            .iter()
            .map(|s| InsertTrueOrFalseStatementRow {
                id: s.id.clone(),
                set_id: set_id.to_string(),
                statement: s.statement.clone(),
                answer: s.answer,
                explanation: s.explanation.clone(),
            })
            .collect();

        let endpoint = self.statements_endpoint();
        debug!(endpoint = %endpoint, count = rows.len(), "Inserting true/false statements");

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
            error!(status = %status, body = %body, "Failed to insert true/false statements");
            return Err(IntelloError::storage(format!("Insert statements failed: {}", status)));
        }

        Ok(())
    }

    /// Get statements for a set
    async fn get_statements(&self, set_id: &str) -> Result<Vec<TrueOrFalseStatement>, IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.statements_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Getting true/false statements");

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
            error!(status = %status, body = %body, "Failed to get true/false statements");
            return Err(IntelloError::storage(format!("Get statements failed: {}", status)));
        }

        let rows: Vec<TrueOrFalseStatementRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let statements: Vec<TrueOrFalseStatement> = rows
            .into_iter()
            .map(|r| TrueOrFalseStatement {
                id: r.id,
                statement: r.statement,
                answer: r.answer,
                explanation: r.explanation,
            })
            .collect();

        Ok(statements)
    }

    /// Delete statements for a set
    async fn delete_statements(&self, set_id: &str) -> Result<(), IntelloError> {
        let endpoint = format!("{}?set_id=eq.{}", self.statements_endpoint(), set_id);
        debug!(endpoint = %endpoint, "Deleting true/false statements");

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
            error!(status = %status, body = %body, "Failed to delete true/false statements");
            return Err(IntelloError::storage(format!("Delete statements failed: {}", status)));
        }

        Ok(())
    }
}

#[async_trait]
impl GameSetRepository<TrueOrFalseSet> for SupabaseTrueOrFalseRepository {
    #[instrument(skip(self, set), fields(set_id = %set.id))]
    async fn insert(&self, set: &TrueOrFalseSet) -> Result<TrueOrFalseSet, IntelloError> {
        // Insert the set
        let row = InsertTrueOrFalseSetRow {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: level_to_db(&set.level),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
        };

        let endpoint = self.sets_endpoint();
        debug!(endpoint = %endpoint, "Inserting true/false set");

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
            error!(status = %status, body = %body, "Failed to insert true/false set");
            return Err(IntelloError::storage(format!("Insert set failed: {}", status)));
        }

        // Insert statements
        self.insert_statements(&set.id, &set.statements).await?;

        info!(set_id = %set.id, "True/false set inserted");
        Ok(set.clone())
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<TrueOrFalseSet>, IntelloError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Finding true/false set by ID");

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
            error!(status = %status, body = %body, "Failed to find true/false set");
            return Err(IntelloError::storage(format!("Find set failed: {}", status)));
        }

        let rows: Vec<TrueOrFalseSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };

        // Get statements
        let statements = self.get_statements(set_id).await?;

        let set = TrueOrFalseSet {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            description: row.description,
            level: level_from_db(&row.level),
            language: row.language,
            subjects: row.subjects,
            statements,
        };

        Ok(Some(set))
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        let endpoint = format!("{}?user_id=eq.{}", self.sets_endpoint(), user_id);
        debug!(endpoint = %endpoint, "Finding true/false sets by user");

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
            error!(status = %status, body = %body, "Failed to find user true/false sets");
            return Err(IntelloError::storage(format!("Find sets failed: {}", status)));
        }

        let rows: Vec<TrueOrFalseSetRow> = response
            .json()
            .await
            .map_err(|e| IntelloError::storage(format!("Parse error: {}", e)))?;

        let mut sets = Vec::with_capacity(rows.len());
        for row in rows {
            let statements = self.get_statements(&row.id).await?;
            sets.push(TrueOrFalseSet {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                description: row.description,
                level: level_from_db(&row.level),
                language: row.language,
                subjects: row.subjects,
                statements,
            });
        }

        info!(count = sets.len(), user_id = %user_id, "Retrieved user true/false sets");
        Ok(sets)
    }

    #[instrument(skip(self), fields(set_id = %set_id))]
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Check if exists
        let existing = self.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            info!(set_id = %set_id, "True/false set not found for deletion");
            return Ok(false);
        }

        // Delete statements first
        self.delete_statements(set_id).await?;

        // Delete the set
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.sets_endpoint(),
            set_id,
            user_id
        );
        debug!(endpoint = %endpoint, "Deleting true/false set");

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
            error!(status = %status, body = %body, "Failed to delete true/false set");
            return Err(IntelloError::storage(format!("Delete set failed: {}", status)));
        }

        info!(set_id = %set_id, "True/false set deleted");
        Ok(true)
    }
}
