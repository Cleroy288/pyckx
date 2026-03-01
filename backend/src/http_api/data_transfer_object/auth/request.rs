//! Auth Request DTOs

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email too long"))]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 128,
        message = "Password must be 6-128 characters"
    ))]
    pub password: String,
}

// Registration is currently disabled but kept for future use
#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email too long"))]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 128,
        message = "Password must be 6-128 characters"
    ))]
    pub password: String,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be 3-50 characters"
    ))]
    pub username: String,

    #[validate(length(max = 5, message = "Country code too long"))]
    pub phone_country_code: Option<String>,

    #[validate(length(max = 20, message = "Phone number too long"))]
    pub phone_number: Option<String>,
}
