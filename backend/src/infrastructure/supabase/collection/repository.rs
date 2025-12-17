//! Supabase Collection Repository - PostgreSQL implementation via REST API
//!
//! Implements CollectionRepository trait using Supabase's PostgREST API.
//! Manages user_collections table operations.

use crate::config::Config;
use crate::domain::{CollectionItemType, UserCollection};
use crate::error::CollectionError;
use crate::infrastructure::repository::CollectionRepository;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

// == SUPABASE COLLECTION REPOSITORY // ==

/// Supabase implementation of CollectionRepository
#[derive(Clone, Debug)]
pub struct SupabaseCollectionRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseCollectionRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseCollectionRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn collections_endpoint(&self) -> String {
        format!("{}/rest/v1/user_collections", self.url)
    }

    fn collection_types_endpoint(&self) -> String {
        format!("{}/rest/v1/collection_types", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }

    /// Get collection_type_id from the collection_types table
    async fn get_collection_type_id(&self, item_type: CollectionItemType) -> Result<i32, CollectionError> {
        let endpoint = format!(
            "{}?name=eq.{}",
            self.collection_types_endpoint(),
            item_type
        );

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            return Err(CollectionError::storage_error(format!(
                "Failed to get collection type: {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct CollectionTypeRow {
            id: i32,
        }

        let rows: Vec<CollectionTypeRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        rows.into_iter()
            .next()
            .map(|row| row.id)
            .ok_or_else(|| CollectionError::storage_error(format!("Collection type not found: {}", item_type)))
    }
}

// == DATABASE ROW TYPES // ==

#[derive(Debug, Serialize)]
struct InsertCollectionRow {
    user_id: String,
    collection_type_id: i32,
}

#[derive(Debug, Deserialize)]
struct CollectionRow {
    id: i32,
    user_id: String,
    #[allow(dead_code)]
    collection_type_id: i32,
    #[serde(default)]
    created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CollectionWithTypeRow {
    id: i32,
    user_id: String,
    #[allow(dead_code)]
    collection_type_id: i32,
    #[serde(default)]
    created_at: Option<String>,
    collection_types: Option<CollectionTypeInfo>,
}

#[derive(Debug, Deserialize)]
struct CollectionTypeInfo {
    name: String,
}

impl CollectionRow {
    fn into_user_collection(self, collection_type: CollectionItemType) -> Result<UserCollection, CollectionError> {
        use chrono::{DateTime, Utc};

        let created_at = match self.created_at {
            Some(ts) => DateTime::parse_from_rfc3339(&ts)
                .map_err(|e| CollectionError::storage_error(format!("Invalid created_at: {}", e)))?
                .with_timezone(&Utc),
            None => Utc::now(),
        };

        Ok(UserCollection {
            id: self.id,
            user_id: self.user_id,
            collection_type,
            created_at,
        })
    }
}

impl TryFrom<CollectionWithTypeRow> for UserCollection {
    type Error = CollectionError;

    fn try_from(row: CollectionWithTypeRow) -> Result<Self, Self::Error> {
        use chrono::{DateTime, Utc};

        let created_at = match row.created_at {
            Some(ts) => DateTime::parse_from_rfc3339(&ts)
                .map_err(|e| CollectionError::storage_error(format!("Invalid created_at: {}", e)))?
                .with_timezone(&Utc),
            None => Utc::now(),
        };

        let collection_type = row
            .collection_types
            .and_then(|ct| CollectionItemType::from_str(&ct.name))
            .ok_or_else(|| CollectionError::storage_error("Invalid collection type"))?;

        Ok(UserCollection {
            id: row.id,
            user_id: row.user_id,
            collection_type,
            created_at,
        })
    }
}

// == REPOSITORY IMPLEMENTATION // ==

#[async_trait]
impl CollectionRepository for SupabaseCollectionRepository {
    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let row = InsertCollectionRow {
            user_id: user_id.to_string(),
            collection_type_id,
        };

        let endpoint = self.collections_endpoint();
        debug!(endpoint = %endpoint, "Creating collection");

        let mut request = self.client.post(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .json(&row)
            .send()
            .await
            .map_err(|e| CollectionError::storage_error(format!("HTTP error: {}", e)))?;

        // Handle 409 Conflict (collection already exists due to unique constraint)
        if response.status() == reqwest::StatusCode::CONFLICT {
            return Err(CollectionError::storage_error(format!(
                "Collection already exists for user {} and type {}",
                user_id, collection_type
            )));
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase insert failed");
            return Err(CollectionError::storage_error(format!("Insert failed: {}", status)));
        }

        let rows: Vec<CollectionRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let collection_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CollectionError::storage_error("No row returned"))?;

        let collection = collection_row.into_user_collection(collection_type)?;

        info!(collection_id = collection.id, "Collection created");
        Ok(collection)
    }

    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn delete(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let endpoint = format!(
            "{}?user_id=eq.{}&collection_type_id=eq.{}",
            self.collections_endpoint(),
            user_id,
            collection_type_id
        );
        debug!(endpoint = %endpoint, "Deleting collection");

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
            return Err(CollectionError::storage_error(format!("Delete failed: {}", status)));
        }

        let rows: Vec<CollectionRow> = response.json().await.unwrap_or_default();
        let deleted = !rows.is_empty();

        if deleted {
            info!(user_id = %user_id, collection_type = %collection_type, "Collection deleted");
        }
        Ok(deleted)
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<UserCollection>, CollectionError> {
        let endpoint = format!(
            "{}?user_id=eq.{}&select=*,collection_types(*)",
            self.collections_endpoint(),
            user_id
        );
        debug!(endpoint = %endpoint, "Finding collections by user");

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
            return Err(CollectionError::storage_error(format!("Select failed: {}", status)));
        }

        let rows: Vec<CollectionWithTypeRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        let collections: Result<Vec<UserCollection>, CollectionError> =
            rows.into_iter().map(UserCollection::try_from).collect();
        let collections = collections?;

        info!(count = collections.len(), "Retrieved user collections");
        Ok(collections)
    }

    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn find_by_type(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<Option<UserCollection>, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let endpoint = format!(
            "{}?user_id=eq.{}&collection_type_id=eq.{}",
            self.collections_endpoint(),
            user_id,
            collection_type_id
        );
        debug!(endpoint = %endpoint, "Finding collection by type");

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
            return Err(CollectionError::storage_error(format!("Select failed: {}", status)));
        }

        let rows: Vec<CollectionRow> = response
            .json()
            .await
            .map_err(|e| CollectionError::storage_error(format!("Parse error: {}", e)))?;

        if let Some(row) = rows.into_iter().next() {
            let collection = row.into_user_collection(collection_type)?;
            Ok(Some(collection))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn get_or_create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError> {
        // First try to find existing collection
        if let Some(collection) = self.find_by_type(user_id, collection_type).await? {
            debug!("Found existing collection: {}", collection.id);
            return Ok(collection);
        }

        // Try to create - if it fails due to conflict (race condition), fetch the existing one
        match self.create(user_id, collection_type).await {
            Ok(collection) => Ok(collection),
            Err(CollectionError::StorageError { message }) 
                if message.contains("already exists") || message.contains("409") || message.contains("Conflict") => {
                // Race condition or already exists: fetch the existing one
                debug!("Collection already exists, fetching existing one");
                self.find_by_type(user_id, collection_type)
                    .await?
                    .ok_or_else(|| CollectionError::storage_error("Collection not found after conflict"))
            }
            Err(e) => Err(e),
        }
    }

    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn exists(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let endpoint = format!(
            "{}?user_id=eq.{}&collection_type_id=eq.{}&select=id",
            self.collections_endpoint(),
            user_id,
            collection_type_id
        );

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
            id: i32,
        }

        let rows: Vec<IdOnly> = response.json().await.unwrap_or_default();
        Ok(!rows.is_empty())
    }
}
