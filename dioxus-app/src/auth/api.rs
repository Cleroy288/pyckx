//! Auth HTTP client — login, register, logout, session.

use crate::api::endpoints;
use crate::api::helpers::post_json;
use crate::domain::auth_types::{
    AuthResponse, LoginRequest, RegisterRequest,
};
use gloo_net::http::Request;
use web_sys::RequestCredentials;

/// Fetch the current user from the session cookie.
/// Returns `None` when no valid session exists.
pub async fn fetch_me() -> Option<AuthResponse> {
    let response = Request::get(endpoints::ME)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .ok()?;
    if !response.ok() {
        return None;
    }
    response.json::<AuthResponse>().await.ok()
}

/// Sign in with email and password.
pub async fn login(
    email: &str,
    password: &str,
) -> Result<AuthResponse, String> {
    let body = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };
    post_json(endpoints::LOGIN, &body).await
}

/// Register a new account.
pub async fn register(
    request: &RegisterRequest,
) -> Result<AuthResponse, String> {
    post_json(endpoints::REGISTER, request).await
}

/// Sign out — clears the server-side session cookie.
pub async fn logout() -> Result<(), String> {
    let response = Request::post(endpoints::LOGOUT)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_err(|err| format!("Network error: {err}"))?;
    if response.ok() {
        Ok(())
    } else {
        Err(format!(
            "Logout failed ({})",
            response.status()
        ))
    }
}
