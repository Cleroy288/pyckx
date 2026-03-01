//! Auth domain types — login, register, session

use serde::{Deserialize, Serialize};

/// Login request body
#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Registration request body
#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// Auth response from backend (safe, no tokens)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_auth_response() {
        let json = r#"{
            "username": "alice",
            "email": "a@test.com",
            "role": "user"
        }"#;
        let r: AuthResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(r.username, "alice");
        assert_eq!(r.role, "user");
    }

    #[test]
    fn test_deserialize_api_error() {
        let json = r#"{"error": "not found"}"#;
        let e: ApiError =
            serde_json::from_str(json).unwrap();
        assert_eq!(e.error, "not found");
    }

    #[test]
    fn test_serialize_login_request() {
        let req = LoginRequest {
            email: "a@b.com".into(),
            password: "pw".into(),
        };
        let json =
            serde_json::to_string(&req).unwrap();
        assert!(json.contains("a@b.com"));
    }

    #[test]
    fn test_serialize_register_request_skips_none() {
        let req = RegisterRequest {
            email: "a@b.com".into(),
            password: "pw".into(),
            username: "bob".into(),
            phone_country_code: None,
            phone_number: None,
        };
        let json =
            serde_json::to_string(&req).unwrap();
        assert!(!json.contains("phone"));
    }
}
