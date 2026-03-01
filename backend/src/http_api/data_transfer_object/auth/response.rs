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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::user::UserId;

    /// Build a test User with known values
    fn test_user() -> User {
        User {
            id: UserId::from("user-123"),
            email: "alice@example.com".into(),
            username: "alice".into(),
            role: "admin".into(),
            access_token: "secret-at".into(),
            refresh_token: "secret-rt".into(),
            expires_at: 9999,
        }
    }

    #[test]
    fn test_from_user_maps_username() {
        // arrange
        let user = test_user();

        // act
        let resp = AuthResponse::from_user(&user);

        // assert
        assert_eq!(resp.username, "alice");
    }

    #[test]
    fn test_from_user_maps_email() {
        // arrange
        let user = test_user();

        // act
        let resp = AuthResponse::from_user(&user);

        // assert
        assert_eq!(resp.email, "alice@example.com");
    }

    #[test]
    fn test_from_user_maps_role() {
        // arrange
        let user = test_user();

        // act
        let resp = AuthResponse::from_user(&user);

        // assert
        assert_eq!(resp.role, "admin");
    }

    #[test]
    fn test_from_user_does_not_expose_tokens() {
        // arrange
        let user = test_user();

        // act
        let resp = AuthResponse::from_user(&user);

        // assert - no token fields on AuthResponse
        assert_ne!(resp.username, user.access_token);
        assert_ne!(resp.email, user.refresh_token);
    }
}
