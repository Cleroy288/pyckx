//! App Repository Trait - Abstraction for app storage operations

use super::types::{CreateApp, UpdateApp};
use crate::services::app_registry::registry_domain::App as AppEntity;
use crate::shared::AppError;
use async_trait::async_trait;

/// Repository trait for app persistence operations
///
/// Follows Interface Segregation Principle - only app CRUD operations.
#[async_trait]
pub trait AppRepository: Send + Sync {
    /// Get all available apps
    async fn find_all(&self) -> Result<Vec<AppEntity>, AppError>;

    /// Find an app by name
    async fn find_by_name(&self, name: &str) -> Result<AppEntity, AppError>;

    /// Create a new app
    async fn insert(&self, app: &CreateApp) -> Result<AppEntity, AppError>;

    /// Update an existing app
    async fn update(
        &self,
        name: &str,
        update: &UpdateApp,
    ) -> Result<AppEntity, AppError>;

    /// Delete an app by name
    async fn delete(&self, name: &str) -> Result<bool, AppError>;

    /// Check if an app exists
    async fn exists(&self, name: &str) -> Result<bool, AppError>;
}
