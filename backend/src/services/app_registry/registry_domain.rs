//! Apps domain module - Core entities for app management
//!
//! Contains domain entities for:
//! - `App` - Available application in the platform
//! - `UserApp` - User's enabled application
//! - `AppInstance` - App module trait and metadata

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// === AppInstance ===

/// Unique identifier for an app
pub type AppId = &'static str;

/// Metadata for an app instance
#[derive(Debug, Clone)]
pub struct AppInstance {
    #[allow(dead_code)]
    pub id: AppId,
    pub name: &'static str,
    #[allow(dead_code)]
    pub description: &'static str,
}

impl AppInstance {
    /// Create a new app instance
    pub const fn new(id: AppId, name: &'static str, description: &'static str) -> Self {
        Self { id, name, description }
    }
}

/// Trait for app modules - implement this for each app
pub trait AppModule: Send + Sync {
    /// Get app metadata
    fn info(&self) -> &AppInstance;
    
    /// Get the app ID
    #[allow(dead_code)]
    fn id(&self) -> AppId {
        self.info().id
    }
    
    /// Get the app name
    fn name(&self) -> &'static str {
        self.info().name
    }
}

// === App ===

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

// === UserApp ===

/// User's enabled application
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserApp {
    /// Unique identifier (auto-increment PK)
    pub id: i32,
    /// User ID (UUID from auth.users)
    pub user_id: String,
    /// App ID (FK to apps.id)
    pub app_id: i32,
    /// When the user added this app
    pub created_at: DateTime<Utc>,
}

impl UserApp {
    /// Create a new UserApp instance (for testing/mocking)
    #[allow(dead_code)]
    pub fn new(id: i32, user_id: impl Into<String>, app_id: i32) -> Self {
        Self {
            id,
            user_id: user_id.into(),
            app_id,
            created_at: Utc::now(),
        }
    }
}


