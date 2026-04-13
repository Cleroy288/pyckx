//! Collection Repository Trait - Abstraction for user collection operations

use crate::services::collection::collection_domain::{
    CollectionItemType, UserCollection,
};
use crate::services::collection::error_domain::CollectionError;
use async_trait::async_trait;

/// Repository trait for user collection persistence operations
///
/// Follows Interface Segregation Principle - only collection management operations.
#[async_trait]
pub trait CollectionRepository: Send + Sync {
    /// Create a new collection for a user and item type
    async fn create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError>;

    /// Delete a collection and all its items
    async fn delete(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError>;

    /// Get all collections for a user
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserCollection>, CollectionError>;

    /// Find a specific collection by user and type
    async fn find_by_type(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<Option<UserCollection>, CollectionError>;

    /// Get or create a collection (idempotent operation)
    async fn get_or_create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError>;

    /// Check if a collection exists
    #[allow(dead_code)]
    async fn exists(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError>;
}
