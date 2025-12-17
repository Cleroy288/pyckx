//! UserApp - Domain entity representing a user's enabled application
//!
//! Maps to the `user_apps` table in Supabase.
//! Uses app_id (INTEGER FK) for proper normalization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

    #[test]
    fn test_user_app_new() {
        let user_app = UserApp::new(1, "user-123", 5);

        assert_eq!(user_app.id, 1);
        assert_eq!(user_app.user_id, "user-123");
        assert_eq!(user_app.app_id, 5);
    }
}
