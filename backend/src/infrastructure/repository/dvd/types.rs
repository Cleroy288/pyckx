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
