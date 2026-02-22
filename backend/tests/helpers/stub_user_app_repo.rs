//! Stub UserAppRepository for integration tests

use async_trait::async_trait;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

use LAPP::infra::UserAppRepository;
use LAPP::services::app_registry::error_domain::AppError as AppsError;
use LAPP::services::app_registry::registry_domain::UserApp;
use LAPP::shared::AppError;

/// In-memory stub for UserAppRepository
pub struct StubUserAppRepository {
    user_apps: Mutex<Vec<UserApp>>,
    next_id: AtomicI32,
}

impl StubUserAppRepository {
    /// Create an empty stub
    pub fn new() -> Self {
        Self {
            user_apps: Mutex::new(vec![]),
            next_id: AtomicI32::new(1),
        }
    }

    /// Create a stub pre-seeded with user apps
    pub fn with_user_apps(items: Vec<UserApp>) -> Self {
        let max_id = items.iter().map(|ua| ua.id).max().unwrap_or(0);
        Self {
            user_apps: Mutex::new(items),
            next_id: AtomicI32::new(max_id + 1),
        }
    }
}

#[async_trait]
impl UserAppRepository for StubUserAppRepository {
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserApp>, AppError> {
        let items = self.user_apps.lock().unwrap();
        Ok(items
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
        let mut items = self.user_apps.lock().unwrap();
        // Check duplicate
        if items
            .iter()
            .any(|ua| ua.user_id == user_id && ua.app_id == app_id)
        {
            return Err(AppError::App(AppsError::UserAppAlreadyAdded {
                user_id: user_id.to_string(),
                app_name: format!("app_{}", app_id),
            }));
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let ua = UserApp::new(id, user_id, app_id);
        items.push(ua.clone());
        Ok(ua)
    }

    async fn remove_app(
        &self,
        user_id: &str,
        app_id: i32,
    ) -> Result<bool, AppError> {
        let mut items = self.user_apps.lock().unwrap();
        let before = items.len();
        items.retain(|ua| {
            !(ua.user_id == user_id && ua.app_id == app_id)
        });
        Ok(items.len() < before)
    }

    async fn has_app(
        &self,
        user_id: &str,
        app_id: i32,
    ) -> Result<bool, AppError> {
        let items = self.user_apps.lock().unwrap();
        Ok(items
            .iter()
            .any(|ua| ua.user_id == user_id && ua.app_id == app_id))
    }
}
