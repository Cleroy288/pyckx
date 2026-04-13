//! DVD repository types - DTOs for DVD operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// CREATE DVD DTO
// ============================================================================

/// Data transfer object for creating a new DVD
#[derive(Debug, Clone, Serialize)]
pub struct CreateDvd {
    pub name: String,
    pub year: DateTime<Utc>,
    pub realisator: Option<String>,
    pub actors: String,
    pub genre: Option<String>,
    pub user_id: String,
    pub collection_id: i32,
}

impl CreateDvd {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        year: DateTime<Utc>,
        realisator: Option<String>,
        actors: Vec<String>,
        genre: Option<String>,
        user_id: impl Into<String>,
        collection_id: i32,
    ) -> Self {
        Self {
            name: name.into(),
            year,
            realisator,
            actors: actors.join(", "),
            genre,
            user_id: user_id.into(),
            collection_id,
        }
    }

    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub fn new_legacy(
        name: impl Into<String>,
        year: DateTime<Utc>,
        realisator: Option<String>,
        actors: Vec<String>,
        genre: Option<String>,
        user_id: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            year,
            realisator,
            actors: actors.join(", "),
            genre,
            user_id: user_id.into(),
            collection_id: 0,
        }
    }
}

// ============================================================================
// UPDATE DVD DTO
// ============================================================================

/// Data transfer object for updating a DVD
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDvd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realisator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
}

impl UpdateDvd {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_genre(mut self, genre: impl Into<String>) -> Self {
        self.genre = Some(genre.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_actors(mut self, actors: Vec<String>) -> Self {
        self.actors = Some(actors.join(", "));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};

    /// Fixed test date for deterministic tests
    fn test_date() -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    }

    // -- CreateDvd::new --

    #[test]
    fn test_create_dvd_new_joins_actors() {
        // arrange
        let actors = vec!["Leo".into(), "Tom".into()];

        // act
        let dvd = CreateDvd::new(
            "Inception", test_date(), None,
            actors, None, "user-1", 1,
        );

        // assert
        assert_eq!(dvd.actors, "Leo, Tom");
    }

    #[test]
    fn test_create_dvd_new_maps_name() {
        // arrange / act
        let dvd = CreateDvd::new(
            "Matrix", test_date(), None,
            vec![], None, "user-1", 1,
        );

        // assert
        assert_eq!(dvd.name, "Matrix");
    }

    #[test]
    fn test_create_dvd_new_maps_user_id() {
        // arrange / act
        let dvd = CreateDvd::new(
            "X", test_date(), None,
            vec![], None, "user-42", 5,
        );

        // assert
        assert_eq!(dvd.user_id, "user-42");
        assert_eq!(dvd.collection_id, 5);
    }

    #[test]
    fn test_create_dvd_new_empty_actors() {
        // arrange / act
        let dvd = CreateDvd::new(
            "X", test_date(), None,
            vec![], None, "u", 1,
        );

        // assert
        assert_eq!(dvd.actors, "");
    }

    // -- CreateDvd::new_legacy --

    #[test]
    fn test_create_dvd_legacy_collection_id_zero() {
        // arrange / act
        let dvd = CreateDvd::new_legacy(
            "Old", test_date(), None,
            vec!["A".into()], None, "u",
        );

        // assert
        assert_eq!(dvd.collection_id, 0);
        assert_eq!(dvd.actors, "A");
    }

    // -- UpdateDvd builder --

    #[test]
    fn test_update_dvd_new_all_none() {
        // arrange / act
        let upd = UpdateDvd::new();

        // assert
        assert!(upd.name.is_none());
        assert!(upd.year.is_none());
        assert!(upd.actors.is_none());
        assert!(upd.genre.is_none());
    }

    #[test]
    fn test_update_dvd_with_name() {
        // arrange / act
        let upd = UpdateDvd::new().with_name("New Name");

        // assert
        assert_eq!(upd.name, Some("New Name".into()));
    }

    #[test]
    fn test_update_dvd_with_genre() {
        // arrange / act
        let upd = UpdateDvd::new().with_genre("Horror");

        // assert
        assert_eq!(upd.genre, Some("Horror".into()));
    }

    #[test]
    fn test_update_dvd_with_actors_joins() {
        // arrange
        let actors = vec!["A".into(), "B".into(), "C".into()];

        // act
        let upd = UpdateDvd::new().with_actors(actors);

        // assert
        assert_eq!(upd.actors, Some("A, B, C".into()));
    }

    #[test]
    fn test_update_dvd_chaining() {
        // arrange / act
        let upd = UpdateDvd::new()
            .with_name("Film")
            .with_genre("Drama");

        // assert
        assert_eq!(upd.name, Some("Film".into()));
        assert_eq!(upd.genre, Some("Drama".into()));
    }
}
