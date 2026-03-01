//! Stub AppRepository for integration tests

use async_trait::async_trait;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

use LAPP::infra::{AppRepository, CreateApp, UpdateApp};
use LAPP::services::app_registry::error_domain::AppError as AppsError;
use LAPP::services::app_registry::registry_domain::App;
use LAPP::shared::AppError;

/// In-memory stub for AppRepository
pub struct StubAppRepository {
    apps: Mutex<Vec<App>>,
    next_id: AtomicI32,
}

impl StubAppRepository {
    /// Create an empty stub
    pub fn new() -> Self {
        Self {
            apps: Mutex::new(vec![]),
            next_id: AtomicI32::new(1),
        }
    }

    /// Create a stub pre-seeded with apps
    pub fn with_apps(apps: Vec<App>) -> Self {
        let max_id = apps.iter().map(|a| a.id).max().unwrap_or(0);
        Self {
            apps: Mutex::new(apps),
            next_id: AtomicI32::new(max_id + 1),
        }
    }
}

#[async_trait]
impl AppRepository for StubAppRepository {
    async fn find_all(&self) -> Result<Vec<App>, AppError> {
        let apps = self.apps.lock().unwrap();
        Ok(apps.clone())
    }

    async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<App, AppError> {
        let apps = self.apps.lock().unwrap();
        apps.iter()
            .find(|a| a.name == name)
            .cloned()
            .ok_or_else(|| {
                AppError::App(AppsError::NotFound(name.to_string()))
            })
    }

    async fn insert(
        &self,
        create: &CreateApp,
    ) -> Result<App, AppError> {
        let mut apps = self.apps.lock().unwrap();
        // Check for duplicates
        if apps.iter().any(|a| a.name == create.name) {
            return Err(AppError::App(AppsError::AlreadyExists(
                create.name.clone(),
            )));
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let app = App::new(id, &create.name, create.description.clone());
        apps.push(app.clone());
        Ok(app)
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
                AppError::App(AppsError::NotFound(name.to_string()))
            })?;
        if let Some(ref desc) = update.description {
            app.description = Some(desc.clone());
        }
        Ok(app.clone())
    }

    async fn delete(&self, name: &str) -> Result<bool, AppError> {
        let mut apps = self.apps.lock().unwrap();
        let before = apps.len();
        apps.retain(|a| a.name != name);
        Ok(apps.len() < before)
    }

    async fn exists(&self, name: &str) -> Result<bool, AppError> {
        let apps = self.apps.lock().unwrap();
        Ok(apps.iter().any(|a| a.name == name))
    }
}
