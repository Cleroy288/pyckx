//! UserApp Repository Trait - Abstraction for user app list operations

use crate::domain::UserApp;
use crate::error::AppError;
use async_trait::async_trait;

/// Repository trait for user app persistence operations
///
/// Follows Interface Segregation Principle - only user app operations.
#[async_trait]
pub trait UserAppRepository: Send + Sync {
    /// Get all apps enabled by a user
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<UserApp>, AppError>;

    /// Add an app to user's list by app_id
    async fn add_app(&self, user_id: &str, app_id: i32) -> Result<UserApp, AppError>;

    /// Remove an app from user's list by app_id
    async fn remove_app(&self, user_id: &str, app_id: i32) -> Result<bool, AppError>;

    /// Check if user has a specific app enabled by app_id
    async fn has_app(&self, user_id: &str, app_id: i32) -> Result<bool, AppError>;
}
