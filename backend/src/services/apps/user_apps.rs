//! User app management operations
//!
//! Operations for managing which apps a user has enabled.

use super::AppService;
use crate::apps::registry;
use crate::domain::{AppEntity, UserApp};
use crate::error::{AppError, AppResult};
use tracing::{info, instrument, warn};

impl AppService {
    /// Get all apps enabled by a user (returns full App objects)
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_apps(&self, user_id: &str) -> AppResult<Vec<AppEntity>> {
        let user_apps = self.user_app_repo.find_by_user(user_id).await?;

        // Fetch full app details for each user app by ID
        let all_apps = self.app_repo.find_all().await?;
        let apps: Vec<AppEntity> = user_apps
            .iter()
            .filter_map(|ua| all_apps.iter().find(|a| a.id == ua.app_id).cloned())
            .collect();

        info!(count = apps.len(), "Retrieved user apps");
        Ok(apps)
    }

    /// Add an app to user's list by app_name (looks up app_id internally)
    ///
    /// Validates that the app exists in the registry before adding.
    #[instrument(skip(self), fields(user_id = %user_id, app_name = %app_name))]
    pub async fn add_user_app(&self, user_id: &str, app_name: &str) -> AppResult<UserApp> {
        // First validate against the registry (source of truth)
        if !registry::is_valid_app(app_name) {
            warn!(app_name = %app_name, "Attempted to add invalid app");
            return Err(AppError::AppNotFound(app_name.to_string()));
        }

        // Get app from database to get its ID
        let app = self.app_repo.find_by_name(app_name).await?;

        let user_app = self.user_app_repo.add_app(user_id, app.id).await?;
        info!(user_app_id = user_app.id, app_id = app.id, "User app added");
        Ok(user_app)
    }

    /// Remove an app from user's list by app_name (looks up app_id internally)
    ///
    /// Validates that the app exists in the registry before removing.
    #[instrument(skip(self), fields(user_id = %user_id, app_name = %app_name))]
    pub async fn remove_user_app(&self, user_id: &str, app_name: &str) -> AppResult<bool> {
        // First validate against the registry (source of truth)
        if !registry::is_valid_app(app_name) {
            warn!(app_name = %app_name, "Attempted to remove invalid app");
            return Err(AppError::AppNotFound(app_name.to_string()));
        }

        // Get app from database to get its ID
        let app = self.app_repo.find_by_name(app_name).await?;

        let removed = self.user_app_repo.remove_app(user_id, app.id).await?;
        if !removed {
            return Err(AppError::UserAppNotFound {
                user_id: user_id.to_string(),
                app_name: app_name.to_string(),
            });
        }
        info!(user_id = %user_id, app_id = app.id, "User app removed");
        Ok(true)
    }

    /// Check if user has a specific app by app_name
    #[allow(dead_code)]
    #[instrument(skip(self), fields(user_id = %user_id, app_name = %app_name))]
    pub async fn user_has_app(&self, user_id: &str, app_name: &str) -> AppResult<bool> {
        let app = self.app_repo.find_by_name(app_name).await?;
        self.user_app_repo.has_app(user_id, app.id).await
    }
}
