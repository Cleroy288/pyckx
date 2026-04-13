//! Shared Supabase HTTP Client
//!
//! Unified HTTP client for all Supabase REST operations
//! with connection pooling, configurable timeouts, and
//! automatic retry with exponential backoff.

use super::error::SupabaseError;
use crate::configs::Config;
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

/// Max retry attempts for transient failures.
const MAX_ATTEMPTS: u32 = 3;

/// Base delay for exponential backoff (ms).
const BASE_DELAY_MS: u64 = 100;

/// Shared HTTP client for all Supabase operations.
///
/// Create once at startup and wrap in `Arc` to share
/// across all repository instances.
#[derive(Clone, Debug)]
pub struct SupabaseHttpClient {
    client: Client,
    base_url: String,
    anon_key: String,
    service_role_key: String,
}

impl SupabaseHttpClient {
    /// Creates a new client with connection pooling.
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "Initializing SupabaseHttpClient");

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role_key: config.sp_service_role.clone(),
        }
    }

    /// Builds `{base}/rest/v1/{table}`.
    pub fn rest_url(&self, table: &str) -> String {
        format!("{}/rest/v1/{}", self.base_url, table)
    }

    /// Builds `{base}/rest/v1/{table}?{query}`.
    pub fn rest_url_with_query(
        &self,
        table: &str,
        query: &str,
    ) -> String {
        format!(
            "{}/rest/v1/{}?{}",
            self.base_url, table, query
        )
    }

    /// Builds `{base}/rest/v1/rpc/{fn_name}`.
    pub fn rpc_url(&self, function_name: &str) -> String {
        format!(
            "{}/rest/v1/rpc/{}",
            self.base_url, function_name
        )
    }

    /// GET with retry and JSON deserialization.
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, SupabaseError> {
        self.request_with_retry(
            Method::GET,
            url,
            None::<&()>,
        )
        .await
    }

    /// POST with retry and JSON deserialization.
    pub async fn post<T, B>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, SupabaseError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_with_retry(
            Method::POST,
            url,
            Some(body),
        )
        .await
    }

    /// PATCH with retry and JSON deserialization.
    pub async fn patch<T, B>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, SupabaseError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_with_retry(
            Method::PATCH,
            url,
            Some(body),
        )
        .await
    }

    /// DELETE with retry (no response body).
    pub async fn delete(
        &self,
        url: &str,
    ) -> Result<(), SupabaseError> {
        self.delete_with_retry(url).await
    }
}

/// Internal retry and execution methods.
impl SupabaseHttpClient {
    /// Executes a DELETE with retry logic.
    async fn delete_with_retry(
        &self,
        url: &str,
    ) -> Result<(), SupabaseError> {
        let mut last_err = SupabaseError::Timeout;

        for attempt in 0..MAX_ATTEMPTS {
            match self.send_delete(url).await {
                Ok(()) => return Ok(()),
                Err(e) if e.is_retryable()
                    && attempt < MAX_ATTEMPTS - 1 =>
                {
                    let delay = Duration::from_millis(
                        BASE_DELAY_MS * (1 << attempt),
                    );
                    warn!(
                        attempt = attempt + 1,
                        delay_ms = ?delay.as_millis(),
                        "Retrying DELETE"
                    );
                    tokio::time::sleep(delay).await;
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_err)
    }

    /// Executes a JSON request with retry.
    async fn request_with_retry<T, B>(
        &self,
        method: Method,
        url: &str,
        body: Option<&B>,
    ) -> Result<T, SupabaseError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        // Cannot use `retry` here due to lifetime
        // constraints on `body` reference across closures.
        let mut last_err = SupabaseError::Timeout;

        for attempt in 0..MAX_ATTEMPTS {
            match self
                .execute_request(&method, url, body)
                .await
            {
                Ok(v) => return Ok(v),
                Err(e) if e.is_retryable()
                    && attempt < MAX_ATTEMPTS - 1 =>
                {
                    let delay = Duration::from_millis(
                        BASE_DELAY_MS * (1 << attempt),
                    );
                    warn!(
                        attempt = attempt + 1,
                        delay_ms = ?delay.as_millis(),
                        "Retrying request"
                    );
                    tokio::time::sleep(delay).await;
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_err)
    }

    /// Sends a single JSON request and parses the response.
    async fn execute_request<T, B>(
        &self,
        method: &Method,
        url: &str,
        body: Option<&B>,
    ) -> Result<T, SupabaseError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        debug!(method = %method, url = %url, "Supabase request");

        let mut req = self
            .client
            .request(method.clone(), url)
            .header("apikey", &self.anon_key)
            .header(
                "Authorization",
                format!("Bearer {}", self.service_role_key),
            )
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation");

        if let Some(b) = body {
            req = req.json(b);
        }

        let resp = req.send().await.map_err(map_send_err)?;
        let status = resp.status();

        if !status.is_success() {
            let body =
                resp.text().await.unwrap_or_default();
            return Err(SupabaseError::http(
                status.as_u16(),
                body,
            ));
        }

        resp.json()
            .await
            .map_err(|e| SupabaseError::parse(e.to_string()))
    }

    /// Sends a single DELETE request (no response body).
    async fn send_delete(
        &self,
        url: &str,
    ) -> Result<(), SupabaseError> {
        debug!(url = %url, "Supabase DELETE");

        let req = self
            .client
            .delete(url)
            .header("apikey", &self.anon_key)
            .header(
                "Authorization",
                format!("Bearer {}", self.service_role_key),
            );

        let resp = req.send().await.map_err(map_send_err)?;

        if resp.status().is_success() {
            return Ok(());
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        Err(SupabaseError::http(status.as_u16(), body))
    }
}

/// Maps a reqwest send error to SupabaseError.
fn map_send_err(err: reqwest::Error) -> SupabaseError {
    if err.is_timeout() {
        SupabaseError::Timeout
    } else {
        SupabaseError::network(err.to_string())
    }
}
