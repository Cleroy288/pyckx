//! App Registry Service - App management and registry
//!
//! Handles app CRUD operations, user app management, and registry validation.

use crate::infra::{AppRepository, CreateApp, UpdateApp, UserAppRepository};
use crate::services::app_registry::error_domain::AppError as AppsError;
use crate::services::app_registry::registry_domain::{
    App, AppInstance, UserApp,
};
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
        "study",
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
            .filter_map(|ua| {
                all_apps.iter().find(|a| a.id == ua.app_id).cloned()
            })
            .collect();

        info!(count = apps.len(), "Retrieved user apps");
        Ok(apps)
    }

    /// Add an app to user's list by app_name (looks up app_id internally)
    ///
    /// Validates that the app exists in the registry before adding.
    #[instrument(skip(self), fields(user_id = %user_id, app_name = %app_name))]
    pub async fn add_user_app(
        &self,
        user_id: &str,
        app_name: &str,
    ) -> AppResult<UserApp> {
        // First validate against the registry (source of truth)
        if !is_valid_app(app_name) {
            warn!(app_name = %app_name, "Attempted to add invalid app");
            return Err(AppError::App(AppsError::NotFound(
                app_name.to_string(),
            )));
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
    pub async fn remove_user_app(
        &self,
        user_id: &str,
        app_name: &str,
    ) -> AppResult<bool> {
        // First validate against the registry (source of truth)
        if !is_valid_app(app_name) {
            warn!(app_name = %app_name, "Attempted to remove invalid app");
            return Err(AppError::App(AppsError::NotFound(
                app_name.to_string(),
            )));
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
    pub async fn user_has_app(
        &self,
        user_id: &str,
        app_name: &str,
    ) -> AppResult<bool> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::infra::{CreateApp, UpdateApp};
    use std::sync::Mutex;

    // -- Stub repositories --

    /// In-memory stub for AppRepository
    struct StubAppRepo {
        apps: Mutex<Vec<App>>,
    }

    impl StubAppRepo {
        fn new(apps: Vec<App>) -> Self {
            Self { apps: Mutex::new(apps) }
        }
    }

    #[async_trait]
    impl AppRepository for StubAppRepo {
        async fn find_all(
            &self,
        ) -> Result<Vec<App>, AppError> {
            Ok(self.apps.lock().unwrap().clone())
        }

        async fn find_by_name(
            &self,
            name: &str,
        ) -> Result<App, AppError> {
            self.apps
                .lock()
                .unwrap()
                .iter()
                .find(|a| a.name == name)
                .cloned()
                .ok_or_else(|| {
                    AppError::App(AppsError::NotFound(
                        name.to_string(),
                    ))
                })
        }

        async fn insert(
            &self,
            app: &CreateApp,
        ) -> Result<App, AppError> {
            let mut apps = self.apps.lock().unwrap();
            let id = apps.len() as i32 + 1;
            let new = App::new(
                id,
                &app.name,
                app.description.clone(),
            );
            apps.push(new.clone());
            Ok(new)
        }

        async fn update(
            &self,
            name: &str,
            update: &UpdateApp,
        ) -> Result<App, AppError> {
            let mut apps = self.apps.lock().unwrap();
            let app = apps
                .iter_mut()
                .find(|a| a.name == name)
                .ok_or_else(|| {
                    AppError::App(AppsError::NotFound(
                        name.to_string(),
                    ))
                })?;
            if let Some(desc) = &update.description {
                app.description = Some(desc.clone());
            }
            Ok(app.clone())
        }

        async fn delete(
            &self,
            name: &str,
        ) -> Result<bool, AppError> {
            let mut apps = self.apps.lock().unwrap();
            let len = apps.len();
            apps.retain(|a| a.name != name);
            Ok(apps.len() < len)
        }

        async fn exists(
            &self,
            name: &str,
        ) -> Result<bool, AppError> {
            Ok(self
                .apps
                .lock()
                .unwrap()
                .iter()
                .any(|a| a.name == name))
        }
    }

    /// In-memory stub for UserAppRepository
    struct StubUserAppRepo {
        entries: Mutex<Vec<UserApp>>,
    }

    impl StubUserAppRepo {
        fn new(entries: Vec<UserApp>) -> Self {
            Self {
                entries: Mutex::new(entries),
            }
        }
    }

    #[async_trait]
    impl UserAppRepository for StubUserAppRepo {
        async fn find_by_user(
            &self,
            user_id: &str,
        ) -> Result<Vec<UserApp>, AppError> {
            let all = self.entries.lock().unwrap();
            Ok(all
                .iter()
                .filter(|ua| ua.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn add_app(
            &self,
            user_id: &str,
            app_id: i32,
        ) -> Result<UserApp, AppError> {
            let mut all = self.entries.lock().unwrap();
            let id = all.len() as i32 + 1;
            let ua = UserApp::new(id, user_id, app_id);
            all.push(ua.clone());
            Ok(ua)
        }

        async fn remove_app(
            &self,
            user_id: &str,
            app_id: i32,
        ) -> Result<bool, AppError> {
            let mut all = self.entries.lock().unwrap();
            let len = all.len();
            all.retain(|ua| {
                !(ua.user_id == user_id
                    && ua.app_id == app_id)
            });
            Ok(all.len() < len)
        }

        async fn has_app(
            &self,
            user_id: &str,
            app_id: i32,
        ) -> Result<bool, AppError> {
            let all = self.entries.lock().unwrap();
            Ok(all.iter().any(|ua| {
                ua.user_id == user_id
                    && ua.app_id == app_id
            }))
        }
    }

    /// Build an AppService with given stubs
    fn make_service(
        apps: Vec<App>,
        user_apps: Vec<UserApp>,
    ) -> AppService {
        AppService::new(
            Arc::new(StubAppRepo::new(apps)),
            Arc::new(StubUserAppRepo::new(user_apps)),
        )
    }

    /// Seed apps matching AVAILABLE_APPS registry
    fn seed_apps() -> Vec<App> {
        vec![
            App::new(1, "collection", None),
            App::new(2, "study", None),
        ]
    }

    // -- is_valid_app tests --

    #[test]
    fn test_is_valid_app_known_name_returns_true() {
        assert!(is_valid_app("collection"));
    }

    #[test]
    fn test_is_valid_app_unknown_name_returns_false() {
        assert!(!is_valid_app("unknown"));
    }

    #[test]
    fn test_is_valid_app_empty_string_returns_false() {
        assert!(!is_valid_app(""));
    }

    // -- get_all_apps tests --

    #[tokio::test]
    async fn test_get_all_apps_returns_all() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let apps = svc.get_all_apps().await.unwrap();

        // assert
        assert_eq!(apps.len(), 2);
    }

    #[tokio::test]
    async fn test_get_all_apps_empty_returns_empty() {
        // arrange
        let svc = make_service(vec![], vec![]);

        // act
        let apps = svc.get_all_apps().await.unwrap();

        // assert
        assert!(apps.is_empty());
    }

    // -- get_app tests --

    #[tokio::test]
    async fn test_get_app_found_returns_app() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let app = svc.get_app("collection").await.unwrap();

        // assert
        assert_eq!(app.name, "collection");
    }

    #[tokio::test]
    async fn test_get_app_not_found_returns_error() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let result = svc.get_app("unknown").await;

        // assert
        assert!(result.is_err());
    }

    // -- create_app tests --

    #[tokio::test]
    async fn test_create_app_success() {
        // arrange
        let svc = make_service(vec![], vec![]);

        // act
        let app = svc
            .create_app("new_app", Some("desc".into()))
            .await
            .unwrap();

        // assert
        assert_eq!(app.name, "new_app");
    }

    // -- update_app tests --

    #[tokio::test]
    async fn test_update_app_with_description() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let app = svc
            .update_app(
                "collection",
                Some("new desc".into()),
            )
            .await
            .unwrap();

        // assert
        assert_eq!(
            app.description.as_deref(),
            Some("new desc")
        );
    }

    #[tokio::test]
    async fn test_update_app_not_found_returns_error() {
        // arrange
        let svc = make_service(vec![], vec![]);

        // act
        let result =
            svc.update_app("missing", None).await;

        // assert
        assert!(result.is_err());
    }

    // -- delete_app tests --

    #[tokio::test]
    async fn test_delete_app_existing_returns_true() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let deleted =
            svc.delete_app("collection").await.unwrap();

        // assert
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_delete_app_missing_returns_false() {
        // arrange
        let svc = make_service(vec![], vec![]);

        // act
        let deleted =
            svc.delete_app("nope").await.unwrap();

        // assert
        assert!(!deleted);
    }

    // -- get_user_apps tests --

    #[tokio::test]
    async fn test_get_user_apps_with_apps() {
        // arrange
        let apps = seed_apps();
        let user_apps =
            vec![UserApp::new(1, "usr-1", 1)];
        let svc = make_service(apps, user_apps);

        // act
        let result =
            svc.get_user_apps("usr-1").await.unwrap();

        // assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "collection");
    }

    #[tokio::test]
    async fn test_get_user_apps_empty() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let result =
            svc.get_user_apps("usr-1").await.unwrap();

        // assert
        assert!(result.is_empty());
    }

    // -- add_user_app tests --

    #[tokio::test]
    async fn test_add_user_app_valid_app_succeeds() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let ua = svc
            .add_user_app("usr-1", "collection")
            .await
            .unwrap();

        // assert
        assert_eq!(ua.user_id, "usr-1");
        assert_eq!(ua.app_id, 1);
    }

    #[tokio::test]
    async fn test_add_user_app_invalid_app_returns_err() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let result =
            svc.add_user_app("usr-1", "unknown").await;

        // assert
        assert!(result.is_err());
    }

    // -- remove_user_app tests --

    #[tokio::test]
    async fn test_remove_user_app_existing_succeeds() {
        // arrange
        let apps = seed_apps();
        let user_apps =
            vec![UserApp::new(1, "usr-1", 1)];
        let svc = make_service(apps, user_apps);

        // act
        let result = svc
            .remove_user_app("usr-1", "collection")
            .await
            .unwrap();

        // assert
        assert!(result);
    }

    #[tokio::test]
    async fn test_remove_user_app_not_found_returns_err() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let result = svc
            .remove_user_app("usr-1", "collection")
            .await;

        // assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_user_app_invalid_app_returns_err(
    ) {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let result =
            svc.remove_user_app("usr-1", "bogus").await;

        // assert
        assert!(result.is_err());
    }

    // -- user_has_app tests --

    #[tokio::test]
    async fn test_user_has_app_true() {
        // arrange
        let apps = seed_apps();
        let user_apps =
            vec![UserApp::new(1, "usr-1", 1)];
        let svc = make_service(apps, user_apps);

        // act
        let has = svc
            .user_has_app("usr-1", "collection")
            .await
            .unwrap();

        // assert
        assert!(has);
    }

    #[tokio::test]
    async fn test_user_has_app_false() {
        // arrange
        let svc = make_service(seed_apps(), vec![]);

        // act
        let has = svc
            .user_has_app("usr-1", "collection")
            .await
            .unwrap();

        // assert
        assert!(!has);
    }
}
