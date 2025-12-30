//! Generic CRUD operations for Intello game sets
//!
//! This module provides generic helper functions for common CRUD operations
//! that are shared across all game set types (Keywords, Flashcards, etc.).
//!
//! # Usage
//!
//! Instead of duplicating validation and logging in each `*_service.rs` file,
//! use these generic functions:
//!
//! ```ignore
//! use crate::services::intello::crud_service;
//!
//! // In keywords_service.rs:
//! pub async fn get_user_keyword_sets(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError> {
//!     crud_service::get_user_sets(self.keywords_repo.as_ref(), user_id, "keywords").await
//! }
//! ```

use crate::services::intello::error_domain::IntelloError;
use crate::infra::GameSetRepository;
use tracing::info;

/// Validate user ID is not empty
pub fn validate_user_id(user_id: &str) -> Result<(), IntelloError> {
    if user_id.trim().is_empty() {
        return Err(IntelloError::validation("user_id", "User ID cannot be empty"));
    }
    Ok(())
}

/// Get all sets for a user (generic implementation)
///
/// # Arguments
/// * `repo` - The repository implementing `GameSetRepository<T>`
/// * `user_id` - The user's ID
/// * `game_type` - Name of the game type for logging (e.g., "keywords", "flashcards")
pub async fn get_user_sets<T: Clone + Send + Sync>(
    repo: &dyn GameSetRepository<T>,
    user_id: &str,
    game_type: &str,
) -> Result<Vec<T>, IntelloError> {
    validate_user_id(user_id)?;
    let sets = repo.find_by_user(user_id).await?;
    info!(count = sets.len(), game_type, "Retrieved user sets");
    Ok(sets)
}


