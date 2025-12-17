//! Supabase DVD Repository - PostgreSQL implementation via REST API
//!
//! Implements DvdRepository trait using Supabase's PostgREST API.

use crate::config::Config;
use crate::domain::Dvd;
use crate::error::CollectionError;
use crate::infrastructure::repository::{CreateDvd, DvdRepository, UpdateDvd};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

// == SUPABASE DVD REPOSITORY // ==

/// Supabase implementation of DvdRepository
#[derive(Clone, Debug)]
pub struct SupabaseDvdRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseDvdRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseDvdRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn dvds_endpoint(&self) -> String {
        format!("{}/rest/v1/dvds", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }
}

// == DATABASE ROW TYPES // ==

#[derive(Debug, Serialize)]
struct InsertDvdRow {
    name: String,
    year: String,
    realisator: Option<String>,
    actors: String,
    genre: Option<String>,
    user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<i32>,
}

impl From<&CreateDvd> for InsertDvdRow {
    fn from(dvd: &CreateDvd) -> Self {
        Self {
            name: dvd.name.clone(),
            year: dvd.year.to_rfc3339(),
            realisator: dvd.realisator.clone(),
            actors: dvd.actors.clone(),
            genre: dvd.genre.clone(),
            user_id: dvd.user_id.clone(),
            collection_id: if dvd.collection_id > 0 { Some(dvd.collection_id) } else { None },
        }
    }
}

#[derive(Debug, Serialize, Default)]
struct UpdateDvdRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realisator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    genre: Option<String>,
}

