//! DVD - Domain entity representing a DVD in a user's collection
//!
//! Maps to the `dvds` table in Supabase.
//! Implements the CollectionItem trait for polymorphic handling.

use super::{CollectionItem, CollectionItemType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// DVD entity - represents a DVD in a user's collection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dvd {
    /// Unique identifier (UUID)
    pub id: String,
    /// Collection ID this DVD belongs to
    pub collection_id: i32,
    /// User ID who owns this DVD
    pub user_id: String,
    /// DVD title
    pub name: String,
    /// Release year
    pub year: DateTime<Utc>,
    /// Director (optional)
    pub realisator: Option<String>,
    /// Actors (comma-separated string)
    pub actors: String,
    /// Genre (optional)
    pub genre: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl Dvd {
    /// Create a new DVD instance
    pub fn new(
        name: impl Into<String>,
        collection_id: i32,
        user_id: impl Into<String>,
        year: DateTime<Utc>,
        realisator: Option<String>,
        actors: Vec<String>,
        genre: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            collection_id,
            user_id: user_id.into(),
            name: name.into(),
            year,
            realisator,
            actors: actors.join(", "),
            genre,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a DVD from database row data (without collection_id for backward compat)
    #[allow(clippy::too_many_arguments)]
    pub fn from_db_legacy(
        id: String,
        name: String,
        year: DateTime<Utc>,
        realisator: Option<String>,
        actors: String,
        genre: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        user_id: String,
    ) -> Self {
        Self {
            id,
            collection_id: 0, // Will be set by repository
            user_id,
            name,
            year,
            realisator,
            actors,
            genre,
            created_at,
            updated_at,
        }
    }

    /// Get actors as a vector
    pub fn actors_vec(&self) -> Vec<String> {
        if self.actors.is_empty() {
            Vec::new()
        } else {
            self.actors
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    }

    /// Get the release year as i32
    pub fn release_year(&self) -> i32 {
        self.year.format("%Y").to_string().parse().unwrap_or(0)
    }
}

// Implement CollectionItem trait for polymorphic handling
impl CollectionItem for Dvd {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn item_type() -> CollectionItemType {
        CollectionItemType::Dvd
    }

    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn collection_id(&self) -> i32 {
        self.collection_id
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn set_collection_id(&mut self, collection_id: i32) {
        self.collection_id = collection_id;
        self.updated_at = Utc::now();
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_dvd_new() {
        let year = Utc.with_ymd_and_hms(2010, 7, 16, 0, 0, 0).unwrap();
        let dvd = Dvd::new(
            "The Matrix",
            1,
            "user-123",
            year,
            Some("Wachowski Sisters".to_string()),
            vec!["Keanu Reeves".to_string(), "Laurence Fishburne".to_string()],
            Some("Sci-Fi".to_string()),
        );

        assert_eq!(dvd.name, "The Matrix");
        assert_eq!(dvd.collection_id, 1);
        assert_eq!(dvd.user_id, "user-123");
        assert_eq!(dvd.realisator, Some("Wachowski Sisters".to_string()));
        assert_eq!(dvd.actors, "Keanu Reeves, Laurence Fishburne");
        assert_eq!(dvd.genre, Some("Sci-Fi".to_string()));
        assert!(!dvd.id.is_empty());
    }

    #[test]
    fn test_collection_item_trait() {
        let year = Utc::now();
        let dvd = Dvd::new(
            "Test Movie",
            1,
            "user-123",
            year,
            None,
            vec![],
            None,
        );

        assert_eq!(dvd.name(), "Test Movie");
        assert_eq!(Dvd::item_type(), CollectionItemType::Dvd);
        assert_eq!(dvd.user_id(), "user-123");
        assert_eq!(dvd.collection_id(), 1);
    }

    #[test]
    fn test_actors_vec() {
        let year = Utc::now();
        let dvd = Dvd::new(
            "Test",
            1,
            "user-123",
            year,
            None,
            vec!["Actor1".to_string(), "Actor2".to_string()],
            None,
        );

        let actors = dvd.actors_vec();
        assert_eq!(actors.len(), 2);
        assert_eq!(actors[0], "Actor1");
        assert_eq!(actors[1], "Actor2");
    }

    #[test]
    fn test_actors_vec_empty() {
        let year = Utc::now();
        let dvd = Dvd::new("Test", 1, "user-123", year, None, vec![], None);

        let actors = dvd.actors_vec();
        assert!(actors.is_empty());
    }

    #[test]
    fn test_release_year() {
        let year = Utc.with_ymd_and_hms(2010, 7, 16, 0, 0, 0).unwrap();
        let dvd = Dvd::new("Test", 1, "user-123", year, None, vec![], None);

        assert_eq!(dvd.release_year(), 2010);
    }

    #[test]
    fn test_set_collection_id() {
        let year = Utc::now();
        let mut dvd = Dvd::new("Test", 1, "user-123", year, None, vec![], None);
        let original_updated = dvd.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));
        dvd.set_collection_id(2);

        assert_eq!(dvd.collection_id, 2);
        assert!(dvd.updated_at > original_updated);
    }
}
