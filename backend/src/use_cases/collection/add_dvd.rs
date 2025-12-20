//! Add DVD Use Case
//!
//! Orchestrates adding a new DVD to a user's collection.

use crate::domain::Dvd;
use crate::error::{AppError, AppResult};
use crate::services::CollectionService;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::{info, instrument};

/// Input for adding a DVD
#[derive(Debug, Clone)]
pub struct AddDvdInput {
    pub user_id: String,
    pub name: String,
    pub year: DateTime<Utc>,
    pub realisator: Option<String>,
    pub actors: Vec<String>,
    pub genre: Option<String>,
}

/// Use case for adding a DVD to a collection
pub struct AddDvdUseCase {
    collection_service: Arc<CollectionService>,
}

impl AddDvdUseCase {
    pub fn new(collection_service: Arc<CollectionService>) -> Self {
        Self { collection_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: AddDvdInput) -> AppResult<Dvd> {
        // Validate name
        if input.name.trim().is_empty() {
            return Err(AppError::validation("name", "DVD name is required"));
        }

        // Validate user_id
        if input.user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }

        info!(
            name = %input.name,
            actors = input.actors.len(),
            "Adding DVD via use case"
        );

        // Delegate to service
        let dvd = self
            .collection_service
            .add_dvd(
                &input.user_id,
                input.name,
                input.year,
                input.realisator,
                input.actors,
                input.genre,
            )
            .await?;

        info!(dvd_id = %dvd.id, "DVD added successfully");

        Ok(dvd)
    }
}
