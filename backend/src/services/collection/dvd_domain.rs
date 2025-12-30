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
