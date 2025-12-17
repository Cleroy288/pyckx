//! UserCollection - Domain entity representing a user's collection of a specific type
//!
//! Maps to the `user_collections` table in Supabase.
//! Each user can have one collection per item type (DVD, Book, etc.).

use super::CollectionItemType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User's collection of a specific item type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCollection {
    /// Unique identifier (auto-increment PK)
    pub id: i32,
    /// User ID (UUID from auth.users)
    pub user_id: String,
    /// Type of items in this collection
    pub collection_type: CollectionItemType,
    /// When this collection was created
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl UserCollection {
    /// Create a new UserCollection instance (for testing/mocking)
    pub fn new(
        id: i32,
        user_id: impl Into<String>,
        collection_type: CollectionItemType,
    ) -> Self {
        Self {
            id,
            user_id: user_id.into(),
            collection_type,
            created_at: Utc::now(),
        }
    }

    /// Get the display name for this collection
    pub fn display_name(&self) -> String {
        format!("{} Collection", self.collection_type.display_name())
    }

    /// Check if this collection is for the given item type
    pub fn is_type(&self, item_type: CollectionItemType) -> bool {
        self.collection_type == item_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_collection_new() {
        let collection = UserCollection::new(1, "user-123", CollectionItemType::Dvd);

        assert_eq!(collection.id, 1);
        assert_eq!(collection.user_id, "user-123");
        assert_eq!(collection.collection_type, CollectionItemType::Dvd);
    }

    #[test]
    fn test_display_name() {
        let dvd_collection = UserCollection::new(1, "user-123", CollectionItemType::Dvd);
        let book_collection = UserCollection::new(2, "user-123", CollectionItemType::Book);

        assert_eq!(dvd_collection.display_name(), "DVD Collection");
        assert_eq!(book_collection.display_name(), "Book Collection");
    }

    #[test]
    fn test_is_type() {
        let collection = UserCollection::new(1, "user-123", CollectionItemType::Dvd);

        assert!(collection.is_type(CollectionItemType::Dvd));
        assert!(!collection.is_type(CollectionItemType::Book));
    }
}
