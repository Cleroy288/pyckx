//! Collection Domain - Entities for user collections
//!
//! This module contains the core domain entities for the collection system:
//! - `CollectionItemType` enum - Types of items (DVD, etc.)
//! - `UserCollection` - User's collection of a specific type

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

// == COLLECTION ITEM TYPE ENUM ==

/// Types of items that can be stored in collections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CollectionItemType {
    /// DVD movies
    Dvd,
    /// Books
    Book,
}

impl CollectionItemType {
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

// == USER COLLECTION ENTITY ==

/// User's collection of a specific item type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCollection {
    pub id: i32,
    pub user_id: String,
    pub collection_type: CollectionItemType,
    pub created_at: DateTime<Utc>,
}

impl UserCollection {
    pub fn display_name(&self) -> String {
        format!("{} Collection", self.collection_type.display_name())
    }
}
