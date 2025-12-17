//! Supabase App Repository - PostgreSQL implementation via REST API

use super::types::{AppRow, InsertAppRow, UpdateAppRow};
use crate::config::Config;
use crate::domain::AppEntity;
use crate::error::AppError;
use crate::infrastructure::repository::{AppRepository, CreateApp, UpdateApp};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, error, info, instrument};

/// Supabase implementation of AppRepository
#[derive(Clone, Debug)]
pub struct SupabaseAppRepository {
    url: String,
    anon_key: String,
    service_role_key: String,
    client: Client,
}

impl SupabaseAppRepository {
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "SupabaseAppRepository initialized");
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }

    fn apps_endpoint(&self) -> String {
        format!("{}/rest/v1/apps", self.url)
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
impl AppRepository for SupabaseAppRepository {
    #[instrument(skip(self))]
    async fn find_all(&self) -> Result<Vec<AppEntity>, AppError> {
        let endpoint = format!("{}?order=name.asc", self.apps_endpoint());
        debug!(endpoint = %endpoint, "Finding all apps");

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

        let rows: Vec<AppRow> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

        let apps: Result<Vec<AppEntity>, AppError> = rows.into_iter().map(AppEntity::try_from).collect();
        let apps = apps?;
        
        info!(count = apps.len(), "Retrieved all apps");
        Ok(apps)
    }

    #[instrument(skip(self), fields(name = %name))]
    async fn find_by_name(&self, name: &str) -> Result<AppEntity, AppError> {
        let endpoint = format!("{}?name=eq.{}", self.apps_endpoint(), name);
        debug!(endpoint = %endpoint, "Finding app by name");

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

        let rows: Vec<AppRow> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

        let row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::AppNotFound(name.to_string()))?;

        AppEntity::try_from(row)
    }

    #[instrument(skip(self, app), fields(name = %app.name))]
    async fn insert(&self, app: &CreateApp) -> Result<AppEntity, AppError> {
        if self.exists(&app.name).await? {
            return Err(AppError::AppAlreadyExists(app.name.clone()));
        }

        let row = InsertAppRow::from(app);
        let endpoint = self.apps_endpoint();
        debug!(endpoint = %endpoint, "Inserting app");

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
            return Err(AppError::Internal(format!("Insert failed: {}", status)));
        }

        let rows: Vec<AppRow> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

        let app_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("No row returned".to_string()))?;

        let created = AppEntity::try_from(app_row)?;
        info!(app_id = created.id, "App created");
        Ok(created)
    }

    #[instrument(skip(self, update), fields(name = %name))]
    async fn update(&self, name: &str, update: &UpdateApp) -> Result<AppEntity, AppError> {
        self.find_by_name(name).await?;

        let row = UpdateAppRow::from(update);
        let endpoint = format!("{}?name=eq.{}", self.apps_endpoint(), name);
        debug!(endpoint = %endpoint, "Updating app");

        let mut request = self.client.patch(&endpoint);
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
            error!(status = %status, body = %body, "Supabase update failed");
            return Err(AppError::Internal(format!("Update failed: {}", status)));
        }

        let rows: Vec<AppRow> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

        let app_row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::AppNotFound(name.to_string()))?;

        let updated = AppEntity::try_from(app_row)?;
        info!(app_id = updated.id, "App updated");
        Ok(updated)
    }

    #[instrument(skip(self), fields(name = %name))]
    async fn delete(&self, name: &str) -> Result<bool, AppError> {
        let endpoint = format!("{}?name=eq.{}", self.apps_endpoint(), name);
        debug!(endpoint = %endpoint, "Deleting app");

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

        let rows: Vec<AppRow> = response.json().await.unwrap_or_default();
        let deleted = !rows.is_empty();

        if deleted {
            info!(name = %name, "App deleted");
        }
        Ok(deleted)
    }

    #[instrument(skip(self), fields(name = %name))]
    async fn exists(&self, name: &str) -> Result<bool, AppError> {
        let endpoint = format!("{}?name=eq.{}&select=id", self.apps_endpoint(), name);

        let mut request = self.client.get(&endpoint);
        for (key, value) in self.headers() {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("HTTP error: {}", e)))?;

        #[derive(Deserialize)]
        struct IdOnly { #[allow(dead_code)] id: i32 }

        let rows: Vec<IdOnly> = response.json().await.unwrap_or_default();
        Ok(!rows.is_empty())
    }
}
