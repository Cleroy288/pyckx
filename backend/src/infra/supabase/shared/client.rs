/*
 * Shared Supabase HTTP Client
 *
 * Provides a unified HTTP client for all Supabase REST operations.
 * Features:
 * - Connection pooling (single client shared across all repositories)
 * - Configurable timeouts (30s request, 5s connect)
 * - Automatic retry with exponential backoff (100ms, 200ms, 400ms)
 * - Pre-built authentication headers
 */

use super::error::SupabaseError;
use crate::configs::Config;
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

/* ============================================================================
 * SUPABASE HTTP CLIENT
 * ============================================================================ */

/*
 * Shared HTTP client for all Supabase operations.
 *
 * Should be created once at application startup and wrapped in Arc
 * to share across all repository instances.
 *
 * Example:
 * ```
 * let client = Arc::new(SupabaseHttpClient::new(&config));
 * let qcm_repo = SupabaseQcmRepository::new(Arc::clone(&client));
 * ```
 */
#[derive(Clone, Debug)]
pub struct SupabaseHttpClient {
    client: Client,
    base_url: String,
    anon_key: String,
    service_role_key: String,
}

/* ============================================================================
 * CONSTRUCTOR
 * ============================================================================ */

impl SupabaseHttpClient {
    /*
     * Create new HTTP client with connection pooling.
     *
     * Configuration:
     * - Request timeout: 30 seconds
     * - Connect timeout: 5 seconds
     * - Max idle connections per host: 10
     */
    pub fn new(config: &Config) -> Self {
        info!(url = %config.sp_url, "Initializing shared SupabaseHttpClient");

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

    /* ========================================================================
     * URL HELPERS
     * ======================================================================== */

    /*
     * Build REST API URL for a table.
     *
     * Returns: {base_url}/rest/v1/{table}
     */
    pub fn rest_url(&self, table: &str) -> String {
        format!("{}/rest/v1/{}", self.base_url, table)
    }

    /*
     * Build REST API URL with query string.
     *
     * Returns: {base_url}/rest/v1/{table}?{query}
     */
    pub fn rest_url_with_query(&self, table: &str, query: &str) -> String {
        format!("{}/rest/v1/{}?{}", self.base_url, table, query)
    }

    /*
     * Build RPC function URL.
     *
     * Returns: {base_url}/rest/v1/rpc/{function_name}
     */
    pub fn rpc_url(&self, function_name: &str) -> String {
        format!("{}/rest/v1/rpc/{}", self.base_url, function_name)
    }

    /* ========================================================================
     * CRUD OPERATIONS
     * ======================================================================== */

    /*
     * GET request - Read operations.
     *
     * Performs GET request and deserializes JSON response.
     * Includes automatic retry on transient failures.
     */
    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, SupabaseError> {
        self.request_with_retry(Method::GET, url, None::<&()>).await
    }

    /*
     * POST request - Create operations.
     *
     * Sends JSON body and deserializes JSON response.
     * Includes automatic retry on transient failures.
     */
    pub async fn post<T, B>(&self, url: &str, body: &B) -> Result<T, SupabaseError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_with_retry(Method::POST, url, Some(body)).await
    }

    /*
     * PATCH request - Update operations.
     *
     * Sends JSON body and deserializes JSON response.
     * Includes automatic retry on transient failures.
     */
    pub async fn patch<T, B>(&self, url: &str, body: &B) -> Result<T, SupabaseError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_with_retry(Method::PATCH, url, Some(body))
            .await
    }

    /*
     * DELETE request - Delete operations.
     *
     * Returns empty result on success.
     * Includes automatic retry on transient failures.
     */
    pub async fn delete(&self, url: &str) -> Result<(), SupabaseError> {
        self.delete_internal(url).await
    }

    /* ========================================================================
     * INTERNAL: REQUEST EXECUTION
     * ======================================================================== */

    /*
     * Execute request with retry logic.
     *
     * Retry strategy:
     * - Max 3 attempts
     * - Exponential backoff: 100ms, 200ms, 400ms
     * - Only retries on transient errors (5xx, 429, network, timeout)
     */
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
        let mut last_error = SupabaseError::Timeout;

        for attempt in 0..3u32 {
            match self.execute_request(&method, url, body).await {
                Ok(response) => return Ok(response),
                Err(e) if e.is_retryable() && attempt < 2 => {
                    let delay = Duration::from_millis(100 * (1 << attempt));
                    warn!(attempt = attempt + 1, delay_ms = ?delay.as_millis(), "Retrying request");
                    tokio::time::sleep(delay).await;
                    last_error = e;
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error)
    }

    /*
     * Execute single HTTP request with JSON body and response.
     */
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
        debug!(method = %method, url = %url, "Executing Supabase request");

        let mut req = self
            .client
            .request(method.clone(), url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation");

        if let Some(b) = body {
            req = req.json(b);
        }

        let response = req.send().await.map_err(|e| {
            if e.is_timeout() {
                SupabaseError::Timeout
            } else {
                SupabaseError::network(e.to_string())
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SupabaseError::http(status.as_u16(), body));
        }

        response
            .json()
            .await
            .map_err(|e| SupabaseError::parse(e.to_string()))
    }

    /*
     * Execute DELETE request (no response body expected).
     */
    async fn delete_internal(&self, url: &str) -> Result<(), SupabaseError> {
        debug!(url = %url, "Executing Supabase DELETE");

        let mut last_error = SupabaseError::Timeout;

        for attempt in 0..3u32 {
            let req = self
                .client
                .delete(url)
                .header("apikey", &self.anon_key)
                .header("Authorization", format!("Bearer {}", self.service_role_key));

            match req.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(());
                    }
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    let err = SupabaseError::http(status.as_u16(), body);

                    if err.is_retryable() && attempt < 2 {
                        let delay = Duration::from_millis(100 * (1 << attempt));
                        warn!(attempt = attempt + 1, "Retrying DELETE request");
                        tokio::time::sleep(delay).await;
                        last_error = err;
                        continue;
                    }
                    return Err(err);
                }
                Err(e) => {
                    let err = if e.is_timeout() {
                        SupabaseError::Timeout
                    } else {
                        SupabaseError::network(e.to_string())
                    };

                    if err.is_retryable() && attempt < 2 {
                        let delay = Duration::from_millis(100 * (1 << attempt));
                        warn!(attempt = attempt + 1, "Retrying DELETE request");
                        tokio::time::sleep(delay).await;
                        last_error = err;
                        continue;
                    }
                    return Err(err);
                }
            }
        }

        Err(last_error)
    }
}
