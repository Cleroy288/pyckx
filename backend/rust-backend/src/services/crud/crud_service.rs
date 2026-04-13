//! Generic CRUD operations for Study game sets
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
//! use crate::services::crud_service;
//!
//! // In keywords_service.rs:
//! pub async fn get_user_keyword_sets(&self, user_id: &str) -> Result<Vec<KeywordSet>, StudyError> {
//!     crud_service::get_user_sets(self.keywords_repo.as_ref(), user_id, "keywords").await
//! }
//! ```

use crate::infra::GameSetRepository;
use crate::services::error_domain::StudyError;
use tracing::info;

// ** validate_user_id **
// ==> Validates that a user ID is not empty or whitespace-only
//
// @ user_id : The user ID to validate
// @ returns : Ok(()) if valid
// @ errors : ValidationFailed if user_id is empty or whitespace-only
pub fn validate_user_id(user_id: &str) -> Result<(), StudyError> {
    // Step 1: Check if user ID is empty or whitespace-only
    if user_id.trim().is_empty() {
        return Err(StudyError::validation(
            "user_id",
            "User ID cannot be empty",
        ));
    }

    // Step 2: Return success
    Ok(())
}

// ** get_user_sets **
// ==> Retrieves all game sets for a specific user from repository
//
// @ repo : The repository implementing GameSetRepository<T>
// @ user_id : The user's ID
// @ game_type : Name of game type for logging (e.g., "keywords")
// @ returns : Vec<T> containing all sets for the user
// @ errors : ValidationFailed if user_id invalid, StorageError if query fails
pub async fn get_user_sets<T: Clone + Send + Sync>(
    repo: &dyn GameSetRepository<T>,
    user_id: &str,
    game_type: &str,
) -> Result<Vec<T>, StudyError> {
    // Step 1: Validate user ID
    validate_user_id(user_id)?;

    // Step 2: Fetch sets from repository
    let sets = repo.find_by_user(user_id).await?;

    // Step 3: Log retrieval operation
    info!(count = sets.len(), game_type, "Retrieved user sets");

    // Step 4: Return the results
    Ok(sets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_id_valid_returns_ok() {
        // arrange
        let user_id = "user-abc-123";

        // act
        let result = validate_user_id(user_id);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_user_id_empty_returns_error() {
        // arrange
        let user_id = "";

        // act
        let result = validate_user_id(user_id);

        // assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code(), "STUDY_VALIDATION_FAILED");
    }

    #[test]
    fn test_validate_user_id_whitespace_returns_error() {
        // arrange
        let user_id = "   ";

        // act
        let result = validate_user_id(user_id);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_user_id_tab_only_returns_error() {
        // arrange
        let user_id = "\t\n";

        // act
        let result = validate_user_id(user_id);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_user_id_with_spaces_around_is_ok() {
        // arrange - has non-whitespace content
        let user_id = "  abc  ";

        // act
        let result = validate_user_id(user_id);

        // assert
        assert!(result.is_ok());
    }
}
