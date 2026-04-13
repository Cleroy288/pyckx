//! DVD Domain - Entities for DVD management
//!
//! This module contains the domain entities for the DVD collection system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// == DVD ENTITY ==

/// DVD entity - represents a DVD in a user's collection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dvd {
    pub id: String,
    pub collection_id: i32,
    pub user_id: String,
    pub name: String,
    pub year: DateTime<Utc>,
    pub realisator: Option<String>,
    pub actors: String,
    pub genre: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Dvd {
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

    pub fn release_year(&self) -> i32 {
        self.year.format("%Y").to_string().parse().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Helper: create a minimal Dvd for testing
    fn test_dvd(actors: &str, year: DateTime<Utc>) -> Dvd {
        let now = Utc::now();
        Dvd {
            id: "dvd-1".to_string(),
            collection_id: 1,
            user_id: "usr-1".to_string(),
            name: "Test Movie".to_string(),
            year,
            realisator: None,
            actors: actors.to_string(),
            genre: None,
            created_at: now,
            updated_at: now,
        }
    }

    // -- actors_vec tests --

    #[test]
    fn test_actors_vec_with_multiple_actors() {
        // arrange
        let dvd = test_dvd("Alice, Bob, Charlie", Utc::now());

        // act
        let actors = dvd.actors_vec();

        // assert
        assert_eq!(
            actors,
            vec!["Alice", "Bob", "Charlie"]
        );
    }

    #[test]
    fn test_actors_vec_with_empty_string() {
        // arrange
        let dvd = test_dvd("", Utc::now());

        // act
        let actors = dvd.actors_vec();

        // assert
        assert!(actors.is_empty());
    }

    #[test]
    fn test_actors_vec_single_actor() {
        // arrange
        let dvd = test_dvd("Alice", Utc::now());

        // act
        let actors = dvd.actors_vec();

        // assert
        assert_eq!(actors, vec!["Alice"]);
    }

    #[test]
    fn test_actors_vec_trims_whitespace() {
        // arrange
        let dvd = test_dvd("  Alice , Bob  ", Utc::now());

        // act
        let actors = dvd.actors_vec();

        // assert
        assert_eq!(actors, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_actors_vec_filters_empty_entries() {
        // arrange
        let dvd = test_dvd("Alice,,Bob,", Utc::now());

        // act
        let actors = dvd.actors_vec();

        // assert
        assert_eq!(actors, vec!["Alice", "Bob"]);
    }

    // -- release_year tests --

    #[test]
    fn test_release_year_returns_correct_year() {
        // arrange
        let year_dt =
            Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let dvd = test_dvd("Actor", year_dt);

        // act
        let year = dvd.release_year();

        // assert
        assert_eq!(year, 2023);
    }

    #[test]
    fn test_release_year_with_different_year() {
        // arrange
        let year_dt =
            Utc.with_ymd_and_hms(1999, 1, 1, 0, 0, 0).unwrap();
        let dvd = test_dvd("Actor", year_dt);

        // act / assert
        assert_eq!(dvd.release_year(), 1999);
    }
}