impl From<&UpdateDvd> for UpdateDvdRow {
    fn from(update: &UpdateDvd) -> Self {
        Self {
            name: update.name.clone(),
            year: update.year.map(|y| y.to_rfc3339()),
            realisator: update.realisator.clone(),
            actors: update.actors.clone(),
            genre: update.genre.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct DvdRow {
    id: String,
    name: String,
    year: String,
    realisator: Option<String>,
    actors: String,
    genre: Option<String>,
    created_at: String,
    updated_at: String,
    user_id: String,
    #[serde(default)]
    collection_id: Option<i32>,
}

impl TryFrom<DvdRow> for Dvd {
    type Error = CollectionError;

    fn try_from(row: DvdRow) -> Result<Self, Self::Error> {
        use chrono::DateTime;

        let year = DateTime::parse_from_rfc3339(&row.year)
            .map_err(|e| CollectionError::storage_error(format!("Invalid year format: {}", e)))?
            .with_timezone(&chrono::Utc);

        let created_at = DateTime::parse_from_rfc3339(&row.created_at)
            .map_err(|e| CollectionError::storage_error(format!("Invalid created_at format: {}", e)))?
            .with_timezone(&chrono::Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|e| CollectionError::storage_error(format!("Invalid updated_at format: {}", e)))?
            .with_timezone(&chrono::Utc);

        Ok(Dvd {
            id: row.id,
            collection_id: row.collection_id.unwrap_or(0),
            user_id: row.user_id,
            name: row.name,
            year,
            realisator: row.realisator,
            actors: row.actors,
            genre: row.genre,
            created_at,
            updated_at,
        })
    }
}

// == REPOSITORY IMPLEMENTATION // ==

#[async_trait]
impl DvdRepository for SupabaseDvdRepository {
    #[instrument(skip(self, dvd), fields(user_id = %dvd.user_id, name = %dvd.name))]
    async fn insert(&self, dvd: &CreateDvd) -> Result<Dvd, CollectionError> {
        // Check for duplicate name first
        if self.exists_by_name(&dvd.user_id, &dvd.name).await? {
            return Err(CollectionError::dvd_duplicate(&dvd.name));
        }

        let row = InsertDvdRow::from(dvd);
        let endpoint = self.dvds_endpoint();

        debug!(endpoint = %endpoint, "Inserting DVD");

        let mut request = self.client.post(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .json(&row)
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase insert failed");
            return Err(CollectionError::storage_error(format!("Insert failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let dvd_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CollectionError::storage_error("No row returned from insert"))?;

        let created_dvd = Dvd::try_from(dvd_row)?;
        info!(dvd_id = %created_dvd.id, "DVD inserted successfully");

        Ok(created_dvd)
    }

    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    async fn find_by_id(&self, user_id: &str, dvd_id: &str) -> Result<Dvd, CollectionError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.dvds_endpoint(),
            dvd_id,
            user_id
        );

        debug!(endpoint = %endpoint, "Finding DVD by ID");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase select failed");
            return Err(CollectionError::storage_error(format!("Select failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let dvd_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CollectionError::dvd_not_found(dvd_id))?;

        Dvd::try_from(dvd_row)
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_all(&self, user_id: &str) -> Result<Vec<Dvd>, CollectionError> {
        let endpoint = format!(
            "{}?user_id=eq.{}&order=created_at.desc",
            self.dvds_endpoint(),
            user_id
        );

        debug!(endpoint = %endpoint, "Finding all DVDs for user");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase select failed");
            return Err(CollectionError::storage_error(format!("Select failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let dvds: Result<Vec<Dvd>, CollectionError> = rows.into_iter().map(Dvd::try_from).collect();
        let dvds = dvds?;
        info!(count = dvds.len(), "Retrieved DVDs for user");

        Ok(dvds)
    }

    #[instrument(skip(self), fields(collection_id = %collection_id))]
    async fn find_by_collection(&self, collection_id: i32) -> Result<Vec<Dvd>, CollectionError> {
        let endpoint = format!(
            "{}?collection_id=eq.{}&order=created_at.desc",
            self.dvds_endpoint(),
            collection_id
        );

        debug!(endpoint = %endpoint, "Finding DVDs by collection");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase select failed");
            return Err(CollectionError::storage_error(format!("Select failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let dvds: Result<Vec<Dvd>, CollectionError> = rows.into_iter().map(Dvd::try_from).collect();
        dvds
    }

    #[instrument(skip(self, update), fields(user_id = %user_id, dvd_id = %dvd_id))]
    async fn update(
        &self,
        user_id: &str,
        dvd_id: &str,
        update: &UpdateDvd,
    ) -> Result<Dvd, CollectionError> {
        // Check if DVD exists
        self.find_by_id(user_id, dvd_id).await?;

        // Check for duplicate name if name is being updated
        if let Some(ref new_name) = update.name {
            let endpoint = format!(
                "{}?user_id=eq.{}&name=ilike.{}&id=neq.{}",
                self.dvds_endpoint(),
                user_id,
                new_name,
                dvd_id
            );

            let mut request = self.client.get(&endpoint);
            for (key, value) in self.headers() {
                request = request.header(key, value);
            }

            let response = request
                .send()
                .await
                .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

            let rows: Vec<DvdRow> = response
                .json()
                .await
                .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

            if !rows.is_empty() {
                return Err(CollectionError::dvd_duplicate(new_name));
            }
        }

        let row = UpdateDvdRow::from(update);
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.dvds_endpoint(),
            dvd_id,
            user_id
        );

        debug!(endpoint = %endpoint, "Updating DVD");

        let mut request = self.client.patch(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .json(&row)
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase update failed");
            return Err(CollectionError::storage_error(format!("Update failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let dvd_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CollectionError::dvd_not_found(dvd_id))?;

        let updated_dvd = Dvd::try_from(dvd_row)?;
        info!(dvd_id = %updated_dvd.id, "DVD updated successfully");

        Ok(updated_dvd)
    }

    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    async fn delete(&self, user_id: &str, dvd_id: &str) -> Result<bool, CollectionError> {
        let endpoint = format!(
            "{}?id=eq.{}&user_id=eq.{}",
            self.dvds_endpoint(),
            dvd_id,
            user_id
        );

        debug!(endpoint = %endpoint, "Deleting DVD");

        let mut request = self.client.delete(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase delete failed");
            return Err(CollectionError::storage_error(format!("Delete failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response.json().await.unwrap_or_default();
        let deleted = !rows.is_empty();

        if deleted {
            info!(dvd_id = %dvd_id, "DVD deleted successfully");
        } else {
            debug!(dvd_id = %dvd_id, "DVD not found for deletion");
        }

        Ok(deleted)
    }

    #[instrument(skip(self), fields(user_id = %user_id, name = %name))]
    async fn exists_by_name(&self, user_id: &str, name: &str) -> Result<bool, CollectionError> {
        let endpoint = format!(
            "{}?user_id=eq.{}&name=ilike.{}&select=id",
            self.dvds_endpoint(),
            user_id,
            name
        );

        debug!(endpoint = %endpoint, "Checking if DVD exists by name");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: String,
        }

        let rows: Vec<IdOnly> = response.json().await.unwrap_or_default();
        Ok(!rows.is_empty())
    }

    #[instrument(skip(self), fields(collection_id = %collection_id))]
    async fn delete_by_collection(&self, collection_id: i32) -> Result<usize, CollectionError> {
        let endpoint = format!(
            "{}?collection_id=eq.{}",
            self.dvds_endpoint(),
            collection_id
        );

        debug!(endpoint = %endpoint, "Deleting DVDs by collection");

        let mut request = self.client.delete(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase delete failed");
            return Err(CollectionError::storage_error(format!("Delete failed: {} - {}", status, body)));
        }

        let rows: Vec<DvdRow> = response.json().await.unwrap_or_default();
        let count = rows.len();

        info!(count = count, "DVDs deleted from collection");
        Ok(count)
    }
}
