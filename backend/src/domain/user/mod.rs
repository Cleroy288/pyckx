//! User domain entity - Core user representation

/// Type alias for user IDs (Supabase user ID)
pub type UserId = String;

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
