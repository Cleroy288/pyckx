//! List DVDs Use Case
//!
//! Orchestrates retrieving all DVDs in a user's collection.

use crate::domain::Dvd;
use crate::error::{AppError, AppResult};
use crate::services::CollectionService;
use std::sync::Arc;
use tracing::{info, instrument};

/// Use case for listing DVDs in a collection
pub struct ListDvdsUseCase {
    collection_service: Arc<CollectionService>,
}

impl ListDvdsUseCase {
    pub fn new(collection_service: Arc<CollectionService>) -> Self {
        Self { collection_service }
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn execute(&self, user_id: &str) -> AppResult<Vec<Dvd>> {
        // Validate user_id
        if user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }

        // Delegate to service
        let dvds = self.collection_service.get_user_dvds(user_id).await?;

        info!(count = dvds.len(), "DVDs retrieved successfully");

        Ok(dvds)
    }
}
