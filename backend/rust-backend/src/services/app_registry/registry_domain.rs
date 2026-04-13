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
    pub const fn new(
        id: AppId,
        name: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            description,
        }
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
    pub fn new(
        id: i32,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    // -- AppInstance::new tests --

    #[test]
    fn test_app_instance_new_stores_fields() {
        // arrange / act
        let instance =
            AppInstance::new("test", "Test App", "A test");

        // assert
        assert_eq!(instance.id, "test");
        assert_eq!(instance.name, "Test App");
        assert_eq!(instance.description, "A test");
    }

    // -- App::new tests --

    #[test]
    fn test_app_new_stores_id_and_name() {
        // arrange / act
        let app = App::new(1, "study", None);

        // assert
        assert_eq!(app.id, 1);
        assert_eq!(app.name, "study");
        assert!(app.description.is_none());
    }

    #[test]
    fn test_app_new_with_description() {
        // arrange / act
        let app = App::new(
            2,
            "quiz",
            Some("A quiz app".to_string()),
        );

        // assert
        assert_eq!(app.id, 2);
        assert_eq!(app.name, "quiz");
        assert_eq!(
            app.description.as_deref(),
            Some("A quiz app")
        );
    }

    #[test]
    fn test_app_new_timestamps_are_equal() {
        // arrange / act
        let app = App::new(1, "app", None);

        // assert - created_at and updated_at should be the same
        assert_eq!(app.created_at, app.updated_at);
    }

    // -- UserApp::new tests --

    #[test]
    fn test_user_app_new_stores_fields() {
        // arrange / act
        let user_app = UserApp::new(1, "user-abc", 5);

        // assert
        assert_eq!(user_app.id, 1);
        assert_eq!(user_app.user_id, "user-abc");
        assert_eq!(user_app.app_id, 5);
    }

    #[test]
    fn test_user_app_new_with_string_user_id() {
        // arrange / act
        let user_app =
            UserApp::new(2, "uid-42".to_string(), 3);

        // assert
        assert_eq!(user_app.user_id, "uid-42");
    }
}
