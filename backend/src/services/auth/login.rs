//! Login operation

use super::AuthService;
use crate::domain::User;
use crate::error::{AppError, AppResult, AuthError};
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
            .map_err(|e| AppError::Auth(AuthError::from(e)))?;

        self.sessions.create_session(user.clone());
        info!(user_id = %user.id, "User logged in");

        Ok(user)
    }
}
