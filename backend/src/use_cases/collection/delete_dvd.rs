//! Delete DVD Use Case
//!
//! Orchestrates deleting a DVD from a user's collection.

use crate::error::{AppError, AppResult};
use crate::services::CollectionService;
use std::sync::Arc;
use tracing::{info, instrument, warn};

/// Input for deleting a DVD
#[derive(Debug, Clone)]
pub struct DeleteDvdInput {
    pub user_id: String,
    pub dvd_id: String,
}

/// Use case for deleting a DVD
pub struct DeleteDvdUseCase {
    collection_service: Arc<CollectionService>,
}

impl DeleteDvdUseCase {
    pub fn new(collection_service: Arc<CollectionService>) -> Self {
        Self { collection_service }
    }

    #[instrument(skip(self), fields(user_id = %input.user_id, dvd_id = %input.dvd_id))]
    pub async fn execute(&self, input: DeleteDvdInput) -> AppResult<bool> {
        // Validate IDs
        if input.user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }
        if input.dvd_id.is_empty() {
            return Err(AppError::validation("dvd_id", "DVD ID is required"));
        }

        info!("Deleting DVD via use case");

        // Delegate to service
        let deleted = self
            .collection_service
            .delete_dvd(&input.user_id, &input.dvd_id)
            .await?;

        if deleted {
            info!("DVD deleted successfully");
        } else {
            warn!("DVD not found or already deleted");
        }

        Ok(deleted)
    }
}
