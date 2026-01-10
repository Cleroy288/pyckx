//! Collection management operations
//!
//! CRUD operations for collections themselves.

use super::CollectionService;
use crate::services::collection::collection_domain::{CollectionItemType, UserCollection};
use crate::shared::AppResult;
use tracing::{info, instrument};

impl CollectionService {
    /// Add a new collection for a user
    #[instrument(skip(self), fields(user_id = %user_id, item_type = %item_type))]
    pub async fn add_collection(
        &self,
        user_id: &str,
        item_type: CollectionItemType,
    ) -> AppResult<UserCollection> {
        let collection = self.collection_repo.create(user_id, item_type).await?;
        info!(collection_id = collection.id, "Collection created");
        Ok(collection)
    }

    /// Delete a collection and all its items
    #[instrument(skip(self), fields(user_id = %user_id, item_type = %item_type))]
    pub async fn delete_collection(
        &self,
        user_id: &str,
        item_type: CollectionItemType,
    ) -> AppResult<bool> {
        // First, find the collection to get its ID
        let collection = self
            .collection_repo
            .find_by_type(user_id, item_type)
            .await?;

        if let Some(coll) = collection {
            // Delete all items in the collection first (only DVDs supported)
            if item_type == CollectionItemType::Dvd {
                self.dvd_repo.delete_by_collection(coll.id).await?;
            }

            // Then delete the collection itself
            let deleted = self.collection_repo.delete(user_id, item_type).await?;
            info!(collection_id = coll.id, "Collection and items deleted");
            Ok(deleted)
        } else {
            Ok(false)
        }
    }

    /// Get all collections for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_collections(&self, user_id: &str) -> AppResult<Vec<UserCollection>> {
        let collections = self.collection_repo.find_by_user(user_id).await?;
        info!(count = collections.len(), "Retrieved user collections");
        Ok(collections)
    }

    /// Get or create a collection for a user (idempotent)
    #[instrument(skip(self), fields(user_id = %user_id, item_type = %item_type))]
    pub async fn get_or_create_collection(
        &self,
        user_id: &str,
        item_type: CollectionItemType,
    ) -> AppResult<UserCollection> {
        let collection = self
            .collection_repo
            .get_or_create(user_id, item_type)
            .await?;
        Ok(collection)
    }
}
