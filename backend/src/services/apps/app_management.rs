//! App management operations (admin)
//!
//! CRUD operations for apps themselves.

use super::AppService;
use crate::domain::AppEntity;
use crate::error::AppResult;
use crate::infrastructure::{CreateApp, UpdateApp};
use tracing::{info, instrument};

impl AppService {
    /// Get all available apps from database
    #[instrument(skip(self))]
    pub async fn get_all_apps(&self) -> AppResult<Vec<AppEntity>> {
        let apps = self.app_repo.find_all().await?;
        info!(count = apps.len(), "Retrieved all apps");
        Ok(apps)
    }

    /// Get an app by name
    #[instrument(skip(self), fields(name = %name))]
    pub async fn get_app(&self, name: &str) -> AppResult<AppEntity> {
        self.app_repo.find_by_name(name).await
    }

    /// Create a new app
    #[instrument(skip(self), fields(name = %name))]
    pub async fn create_app(
        &self,
        name: &str,
        description: Option<String>,
    ) -> AppResult<AppEntity> {
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
    ) -> AppResult<AppEntity> {
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
}
