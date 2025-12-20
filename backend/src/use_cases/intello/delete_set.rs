//! Delete Set Use Case
//!
//! Unified use case for deleting any type of content set.
//! Note: Prepared for handler migration, not yet connected.

#![allow(dead_code)]

use crate::error::{AppError, AppResult};
use crate::services::IntelloService;
use std::sync::Arc;
use tracing::{info, instrument, warn};

/// Enum for specifying which type of set to delete
#[derive(Debug, Clone, Copy)]
pub enum DeleteSetType {
    Qcm,
    OpenQuestion,
    Flashcard,
    TrueOrFalse,
    Keywords,
    OrderPhrase,
    FillBlank,
}

/// Input for deleting a set
#[derive(Debug, Clone)]
pub struct DeleteSetInput {
    pub user_id: String,
    pub set_id: String,
    pub set_type: DeleteSetType,
}

/// Use case for deleting content sets
pub struct DeleteSetUseCase {
    intello_service: Arc<IntelloService>,
}

impl DeleteSetUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self), fields(user_id = %input.user_id, set_id = %input.set_id, set_type = ?input.set_type))]
    pub async fn execute(&self, input: DeleteSetInput) -> AppResult<bool> {
        // Validate inputs
        if input.user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }
        if input.set_id.is_empty() {
            return Err(AppError::validation("set_id", "Set ID is required"));
        }

        let deleted = match input.set_type {
            DeleteSetType::Qcm => {
                self.intello_service.delete_qcm_set(&input.set_id, &input.user_id).await?
            }
            DeleteSetType::OpenQuestion => {
                // Open questions don't have a delete method yet, return false
                warn!("Delete not implemented for open questions");
                false
            }
            DeleteSetType::Flashcard => {
                self.intello_service.delete_flashcard_set(&input.set_id, &input.user_id).await?
            }
            DeleteSetType::TrueOrFalse => {
                self.intello_service.delete_true_false_set(&input.set_id, &input.user_id).await?
            }
            DeleteSetType::Keywords => {
                self.intello_service.delete_keyword_set(&input.set_id, &input.user_id).await?
            }
            DeleteSetType::OrderPhrase => {
                self.intello_service.delete_order_phrase_set(&input.set_id, &input.user_id).await?
            }
            DeleteSetType::FillBlank => {
                self.intello_service.delete_fill_blank_set(&input.set_id, &input.user_id).await?
            }
        };

        if deleted {
            info!("Set deleted successfully");
        } else {
            warn!("Set not found or already deleted");
        }

        Ok(deleted)
    }
}
