//! DVD operations
//!
//! CRUD operations for DVDs within a collection.

use super::CollectionService;
use crate::infra::{CreateDvd, UpdateDvd};
use crate::services::collection::collection_domain::CollectionItemType;
use crate::services::collection::dvd_domain::Dvd;
use crate::services::collection::error_domain::CollectionError;
use crate::shared::AppResult;
use chrono::{DateTime, Utc};
use tracing::{info, instrument};

impl CollectionService {
    /// Add a new DVD to a user's collection
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self), fields(user_id = %user_id, dvd_name = %name))]
    pub async fn add_dvd(
        &self,
        user_id: &str,
        name: String,
        year: DateTime<Utc>,
        realisator: Option<String>,
        actors: Vec<String>,
        genre: Option<String>,
    ) -> AppResult<Dvd> {
        // Get or create the DVD collection
        let collection = self
            .get_or_create_collection(user_id, CollectionItemType::Dvd)
            .await?;

        let create_dvd = CreateDvd::new(
            name,
            year,
            realisator,
            actors,
            genre,
            user_id,
            collection.id,
        );

        let dvd = self.dvd_repo.insert(&create_dvd).await?;
        info!(dvd_id = %dvd.id, "DVD added to collection");
        Ok(dvd)
    }

    /// Get all DVDs for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_dvds(&self, user_id: &str) -> AppResult<Vec<Dvd>> {
        let dvds = self.dvd_repo.find_all(user_id).await?;
        info!(count = dvds.len(), "Retrieved user DVDs");
        Ok(dvds)
    }

    /// Find a specific DVD by ID
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    pub async fn find_dvd(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> AppResult<Dvd> {
        let dvd = self.dvd_repo.find_by_id(user_id, dvd_id).await?;
        Ok(dvd)
    }

    /// Update a DVD's information
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    pub async fn update_dvd(
        &self,
        user_id: &str,
        dvd_id: &str,
        name: Option<String>,
        year: Option<DateTime<Utc>>,
        realisator: Option<String>,
        actors: Option<Vec<String>>,
        genre: Option<String>,
    ) -> AppResult<Dvd> {
        let update = UpdateDvd {
            name,
            year,
            realisator,
            actors: actors.map(|a| a.join(", ")),
            genre,
        };

        let updated = self.dvd_repo.update(user_id, dvd_id, &update).await?;
        info!(dvd_id = %dvd_id, "DVD updated");
        Ok(updated)
    }

    /// Delete a DVD from a user's collection
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    pub async fn delete_dvd(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> AppResult<bool> {
        let deleted = self.dvd_repo.delete(user_id, dvd_id).await?;

        if !deleted {
            return Err(CollectionError::dvd_not_found(dvd_id).into());
        }

        info!(dvd_id = %dvd_id, "DVD deleted from collection");
        Ok(true)
    }

    /// Get all DVDs for a user (collection items)
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_collection_dvds(
        &self,
        user_id: &str,
    ) -> AppResult<Vec<Dvd>> {
        let dvds = self.dvd_repo.find_all(user_id).await?;
        Ok(dvds)
    }

    /// Get the count of DVDs in a user's collection
    #[allow(dead_code)]
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_dvd_count(&self, user_id: &str) -> AppResult<usize> {
        let dvds = self.dvd_repo.find_all(user_id).await?;
        Ok(dvds.len())
    }
}
