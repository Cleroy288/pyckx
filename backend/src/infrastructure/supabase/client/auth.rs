//! Supabase HTTP client - Handles all Supabase API communication

use super::types::{LoginBody, RegisterBody, RegisterMetadata, SupabaseAuthResponse};
use crate::config::Config;
use crate::domain::User;
use crate::error::SupabaseError;
use crate::shared::constants::urls::{SUPABASE_AUTH_PATH, SUPABASE_LOGOUT_PATH, SUPABASE_SIGNUP_PATH};
use reqwest::Client;
use std::fmt;
use tracing::{debug, info, instrument, warn};

/// Supabase API client
#[derive(Clone, Debug)]
pub struct SupabaseClient {
    url: String,
    anon_key: String,
}

impl SupabaseClient {
    pub fn new(cfg: &Config) -> Self {
        info!(url = %cfg.sp_url, "Supabase client initialized");
        Self {
            url: cfg.sp_url.clone(),
            anon_key: cfg.sp_anon.clone(),
        }
    }

    /// Login with email and password
    #[instrument(skip(self, password), fields(email = %email))]
    pub async fn login(&self, email: &str, password: &str) -> Result<User, SupabaseError> {
        let endpoint = format!("{}{}", self.url, SUPABASE_AUTH_PATH);
        debug!(endpoint = %endpoint, "Sending login request");

        let response = Client::new()
            .post(&endpoint)
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&LoginBody { email, password })
            .send()
            .await
            .map_err(SupabaseError::from_reqwest)?;

        let parsed: SupabaseAuthResponse = SupabaseError::parse_response(response).await?;
        info!(user_id = %parsed.user.id, "Login successful");
        Ok(parsed.into())
    }


    /// Register a new user with profile data
    #[instrument(skip(self, password), fields(email = %email, username = %username))]
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        username: &str,
        phone_country_code: Option<&str>,
        phone_number: Option<&str>,
    ) -> Result<User, SupabaseError> {
        let endpoint = format!("{}{}", self.url, SUPABASE_SIGNUP_PATH);
        debug!(endpoint = %endpoint, "Sending register request");

        let response = Client::new()
            .post(&endpoint)
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&RegisterBody {
                email,
                password,
                data: RegisterMetadata {
                    username,
                    phone_country_code,
                    phone_number,
                },
            })
            .send()
            .await
            .map_err(SupabaseError::from_reqwest)?;

        let parsed: SupabaseAuthResponse = SupabaseError::parse_response(response).await?;
        info!(user_id = %parsed.user.id, "Registration successful");
        Ok(parsed.into())
    }

    /// Logout user by invalidating their access token with Supabase
    /// This is a best-effort operation - we don't fail if Supabase is unreachable
    #[instrument(skip(self, access_token))]
    pub async fn logout(&self, access_token: &str) {
        let endpoint = format!("{}{}", self.url, SUPABASE_LOGOUT_PATH);
        debug!(endpoint = %endpoint, "Sending logout request to Supabase");

        let result = Client::new()
            .post(&endpoint)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await;

        match result {
            Ok(response) if response.status().is_success() => {
                info!("Supabase logout successful");
            }
            Ok(response) => {
                warn!(status = %response.status(), "Supabase logout returned non-success status");
            }
            Err(e) => {
                warn!(error = %e, "Failed to notify Supabase of logout");
            }
        }
    }
}

impl fmt::Display for SupabaseClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SupabaseClient(url={})", self.url)
    }
}
