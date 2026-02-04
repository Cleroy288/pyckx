//! Auth API - Login/Logout/Session HTTP requests

use super::types::{ApiError, AuthResponse, LoginRequest};
use gloo_net::http::Request;

/// Get current user from session cookie
/// Returns AuthResponse if session is valid, None if not authenticated
pub async fn get_me() -> Option<AuthResponse> {
    let response = Request::get("/api/user/me")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .ok()?;

    if response.ok() {
        response.json::<AuthResponse>().await.ok()
    } else {
        None
    }
}

/// Login with email and password
/// Returns AuthResponse on success, error message on failure
pub async fn login(email: &str, password: &str) -> Result<AuthResponse, String> {
    let body = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = Request::post("/api/auth/login")
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&body)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        // Try to parse error message from backend
        let error_msg = response
            .json::<ApiError>()
            .await
            .map(|e| e.error)
            .unwrap_or_else(|_| format!("Login failed with status: {}", response.status()));
        Err(error_msg)
    }
}

/// Logout - clears session cookie
pub async fn logout() -> Result<(), String> {
    let response = Request::post("/api/auth/logout")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        Ok(())
    } else {
        Err(format!("Logout failed with status: {}", response.status()))
    }
}

