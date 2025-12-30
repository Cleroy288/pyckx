//! Auth Response DTOs

use crate::infra::User;
use serde::Serialize;

/// Safe auth response - NO tokens exposed
#[derive(Serialize)]
pub struct AuthResponse {
    pub username: String,
    pub email: String,
    pub role: String,
}

impl AuthResponse {
    pub fn from_user(u: &User) -> Self {
        Self {
            username: u.username.clone(),
            email: u.email.clone(),
            role: u.role.clone(),
        }
    }
}
