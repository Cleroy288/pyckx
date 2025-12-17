//! Supabase UserApp Repository - PostgreSQL implementation via REST API

use super::types::{InsertUserAppRow, UserAppRow};
use crate::config::Config;
use crate::domain::UserApp;
use crate::error::AppError;
use crate::infrastructure::repository::UserAppRepository;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, error, info, instrument};

/// Supabase implementation of UserAppRepository
#[derive(Clone, Debug)]
pub struct SupabaseUserAppRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseUserAppRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseUserAppRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn user_apps_endpoint(&self) -> String {
        format!("{}/rest/v1/user_apps", self.url)
    }

    fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.service_role_key)),
            ("Content-Type", "application/json".to_string()),
            ("Prefer", "return=representation".to_string()),
        ]
    }
}

#[async_trait]
impl UserAppRepository for SupabaseUserAppRepository {
    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<UserApp>, AppError> {
        let endpoint = format!(
            "{}?user_id=eq.{}&order=id.desc",
            self.user_apps_endpoint(),
            user_id
        );
        debug!(endpoint = %endpoint, "Finding user apps");

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase select failed");
            return Err(AppError::Internal(format!("Select failed: {}", status)));
        }

        let rows: Vec<UserAppRow> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

        let user_apps: Result<Vec<UserApp>, AppError> =
            rows.into_iter().map(UserApp::try_from).collect();
        let user_apps = user_apps?;

        info!(count = user_apps.len(), "Retrieved user apps");
        Ok(user_apps)
    }

    #[instrument(skip(self), fields(user_id = %user_id, app_id = %app_id))]
    async fn add_app(&self, user_id: &str, app_id: i32) -> Result<UserApp, AppError> {
        if self.has_app(user_id, app_id).await? {
            return Err(AppError::UserAppAlreadyAdded {
                user_id: user_id.to_string(),
                app_name: format!("app_id:{}", app_id),
            });
        }

        let row = InsertUserAppRow {
            user_id: user_id.to_string(),
            app_id,
        };

        let endpoint = self.user_apps_endpoint();
        debug!(endpoint = %endpoint, "Adding user app");

        let mut request = self.client.post(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .json(&row)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase insert failed");
            return Err(AppError::Internal(format!(
                "Insert failed: {} - {}",
                status, body
            )));
        }

        let rows: Vec<UserAppRow> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

        let user_app_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("No row returned".to_string()))?;

        let user_app = UserApp::try_from(user_app_row)?;
        info!(user_app_id = user_app.id, "User app added");
        Ok(user_app)
    }

    #[instrument(skip(self), fields(user_id = %user_id, app_id = %app_id))]
    async fn remove_app(&self, user_id: &str, app_id: i32) -> Result<bool, AppError> {
        let endpoint = format!(
            "{}?user_id=eq.{}&app_id=eq.{}",
            self.user_apps_endpoint(),
            user_id,
            app_id
        );
        debug!(endpoint = %endpoint, "Removing user app");

        let mut request = self.client.delete(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Supabase delete failed");
            return Err(AppError::Internal(format!("Delete failed: {}", status)));
        }

        let rows: Vec<UserAppRow> = response.json().await.unwrap_or_default();
        let removed = !rows.is_empty();

        if removed {
            info!(user_id = %user_id, app_id = %app_id, "User app removed");
        }
        Ok(removed)
    }

    #[instrument(skip(self), fields(user_id = %user_id, app_id = %app_id))]
    async fn has_app(&self, user_id: &str, app_id: i32) -> Result<bool, AppError> {
        let endpoint = format!(
            "{}?user_id=eq.{}&app_id=eq.{}&select=id",
            self.user_apps_endpoint(),
            user_id,
            app_id
        );

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("HTTP error: {}", e)))?;

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: i32,
        }

        let rows: Vec<IdOnly> = response.json().await.unwrap_or_default();
        Ok(!rows.is_empty())
    }
}
