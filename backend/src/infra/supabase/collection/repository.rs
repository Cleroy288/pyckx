/*
 * Supabase Collection Repository
 *
 * Manages user collections with their types.
 * Uses shared SupabaseHttpClient for connection pooling.
 *
 * Tables: user_collections, collection_types
 */

use crate::infra::database::CollectionRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::collection::collection_domain::{CollectionItemType, UserCollection};
use crate::services::collection::error_domain::CollectionError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_COLLECTIONS: &str = "user_collections";
const TABLE_COLLECTION_TYPES: &str = "collection_types";

/* ============================================================================
 * ROW TYPES
 * ============================================================================ */

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

#[derive(Debug, Deserialize)]
struct CollectionTypeRow {
    id: i32,
}

/* ============================================================================
 * HELPER: Row Conversion
 * ============================================================================ */

impl CollectionRow {
    fn into_user_collection(
        self,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError> {
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

/* ============================================================================
 * SUPABASE COLLECTION REPOSITORY
 * ============================================================================ */

#[derive(Clone, Debug)]
pub struct SupabaseCollectionRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseCollectionRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseCollectionRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> CollectionError {
        CollectionError::storage_error(err.to_string())
    }

    /*
     * Get collection_type_id from the collection_types table.
     */
    async fn get_collection_type_id(
        &self,
        item_type: CollectionItemType,
    ) -> Result<i32, CollectionError> {
        let query = format!("name=eq.{}", item_type);
        let url = self
            .client
            .rest_url_with_query(TABLE_COLLECTION_TYPES, &query);

        let rows: Vec<CollectionTypeRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        rows.into_iter().next().map(|row| row.id).ok_or_else(|| {
            CollectionError::storage_error(format!("Collection type not found: {}", item_type))
        })
    }
}

/* ============================================================================
 * CRUD IMPLEMENTATION
 * ============================================================================ */

#[async_trait]
impl CollectionRepository for SupabaseCollectionRepository {
    /* CREATE */
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

        let url = self.client.rest_url(TABLE_COLLECTIONS);
        debug!(url = %url, "Creating collection");

        /* Handle 409 Conflict (unique constraint violation) */
        match self.client.post::<Vec<CollectionRow>, _>(&url, &row).await {
            Ok(rows) => {
                let collection_row = rows
                    .into_iter()
                    .next()
                    .ok_or_else(|| CollectionError::storage_error("No row returned"))?;

                let collection = collection_row.into_user_collection(collection_type)?;
                info!(collection_id = collection.id, "Collection created");
                Ok(collection)
            }
            Err(SupabaseError::Http {
                status: 409,
                body: _,
            }) => Err(CollectionError::storage_error(format!(
                "Collection already exists for user {} and type {}",
                user_id, collection_type
            ))),
            Err(e) => Err(Self::map_error(e)),
        }
    }

    /* DELETE */
    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn delete(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let query = format!(
            "user_id=eq.{}&collection_type_id=eq.{}",
            user_id, collection_type_id
        );
        let url = self.client.rest_url_with_query(TABLE_COLLECTIONS, &query);
        debug!(url = %url, "Deleting collection");

        self.client.delete(&url).await.map_err(Self::map_error)?;
        info!(user_id = %user_id, collection_type = %collection_type, "Collection deleted");
        Ok(true)
    }

    /* READ: Find by user */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<UserCollection>, CollectionError> {
        let query = format!("user_id=eq.{}&select=*,collection_types(*)", user_id);
        let url = self.client.rest_url_with_query(TABLE_COLLECTIONS, &query);
        debug!(url = %url, "Finding collections by user");

        let rows: Vec<CollectionWithTypeRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let collections: Result<Vec<UserCollection>, CollectionError> =
            rows.into_iter().map(UserCollection::try_from).collect();
        let collections = collections?;

        info!(count = collections.len(), "Retrieved user collections");
        Ok(collections)
    }

    /* READ: Find by type */
    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn find_by_type(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<Option<UserCollection>, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let query = format!(
            "user_id=eq.{}&collection_type_id=eq.{}",
            user_id, collection_type_id
        );
        let url = self.client.rest_url_with_query(TABLE_COLLECTIONS, &query);
        debug!(url = %url, "Finding collection by type");

        let rows: Vec<CollectionRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        if let Some(row) = rows.into_iter().next() {
            let collection = row.into_user_collection(collection_type)?;
            Ok(Some(collection))
        } else {
            Ok(None)
        }
    }

    /* HELPER: Get or create */
    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn get_or_create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError> {
        /* Try to find existing */
        if let Some(collection) = self.find_by_type(user_id, collection_type).await? {
            debug!("Found existing collection: {}", collection.id);
            return Ok(collection);
        }

        /* Try to create - handle race conditions */
        match self.create(user_id, collection_type).await {
            Ok(collection) => Ok(collection),
            Err(CollectionError::StorageError { message })
                if message.contains("already exists")
                    || message.contains("409")
                    || message.contains("Conflict") =>
            {
                debug!("Collection already exists, fetching existing one");
                self.find_by_type(user_id, collection_type)
                    .await?
                    .ok_or_else(|| {
                        CollectionError::storage_error("Collection not found after conflict")
                    })
            }
            Err(e) => Err(e),
        }
    }

    /* HELPER: Check existence */
    #[instrument(skip(self), fields(user_id = %user_id, collection_type = %collection_type))]
    async fn exists(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError> {
        let collection_type_id = self.get_collection_type_id(collection_type).await?;
        let query = format!(
            "user_id=eq.{}&collection_type_id=eq.{}&select=id",
            user_id, collection_type_id
        );
        let url = self.client.rest_url_with_query(TABLE_COLLECTIONS, &query);

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: i32,
        }

        let rows: Vec<IdOnly> = self.client.get(&url).await.map_err(Self::map_error)?;
        Ok(!rows.is_empty())
    }
}
