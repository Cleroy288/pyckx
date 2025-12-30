/*
 * Supabase App Repository
 *
 * CRUD implementation for applications using Supabase.
 * Uses shared SupabaseHttpClient for connection pooling.
 *
 * Tables: apps
 */

use super::types::{AppRow, InsertAppRow, UpdateAppRow};
use crate::services::app_registry::registry_domain::App as AppEntity;
use crate::shared::AppError;
use crate::infra::database::{AppRepository, CreateApp, UpdateApp};
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_APPS: &str = "apps";

/* ============================================================================
 * SUPABASE APP REPOSITORY
 * ============================================================================ */

/*
 * Repository for application persistence operations.
 *
 * Manages application metadata in the system.
 */
#[derive(Clone, Debug)]
pub struct SupabaseAppRepository {
    client: Arc<SupabaseHttpClient>,
}

/* ============================================================================
 * CONSTRUCTOR
 * ============================================================================ */

impl SupabaseAppRepository {
    /*
     * Create repository with shared HTTP client.
     */
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseAppRepository initialized with shared client");
        Self { client }
    }

    /* ========================================================================
     * HELPER: Error Mapping
     * ======================================================================== */

    /*
     * Convert SupabaseError to AppError.
     */
    fn map_error(err: SupabaseError) -> AppError {
        AppError::Internal(crate::http_api::utils::InternalError::new(err.to_string()))
    }
}

/* ============================================================================
 * CRUD IMPLEMENTATION
 * ============================================================================ */

#[async_trait]
impl AppRepository for SupabaseAppRepository {
    /* ========================================================================
     * READ: Find all apps
     * ======================================================================== */

    /*
     * Retrieve all applications, ordered by name.
     */
    #[instrument(skip(self))]
    async fn find_all(&self) -> Result<Vec<AppEntity>, AppError> {
        let url = self.client.rest_url_with_query(TABLE_APPS, "order=name.asc");
        debug!(url = %url, "Finding all apps");

        let rows: Vec<AppRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        let apps: Result<Vec<AppEntity>, AppError> =
            rows.into_iter().map(AppEntity::try_from).collect();
        let apps = apps?;

        info!(count = apps.len(), "Retrieved all apps");
        Ok(apps)
    }

    /* ========================================================================
     * READ: Find by name
     * ======================================================================== */

    /*
     * Find an application by its name.
     *
     * Returns AppNotFound error if doesn't exist.
     */
    #[instrument(skip(self), fields(name = %name))]
    async fn find_by_name(&self, name: &str) -> Result<AppEntity, AppError> {
        let query = format!("name=eq.{}", name);
        let url = self.client.rest_url_with_query(TABLE_APPS, &query);
        debug!(url = %url, "Finding app by name");

        let rows: Vec<AppRow> = self.client.get(&url).await.map_err(Self::map_error)?;

        let row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::App(crate::services::app_registry::AppsError::NotFound(name.to_string())))?;

        AppEntity::try_from(row)
    }

    /* ========================================================================
     * CREATE: Insert app
     * ======================================================================== */

    /*
     * Create a new application.
     *
     * Returns AppAlreadyExists error if name is taken.
     */
    #[instrument(skip(self, app),fields(name = %app.name))]
    async fn insert(&self, app: &CreateApp) -> Result<AppEntity, AppError> {
        if self.exists(&app.name).await? {
            return Err(AppError::App(crate::services::app_registry::AppsError::AlreadyExists(app.name.clone())));
        }

        let row = InsertAppRow::from(app);
        let url = self.client.rest_url(TABLE_APPS);
        debug!(url = %url, "Inserting app");

        let rows: Vec<AppRow> = self.client.post(&url, &row).await.map_err(Self::map_error)?;

        let app_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal(crate::http_api::utils::InternalError::new("No row returned".to_string())))?;

        let created = AppEntity::try_from(app_row)?;
        info!(app_id = created.id, "App created");
        Ok(created)
    }

    /* ========================================================================
     * UPDATE: Update app
     * ======================================================================== */

    /*
     * Update an existing application.
     *
     * Returns AppNotFound if doesn't exist.
     */
    #[instrument(skip(self, update), fields(name = %name))]
    async fn update(&self, name: &str, update: &UpdateApp) -> Result<AppEntity, AppError> {
        self.find_by_name(name).await?;

        let row = UpdateAppRow::from(update);
        let query = format!("name=eq.{}", name);
        let url = self.client.rest_url_with_query(TABLE_APPS, &query);
        debug!(url = %url, "Updating app");

        let rows: Vec<AppRow> = self.client.patch(&url, &row).await.map_err(Self::map_error)?;

        let app_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::App(crate::services::app_registry::AppsError::NotFound(name.to_string())))?;

        let updated = AppEntity::try_from(app_row)?;
        info!(app_id = updated.id, "App updated");
        Ok(updated)
    }

    /* ========================================================================
     * DELETE: Delete app
     * ======================================================================== */

    /*
     * Delete an application by name.
     *
     * Returns true if deleted, false if not found.
     */
    #[instrument(skip(self), fields(name = %name))]
    async fn delete(&self, name: &str) -> Result<bool, AppError> {
        let query = format!("name=eq.{}", name);
        let url = self.client.rest_url_with_query(TABLE_APPS, &query);
        debug!(url = %url, "Deleting app");

        self.client.delete(&url).await.map_err(Self::map_error)?;

        /* Delete returns empty response, assume success if no error */
        info!(name = %name, "App deleted");
        Ok(true)
    }

    /* ========================================================================
     * HELPER: Check existence
     * ======================================================================== */

    /*
     * Check if an application exists by name.
     */
    #[instrument(skip(self), fields(name = %name))]
    async fn exists(&self, name: &str) -> Result<bool, AppError> {
        let query = format!("name=eq.{}&select=id", name);
        let url = self.client.rest_url_with_query(TABLE_APPS, &query);

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: i32,
        }

        let rows: Vec<IdOnly> = self.client.get(&url).await.map_err(Self::map_error)?;
        Ok(!rows.is_empty())
    }
}

/* ============================================================================
 * LEGACY CONSTRUCTOR (COMMENTED - Preserved for reference)
 * ============================================================================ */

/*
 * Legacy constructor that creates its own HTTP client.
 *
 * DEPRECATED: Use new(Arc<SupabaseHttpClient>) instead.
 */
// impl SupabaseAppRepository {
//     pub fn new_legacy(config: &crate::configs::Config) -> Self {
//         let client = Arc::new(SupabaseHttpClient::new(config));
//         Self { client }
//     }
// }
