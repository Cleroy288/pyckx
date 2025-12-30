//! App Registry Service - App management and registry
//!
//! Handles app CRUD operations, user app management, and registry validation.

use crate::services::app_registry::registry_domain::{App, UserApp, AppInstance};
use crate::services::app_registry::error_domain::AppError as AppsError;
use crate::infra::{AppRepository, CreateApp, UpdateApp, UserAppRepository};
use crate::shared::{AppError, AppResult};
use std::fmt;
use std::sync::Arc;
use tracing::{info, instrument, warn};

// == AVAILABLE APPS REGISTRY ==

/// All available apps in the platform
pub const AVAILABLE_APPS: &[AppInstance] = &[
    AppInstance::new(
        "collection",
        "Collection",
        "Create and manage collections of items, bookmarks, and resources",
    ),
    AppInstance::new(
        "intello",
        "Intello",
        "Learn and test your knowledge with QCM, flashcards, and other games",
    ),
];

// == REGISTRY FUNCTIONS ==

/// Check if an app name is valid (exists in the registry)
pub fn is_valid_app(name: &str) -> bool {
    AVAILABLE_APPS.iter().any(|app| app.id == name)
}

// == APP SERVICE ==

/// Service for managing apps and user app lists
pub struct AppService {
    app_repo: Arc<dyn AppRepository>,
    user_app_repo: Arc<dyn UserAppRepository>,
}

impl AppService {
    /// Create a new AppService with the given repositories
    pub fn new(
        app_repo: Arc<dyn AppRepository>,
        user_app_repo: Arc<dyn UserAppRepository>,
    ) -> Self {
        info!("AppService initialized");
        Self {
            app_repo,
            user_app_repo,
        }
    }

    // ============= APP CRUD OPERATIONS =============

    /// Get all available apps from database
    #[instrument(skip(self))]
    pub async fn get_all_apps(&self) -> AppResult<Vec<App>> {
        let apps = self.app_repo.find_all().await?;
        info!(count = apps.len(), "Retrieved all apps");
        Ok(apps)
    }

    /// Get an app by name
    #[instrument(skip(self), fields(name = %name))]
    pub async fn get_app(&self, name: &str) -> AppResult<App> {
        self.app_repo.find_by_name(name).await
    }

    /// Create a new app
    #[instrument(skip(self), fields(name = %name))]
    pub async fn create_app(
        &self,
        name: &str,
        description: Option<String>,
    ) -> AppResult<App> {
        let create = CreateApp::new(name, description);
        let app = self.app_repo.insert(&create).await?;
        info!(app_id = app.id, "App created");
        Ok(app)
    }

    /// Update an existing app
    #[instrument(skip(self), fields(name = %name))]
    pub async fn update_app(
        &self,
        name: &str,
        new_description: Option<String>,
    ) -> AppResult<App> {
        let mut update = UpdateApp::new();
        if let Some(desc) = new_description {
            update = update.with_description(desc);
        }
        let app = self.app_repo.update(name, &update).await?;
        info!(app_id = app.id, "App updated");
        Ok(app)
    }

    /// Delete an app
    #[instrument(skip(self), fields(name = %name))]
    pub async fn delete_app(&self, name: &str) -> AppResult<bool> {
        let deleted = self.app_repo.delete(name).await?;
        if deleted {
            info!(app_name = %name, "App deleted");
        }
        Ok(deleted)
    }

    // ============= USER APP OPERATIONS =============

    /// Get all apps enabled by a user (returns full App objects)
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_apps(&self, user_id: &str) -> AppResult<Vec<App>> {
        let user_apps = self.user_app_repo.find_by_user(user_id).await?;

        // Fetch full app details for each user app by ID
        let all_apps = self.app_repo.find_all().await?;
        let apps: Vec<App> = user_apps
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
        if !is_valid_app(app_name) {
            warn!(app_name = %app_name, "Attempted to add invalid app");
            return Err(AppError::App(AppsError::NotFound(app_name.to_string())));
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
        if !is_valid_app(app_name) {
            warn!(app_name = %app_name, "Attempted to remove invalid app");
            return Err(AppError::App(AppsError::NotFound(app_name.to_string())));
        }

        // Get app from database to get its ID
        let app = self.app_repo.find_by_name(app_name).await?;

        let removed = self.user_app_repo.remove_app(user_id, app.id).await?;
        if !removed {
            return Err(AppError::App(AppsError::UserAppNotFound {
                user_id: user_id.to_string(),
                app_name: app_name.to_string(),
            }));
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

impl fmt::Debug for AppService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppService").finish()
    }
}

impl fmt::Display for AppService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppService")
    }
}
