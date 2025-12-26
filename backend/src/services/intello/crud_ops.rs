//! Generic CRUD operations for Intello game sets
//!
//! This module provides generic helper functions for common CRUD operations
//! that are shared across all game set types (Keywords, Flashcards, etc.).
//!
//! # Usage
//!
//! Instead of duplicating validation and logging in each `*_ops.rs` file,
//! use these generic functions:
//!
//! ```ignore
//! use crate::services::intello::crud_ops;
//!
//! // In keywords_ops.rs:
//! pub async fn get_user_keyword_sets(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError> {
//!     crud_ops::get_user_sets(self.keywords_repo.as_ref(), user_id, "keywords").await
//! }
//! ```

use crate::error::IntelloError;
use crate::infrastructure::GameSetRepository;
use tracing::{info, warn};

/// Validate user ID is not empty
pub fn validate_user_id(user_id: &str) -> Result<(), IntelloError> {
    if user_id.trim().is_empty() {
        return Err(IntelloError::validation("user_id", "User ID cannot be empty"));
    }
    Ok(())
}

/// Validate set ID is not empty
pub fn validate_set_id(set_id: &str) -> Result<(), IntelloError> {
    if set_id.trim().is_empty() {
        return Err(IntelloError::validation("set_id", "Set ID cannot be empty"));
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

/// Get a specific set by ID (generic implementation)
///
/// # Arguments
/// * `repo` - The repository implementing `GameSetRepository<T>`
/// * `set_id` - The set's ID
/// * `user_id` - The user's ID (for ownership verification)
#[allow(dead_code)]
pub async fn get_set<T: Clone + Send + Sync>(
    repo: &dyn GameSetRepository<T>,
    set_id: &str,
    user_id: &str,
) -> Result<Option<T>, IntelloError> {
    validate_user_id(user_id)?;
    validate_set_id(set_id)?;
    repo.find_by_id(set_id, user_id).await
}

/// Delete a set with ownership verification (generic implementation)
///
/// # Arguments
/// * `repo` - The repository implementing `GameSetRepository<T>`
/// * `set_id` - The set's ID
/// * `user_id` - The user's ID (for ownership verification)
/// * `game_type` - Name of the game type for logging (e.g., "keywords", "flashcards")
///
/// # Returns
/// * `Ok(true)` - Set was found and deleted
/// * `Ok(false)` - Set not found or user doesn't own it
#[allow(dead_code)]
pub async fn delete_set<T: Clone + Send + Sync>(
    repo: &dyn GameSetRepository<T>,
    set_id: &str,
    user_id: &str,
    game_type: &str,
) -> Result<bool, IntelloError> {
    validate_user_id(user_id)?;
    validate_set_id(set_id)?;

    // Verify ownership
    let existing = repo.find_by_id(set_id, user_id).await?;
    if existing.is_none() {
        warn!(set_id, game_type, "Set not found or user doesn't own it");
        return Ok(false);
    }

    let deleted = repo.delete(set_id, user_id).await?;
    if deleted {
        info!(set_id, game_type, "Set deleted");
    }
    Ok(deleted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_id_empty() {
        assert!(validate_user_id("").is_err());
        assert!(validate_user_id("   ").is_err());
    }

    #[test]
    fn test_validate_user_id_valid() {
        assert!(validate_user_id("user-123").is_ok());
    }

    #[test]
    fn test_validate_set_id_empty() {
        assert!(validate_set_id("").is_err());
        assert!(validate_set_id("   ").is_err());
    }

    #[test]
    fn test_validate_set_id_valid() {
        assert!(validate_set_id("set-456").is_ok());
    }
}
