/*
 * Supabase DVD Repository
 *
 * CRUD implementation for DVD management.
 * Uses shared SupabaseHttpClient for connection pooling.
 *
 * Tables: dvds
 */

use crate::infra::database::{CreateDvd, DvdRepository, UpdateDvd};
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::collection::dvd_domain::Dvd;
use crate::services::collection::error_domain::CollectionError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_DVDS: &str = "dvds";

/* ============================================================================
 * ROW TYPES
 * ============================================================================ */

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
            user_id: dvd.user_id.to_string(),
            collection_id: if dvd.collection_id > 0 {
                Some(dvd.collection_id)
            } else {
                None
            },
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
            .map_err(|err| {
                CollectionError::storage_error(format!(
                    "Invalid year format: {}",
                    err
                ))
            })?
            .with_timezone(&chrono::Utc);

        let created_at = DateTime::parse_from_rfc3339(&row.created_at)
            .map_err(|err| {
                CollectionError::storage_error(format!(
                    "Invalid created_at format: {}",
                    err
                ))
            })?
            .with_timezone(&chrono::Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|err| {
                CollectionError::storage_error(format!(
                    "Invalid updated_at format: {}",
                    err
                ))
            })?
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

/* ============================================================================
 * SUPABASE DVD REPOSITORY
 * ============================================================================ */

#[derive(Clone, Debug)]
pub struct SupabaseDvdRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseDvdRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseDvdRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> CollectionError {
        CollectionError::storage_error(err.to_string())
    }
}

/* ============================================================================
 * CRUD IMPLEMENTATION
 * ============================================================================ */

#[async_trait]
impl DvdRepository for SupabaseDvdRepository {
    /* CREATE */
    #[instrument(skip(self, dvd), fields(user_id = %dvd.user_id, name = %dvd.name))]
    async fn insert(&self, dvd: &CreateDvd) -> Result<Dvd, CollectionError> {
        /* Check for duplicates */
        if self.exists_by_name(&dvd.user_id, &dvd.name).await? {
            return Err(CollectionError::dvd_duplicate(&dvd.name));
        }

        let row = InsertDvdRow::from(dvd);
        let url = self.client.rest_url(TABLE_DVDS);
        debug!(url = %url, "Inserting DVD");

        let rows: Vec<DvdRow> = self
            .client
            .post(&url, &row)
            .await
            .map_err(Self::map_error)?;

        let dvd_row = rows.into_iter().next().ok_or_else(|| {
            CollectionError::storage_error("No row returned from insert")
        })?;

        let created_dvd = Dvd::try_from(dvd_row)?;
        info!(dvd_id = %created_dvd.id, "DVD inserted successfully");

        Ok(created_dvd)
    }

    /* READ: By ID */
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    async fn find_by_id(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> Result<Dvd, CollectionError> {
        let query = format!("id=eq.{}&user_id=eq.{}", dvd_id, user_id);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Finding DVD by ID");

        let rows: Vec<DvdRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let dvd_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CollectionError::dvd_not_found(dvd_id))?;

        Dvd::try_from(dvd_row)
    }

    /* READ: All for user */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_all(
        &self,
        user_id: &str,
    ) -> Result<Vec<Dvd>, CollectionError> {
        let query = format!("user_id=eq.{}&order=created_at.desc", user_id);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Finding all DVDs for user");

        let rows: Vec<DvdRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let dvds: Result<Vec<Dvd>, CollectionError> =
            rows.into_iter().map(Dvd::try_from).collect();
        let dvds = dvds?;
        info!(count = dvds.len(), "Retrieved DVDs for user");

        Ok(dvds)
    }

    /* READ: By collection */
    #[instrument(skip(self), fields(collection_id = %collection_id))]
    async fn find_by_collection(
        &self,
        collection_id: i32,
    ) -> Result<Vec<Dvd>, CollectionError> {
        let query =
            format!("collection_id=eq.{}&order=created_at.desc", collection_id);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Finding DVDs by collection");

        let rows: Vec<DvdRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let dvds: Result<Vec<Dvd>, CollectionError> =
            rows.into_iter().map(Dvd::try_from).collect();
        dvds
    }

    /* UPDATE */
    #[instrument(skip(self, update), fields(user_id = %user_id, dvd_id = %dvd_id))]
    async fn update(
        &self,
        user_id: &str,
        dvd_id: &str,
        update: &UpdateDvd,
    ) -> Result<Dvd, CollectionError> {
        /* Check existence */
        self.find_by_id(user_id, dvd_id).await?;

        /* Check for name collision if updating name */
        if let Some(ref new_name) = update.name {
            let query = format!(
                "user_id=eq.{}&name=ilike.{}&id=neq.{}",
                user_id, new_name, dvd_id
            );
            let url = self.client.rest_url_with_query(TABLE_DVDS, &query);

            let rows: Vec<DvdRow> =
                self.client.get(&url).await.map_err(Self::map_error)?;
            if !rows.is_empty() {
                return Err(CollectionError::dvd_duplicate(new_name));
            }
        }

        let row = UpdateDvdRow::from(update);
        let query = format!("id=eq.{}&user_id=eq.{}", dvd_id, user_id);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Updating DVD");

        let rows: Vec<DvdRow> = self
            .client
            .patch(&url, &row)
            .await
            .map_err(Self::map_error)?;

        let dvd_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CollectionError::dvd_not_found(dvd_id))?;

        let updated_dvd = Dvd::try_from(dvd_row)?;
        info!(dvd_id = %updated_dvd.id, "DVD updated successfully");

        Ok(updated_dvd)
    }

    /* DELETE */
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    async fn delete(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> Result<bool, CollectionError> {
        let query = format!("id=eq.{}&user_id=eq.{}", dvd_id, user_id);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Deleting DVD");

        self.client.delete(&url).await.map_err(Self::map_error)?;
        info!(dvd_id = %dvd_id, "DVD deleted successfully");
        Ok(true)
    }

    /* HELPER: Check existence by name */
    #[instrument(skip(self), fields(user_id = %user_id, name = %name))]
    async fn exists_by_name(
        &self,
        user_id: &str,
        name: &str,
    ) -> Result<bool, CollectionError> {
        let query =
            format!("user_id=eq.{}&name=ilike.{}&select=id", user_id, name);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Checking if DVD exists by name");

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: String,
        }

        let rows: Vec<IdOnly> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        Ok(!rows.is_empty())
    }

    /* DELETE: By collection */
    #[instrument(skip(self), fields(collection_id = %collection_id))]
    async fn delete_by_collection(
        &self,
        collection_id: i32,
    ) -> Result<usize, CollectionError> {
        let query = format!("collection_id=eq.{}", collection_id);
        let url = self.client.rest_url_with_query(TABLE_DVDS, &query);
        debug!(url = %url, "Deleting DVDs by collection");

        self.client.delete(&url).await.map_err(Self::map_error)?;
        /* Note: Supabase DELETE with Prefer=representation would return deleted rows,
         * but we use default delete which returns empty. Return 1 to indicate success. */
        info!(collection_id = %collection_id, "DVDs deleted from collection");
        Ok(1)
    }
}
