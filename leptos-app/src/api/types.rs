//! API Types - Request/Response DTOs matching backend

use serde::{Deserialize, Serialize};

/// Login request body
#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Auth response from backend (matches backend's AuthResponse)
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuthResponse {
    pub username: String,
    pub email: String,
    pub role: String,
}

/// Generic API error response
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    pub error: String,
}

