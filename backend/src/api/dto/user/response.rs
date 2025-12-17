//! User Response DTOs

use crate::domain::User;
use serde::Serialize;

/// Safe user response - NO tokens exposed
#[derive(Serialize)]
pub struct UserResponse {
    pub email: String,
    pub username: String,
    pub role: String,
}

impl From<&User> for UserResponse {
    fn from(u: &User) -> Self {
        Self {
            email: u.email.clone(),
            username: u.username.clone(),
            role: u.role.clone(),
        }
    }
}
