//! Auth API — Login/Logout/Register/Session

use super::{endpoints, helpers};
use crate::domain::auth_types::{
    AuthResponse, LoginRequest, RegisterRequest,
};
use gloo_net::http::Request;

/// Get current user from session cookie
pub async fn get_me() -> Option<AuthResponse> {
    let resp = Request::get(endpoints::ME)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .ok()?;

    if resp.ok() {
        resp.json::<AuthResponse>().await.ok()
    } else {
        None
    }
}

/// Login with email and password
pub async fn login(
    email: &str,
    password: &str,
) -> Result<AuthResponse, String> {
    let body = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };
    helpers::post_json(endpoints::LOGIN, &body).await
}

/// Logout — clears session cookie
pub async fn logout() -> Result<(), String> {
    let resp = Request::post(endpoints::LOGOUT)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!(
            "Logout failed ({})",
            resp.status()
        ))
    }
}

/// Register a new user
pub async fn register(
    req: &RegisterRequest,
) -> Result<AuthResponse, String> {
    helpers::post_json(endpoints::REGISTER, req).await
}
