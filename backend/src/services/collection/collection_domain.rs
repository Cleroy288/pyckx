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
}

impl std::str::FromStr for CollectionItemType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dvd" => Ok(Self::Dvd),
            "book" => Ok(Self::Book),
            _ => Err(format!(
                "Unknown collection type: {}", s
            )),
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
        format!(
            "{} Collection",
            self.collection_type.display_name()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- CollectionItemType::display_name tests --

    #[test]
    fn test_display_name_dvd_returns_uppercase() {
        // arrange
        let item = CollectionItemType::Dvd;

        // act / assert
        assert_eq!(item.display_name(), "DVD");
    }

    #[test]
    fn test_display_name_book_returns_title_case() {
        // arrange
        let item = CollectionItemType::Book;

        // act / assert
        assert_eq!(item.display_name(), "Book");
    }

    // -- CollectionItemType::from_str tests --

    #[test]
    fn test_from_str_valid_cases() {
        let cases = vec![
            ("dvd", Some(CollectionItemType::Dvd)),
            ("DVD", Some(CollectionItemType::Dvd)),
            ("Dvd", Some(CollectionItemType::Dvd)),
            ("book", Some(CollectionItemType::Book)),
            ("BOOK", Some(CollectionItemType::Book)),
            ("unknown", None),
            ("", None),
        ];
        for (input, expected) in cases {
            assert_eq!(
                input.parse::<CollectionItemType>().ok(),
                expected,
                "from_str({:?})",
                input
            );
        }
    }

    // -- CollectionItemType Display --

    #[test]
    fn test_item_type_display_dvd() {
        assert_eq!(
            CollectionItemType::Dvd.to_string(),
            "dvd"
        );
    }

    #[test]
    fn test_item_type_display_book() {
        assert_eq!(
            CollectionItemType::Book.to_string(),
            "book"
        );
    }

    // -- UserCollection::display_name --

    #[test]
    fn test_user_collection_display_name_dvd() {
        // arrange
        let collection = UserCollection {
            id: 1,
            user_id: "usr-1".to_string(),
            collection_type: CollectionItemType::Dvd,
            created_at: Utc::now(),
        };

        // act / assert
        assert_eq!(collection.display_name(), "DVD Collection");
    }

    #[test]
    fn test_user_collection_display_name_book() {
        // arrange
        let collection = UserCollection {
            id: 2,
            user_id: "usr-2".to_string(),
            collection_type: CollectionItemType::Book,
            created_at: Utc::now(),
        };

        // act / assert
        assert_eq!(
            collection.display_name(),
            "Book Collection"
        );
    }
}
