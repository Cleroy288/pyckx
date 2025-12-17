//! Supabase API types - Request/Response structures

use crate::domain::User;
use serde::{Deserialize, Serialize};

// ============================================================================
// REQUEST TYPES
// ============================================================================

#[derive(Serialize)]
pub struct LoginBody<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
pub struct RegisterBody<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub data: RegisterMetadata<'a>,
}

#[derive(Serialize)]
pub struct RegisterMetadata<'a> {
    pub username: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,
}

// ============================================================================
// RESPONSE TYPES
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    token_type: String,
    pub expires_in: u64,
    pub expires_at: u64,
    pub refresh_token: String,
    pub user: SupabaseUserRaw,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SupabaseUserRaw {
    pub id: String,
    pub email: String,
    pub role: String,
    aud: String,
    app_metadata: serde_json::Value,
    pub user_metadata: serde_json::Value,
    created_at: Option<String>,
    email_confirmed_at: Option<String>,
    last_sign_in_at: Option<String>,
}

// ============================================================================
// CONVERSION TO DOMAIN MODEL
// ============================================================================

impl From<SupabaseAuthResponse> for User {
    fn from(resp: SupabaseAuthResponse) -> Self {
        let username = resp
            .user
            .user_metadata
            .get("username")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        User {
            id: resp.user.id,
            email: resp.user.email,
            username,
            role: resp.user.role,
            access_token: resp.access_token,
            refresh_token: resp.refresh_token,
            expires_at: resp.expires_at,
        }
    }
}
