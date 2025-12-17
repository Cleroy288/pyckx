//! App - Domain entity representing an available application
//!
//! Maps to the `apps` table in Supabase.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Available application in the platform
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct App {
    /// Unique identifier (auto-increment)
    pub id: i32,
    /// App name (unique, used as identifier)
    pub name: String,
    /// App description
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl App {
    /// Create a new App instance (for testing/mocking)
    #[allow(dead_code)]
    pub fn new(id: i32, name: impl Into<String>, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            name: name.into(),
            description,
            created_at: now,
            updated_at: now,
        }
    }
}


