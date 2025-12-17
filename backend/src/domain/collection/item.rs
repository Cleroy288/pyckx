//! CollectionItem - Core trait and types for polymorphic collection items
//!
//! Defines the contract that all collection items (DVD, Book, etc.) must implement.
//! Follows Interface Segregation Principle.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

// == COLLECTION ITEM TYPE ENUM // ==

/// Types of items that can be stored in collections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CollectionItemType {
    /// DVD movies
    Dvd,
    /// Books
    Book,
}

#[allow(dead_code)]
impl CollectionItemType {
    /// Get all available item types
    pub fn all() -> Vec<Self> {
        vec![Self::Dvd, Self::Book]
    }

    /// Get the database table name for this item type
    pub fn table_name(&self) -> &'static str {
        match self {
            Self::Dvd => "dvds",
            Self::Book => "books",
        }
    }

    /// Get the display name for this item type
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Dvd => "DVD",
            Self::Book => "Book",
        }
    }

    /// Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "dvd" => Some(Self::Dvd),
            "book" => Some(Self::Book),
            _ => None,
        }
    }
}

impl fmt::Display for CollectionItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dvd => write!(f, "dvd"),
            Self::Book => write!(f, "book"),
        }
    }
}

// == COLLECTION ITEM TRAIT // ==

/// Core trait that all collection items must implement
///
/// This trait defines the common interface for all items that can be stored
/// in user collections. It follows the Interface Segregation Principle by
/// only requiring the essential methods that all items share.
#[allow(dead_code)]
pub trait CollectionItem: Send + Sync + Clone {
    /// Get the unique identifier of this item
    fn id(&self) -> &str;

    /// Get the name/title of this item
    fn name(&self) -> &str;

    /// Get the type of this item (associated function)
    fn item_type() -> CollectionItemType
    where
        Self: Sized;

    /// Get the user ID who owns this item
    fn user_id(&self) -> &str;

    /// Get the collection ID this item belongs to
    fn collection_id(&self) -> i32;

    /// Get when this item was created
    fn created_at(&self) -> DateTime<Utc>;

    /// Get when this item was last updated
    fn updated_at(&self) -> DateTime<Utc>;

    /// Set the collection ID (used when moving items between collections)
    fn set_collection_id(&mut self, collection_id: i32);

    /// Update the updated_at timestamp
    fn touch(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_item_type_display() {
        assert_eq!(CollectionItemType::Dvd.to_string(), "dvd");
        assert_eq!(CollectionItemType::Book.to_string(), "book");
    }

    #[test]
    fn test_collection_item_type_from_str() {
        assert_eq!(CollectionItemType::from_str("dvd"), Some(CollectionItemType::Dvd));
        assert_eq!(CollectionItemType::from_str("DVD"), Some(CollectionItemType::Dvd));
        assert_eq!(CollectionItemType::from_str("book"), Some(CollectionItemType::Book));
        assert_eq!(CollectionItemType::from_str("BOOK"), Some(CollectionItemType::Book));
        assert_eq!(CollectionItemType::from_str("invalid"), None);
    }

    #[test]
    fn test_collection_item_type_table_name() {
        assert_eq!(CollectionItemType::Dvd.table_name(), "dvds");
        assert_eq!(CollectionItemType::Book.table_name(), "books");
    }

    #[test]
    fn test_collection_item_type_all() {
        let all_types = CollectionItemType::all();
        assert_eq!(all_types.len(), 2);
        assert!(all_types.contains(&CollectionItemType::Dvd));
        assert!(all_types.contains(&CollectionItemType::Book));
    }
}
