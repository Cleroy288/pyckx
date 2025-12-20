//! Register operation

use super::AuthService;
use crate::domain::User;
use crate::error::{AppError, AppResult, AuthError};
use tracing::{info, instrument};

impl AuthService {
    /// Register a new user with profile data
    /// NOTE: Registration route is disabled - users are added via Supabase Dashboard
    #[allow(dead_code)]
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
            .register(email, password, username, phone_country_code, phone_number)
            .await
            .map_err(|e| AppError::Auth(AuthError::from(e)))?;

        self.sessions.create_session(user.clone());
        info!(user_id = %user.id, "User registered");

        Ok(user)
    }
}
