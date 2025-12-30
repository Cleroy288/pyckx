//! User entity - Core user representation with auth tokens

use crate::infra::user::UserId;

/// User - Internal representation with tokens (never sent to frontend)
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub username: String,
    pub role: String,
    pub access_token: String,  // JWT - stored server-side only
    pub refresh_token: String, // Refresh token - stored server-side only
    pub expires_at: u64,
}
