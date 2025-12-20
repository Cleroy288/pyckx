//! Update DVD Use Case
//!
//! Orchestrates updating an existing DVD in a user's collection.

use crate::domain::Dvd;
use crate::error::{AppError, AppResult};
use crate::services::CollectionService;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::{info, instrument};

/// Input for updating a DVD
#[derive(Debug, Clone)]
pub struct UpdateDvdInput {
    pub user_id: String,
    pub dvd_id: String,
    pub name: Option<String>,
    pub year: Option<DateTime<Utc>>,
    pub realisator: Option<String>,
    pub actors: Option<Vec<String>>,
    pub genre: Option<String>,
}

/// Use case for updating a DVD
pub struct UpdateDvdUseCase {
    collection_service: Arc<CollectionService>,
}

impl UpdateDvdUseCase {
    pub fn new(collection_service: Arc<CollectionService>) -> Self {
        Self { collection_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, dvd_id = %input.dvd_id))]
    pub async fn execute(&self, input: UpdateDvdInput) -> AppResult<Dvd> {
        // Validate IDs
        if input.user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }
        if input.dvd_id.is_empty() {
            return Err(AppError::validation("dvd_id", "DVD ID is required"));
        }

        // Validate name if provided
        if let Some(ref name) = input.name {
            if name.trim().is_empty() {
                return Err(AppError::validation("name", "DVD name cannot be empty"));
            }
        }

        info!("Updating DVD via use case");

        // Delegate to service
        let dvd = self
            .collection_service
            .update_dvd(
                &input.user_id,
                &input.dvd_id,
                input.name,
                input.year,
                input.realisator,
                input.actors,
                input.genre,
            )
            .await?;

        info!(dvd_id = %dvd.id, "DVD updated successfully");

        Ok(dvd)
    }
}
