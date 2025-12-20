//! Delete Collection Use Case
//!
//! Orchestrates deleting an entire collection and all its items.
//! Note: Prepared for handler migration, not yet connected.

#![allow(dead_code)]

use crate::domain::CollectionItemType;
use crate::error::{AppError, AppResult};
use crate::services::CollectionService;
use std::sync::Arc;
use tracing::{info, instrument, warn};

/// Input for deleting a collection
#[derive(Debug, Clone)]
pub struct DeleteCollectionInput {
    pub user_id: String,
    pub item_type: CollectionItemType,
}

/// Use case for deleting an entire collection
pub struct DeleteCollectionUseCase {
    collection_service: Arc<CollectionService>,
}

impl DeleteCollectionUseCase {
    pub fn new(collection_service: Arc<CollectionService>) -> Self {
        Self { collection_service }
    }

    #[instrument(skip(self), fields(user_id = %input.user_id, item_type = ?input.item_type))]
    pub async fn execute(&self, input: DeleteCollectionInput) -> AppResult<bool> {
        // Validate user_id
        if input.user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }

        info!("Deleting collection via use case");

        // Delegate to service
        let deleted = self
            .collection_service
            .delete_collection(&input.user_id, input.item_type)
            .await?;

        if deleted {
            info!("Collection deleted successfully");
        } else {
            warn!("Collection not found or already deleted");
        }

        Ok(deleted)
    }
}
