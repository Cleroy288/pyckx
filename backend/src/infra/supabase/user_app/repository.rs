/*
 * Supabase UserApp Repository
 *
 * Manages user-to-application associations.
 * Uses shared SupabaseHttpClient for connection pooling.
 *
 * Tables: user_apps
 */

use super::types::{InsertUserAppRow, UserAppRow};
use crate::infra::database::UserAppRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::app_registry::registry_domain::UserApp;
use crate::shared::AppError;
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_USER_APPS: &str = "user_apps";

/* ============================================================================
 * SUPABASE USERAPP REPOSITORY
 * ============================================================================ */

#[derive(Clone, Debug)]
pub struct SupabaseUserAppRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseUserAppRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseUserAppRepository initialized with shared client");
        Self { client }
    }

    fn map_error(err: SupabaseError) -> AppError {
        AppError::Internal(crate::http_api::utils::InternalError::new(
            err.to_string(),
        ))
    }
}

#[async_trait]
impl UserAppRepository for SupabaseUserAppRepository {
    /* READ: Find by user */
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserApp>, AppError> {
        let query = format!("user_id=eq.{}&order=id.desc", user_id);
        let url = self.client.rest_url_with_query(TABLE_USER_APPS, &query);
        debug!(url = %url, "Finding user apps");

        let rows: Vec<UserAppRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        let user_apps: Result<Vec<UserApp>, AppError> =
            rows.into_iter().map(UserApp::try_from).collect();
        let user_apps = user_apps?;

        info!(count = user_apps.len(), "Retrieved user apps");
        Ok(user_apps)
    }

    /* CREATE: Add app */
    #[instrument(skip(self), fields(user_id = %user_id, app_id = %app_id))]
    async fn add_app(
        &self,
        user_id: &str,
        app_id: i32,
    ) -> Result<UserApp, AppError> {
        if self.has_app(user_id, app_id).await? {
            return Err(AppError::App(
                crate::services::app_registry::AppsError::UserAppAlreadyAdded {
                    user_id: user_id.to_string(),
                    app_name: format!("app_id:{}", app_id),
                },
            ));
        }

        let row = InsertUserAppRow {
            user_id: user_id.to_string(),
            app_id,
        };
        let url = self.client.rest_url(TABLE_USER_APPS);
        debug!(url = %url, "Adding user app");

        let rows: Vec<UserAppRow> = self
            .client
            .post(&url, &row)
            .await
            .map_err(Self::map_error)?;
        let user_app_row = rows.into_iter().next().ok_or_else(|| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                "No row returned".to_string(),
            ))
        })?;

        let user_app = UserApp::try_from(user_app_row)?;
        info!(user_app_id = user_app.id, "User app added");
        Ok(user_app)
    }

    /* DELETE: Remove app */
    #[instrument(skip(self), fields(user_id = %user_id, app_id = %app_id))]
    async fn remove_app(
        &self,
        user_id: &str,
        app_id: i32,
    ) -> Result<bool, AppError> {
        let query = format!("user_id=eq.{}&app_id=eq.{}", user_id, app_id);
        let url = self.client.rest_url_with_query(TABLE_USER_APPS, &query);
        debug!(url = %url, "Removing user app");

        self.client.delete(&url).await.map_err(Self::map_error)?;
        info!(user_id = %user_id, app_id = %app_id, "User app removed");
        Ok(true)
    }

    /* HELPER: Check if user has app */
    #[instrument(skip(self), fields(user_id = %user_id, app_id = %app_id))]
    async fn has_app(
        &self,
        user_id: &str,
        app_id: i32,
    ) -> Result<bool, AppError> {
        let query =
            format!("user_id=eq.{}&app_id=eq.{}&select=id", user_id, app_id);
        let url = self.client.rest_url_with_query(TABLE_USER_APPS, &query);

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: i32,
        }

        let rows: Vec<IdOnly> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        Ok(!rows.is_empty())
    }
}
