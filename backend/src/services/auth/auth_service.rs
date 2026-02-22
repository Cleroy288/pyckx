//! Auth Service - Core authentication logic
//!
//! Handles login, logout, and registration flows.

use super::types_domain::AuthService;
use crate::infra::User;
use crate::services::auth::error_domain::AuthError;
use crate::shared::{AppError, AppResult};
use tracing::{info, instrument};

impl AuthService {
    /// Login user with email and password
    /// Returns User on success, AppError on failure (automatically logged)
    #[instrument(skip(self, password), fields(email = %email))]
    pub async fn login(&self, email: &str, password: &str) -> AppResult<User> {
        let user = self
            .supabase
            .login(email, password)
            .await
            .map_err(|err| AppError::Auth(AuthError::from(err)))?;

        self.sessions.create_session(user.clone());
        info!(user_id = %user.id, "User logged in");

        Ok(user)
    }

    /// Logout user - invalidates session locally and notifies Supabase
    #[instrument(skip(self, session_id))]
    pub async fn logout(&self, session_id: &str) -> bool {
        // Remove from local store and get user data (contains access_token)
        let user = self.sessions.delete_session(session_id);

        if let Some(user) = user {
            // Notify Supabase to invalidate the token (best-effort)
            self.supabase.logout(&user.access_token).await;
            info!(session_id = %session_id, "User logged out");
            true
        } else {
            info!(session_id = %session_id, "Logout attempted but session not found");
            false
        }
    }

    /// Register a new user with profile data
    /// NOTE: Registration route is disabled - users are added via Supabase Dashboard
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self, password), fields(email = %email, username = %username))]
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        username: &str,
        phone_country_code: Option<&str>,
        phone_number: Option<&str>,
    ) -> AppResult<User> {
        let user = self
            .supabase
            .register(
                email,
                password,
                username,
                phone_country_code,
                phone_number,
            )
            .await
            .map_err(|err| AppError::Auth(AuthError::from(err)))?;

        self.sessions.create_session(user.clone());
        info!(user_id = %user.id, "User registered");

        Ok(user)
    }

    /// Get session store reference (for user lookups)
    pub fn sessions(&self) -> &crate::infra::SessionStore {
        &self.sessions
    }
}
