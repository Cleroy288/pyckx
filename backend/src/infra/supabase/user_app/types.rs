//! User app repository types - Row structs for database operations

use crate::services::app_registry::registry_domain::UserApp;
use crate::shared::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct InsertUserAppRow {
    pub user_id: String,
    pub app_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UserAppRow {
    pub id: i32,
    pub user_id: String,
    pub app_id: i32,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl TryFrom<UserAppRow> for UserApp {
    type Error = AppError;

    fn try_from(row: UserAppRow) -> Result<Self, Self::Error> {
        use chrono::{DateTime, Utc};

        let created_at = match row.created_at {
            Some(ts) => DateTime::parse_from_rfc3339(&ts)
                .map_err(|err| {
                    AppError::Internal(
                        crate::http_api::utils::InternalError::new(format!(
                            "Invalid created_at: {}",
                            err
                        )),
                    )
                })?
                .with_timezone(&Utc),
            None => Utc::now(),
        };

        Ok(UserApp {
            id: row.id,
            user_id: row.user_id,
            app_id: row.app_id,
            created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_app_row_valid_created_at() {
        // arrange
        let row = UserAppRow {
            id: 1,
            user_id: "user-abc".into(),
            app_id: 5,
            created_at: Some(
                "2024-03-15T10:00:00+00:00".into(),
            ),
        };

        // act
        let result = UserApp::try_from(row);

        // assert
        assert!(result.is_ok());
        let ua = result.unwrap();
        assert_eq!(ua.id, 1);
        assert_eq!(ua.app_id, 5);
    }

    #[test]
    fn test_user_app_row_none_created_at_uses_now() {
        // arrange
        let row = UserAppRow {
            id: 2,
            user_id: "user-xyz".into(),
            app_id: 3,
            created_at: None,
        };

        // act
        let result = UserApp::try_from(row);

        // assert
        assert!(result.is_ok());
        let ua = result.unwrap();
        assert_eq!(ua.user_id, "user-xyz");
    }

    #[test]
    fn test_user_app_row_invalid_created_at() {
        // arrange
        let row = UserAppRow {
            id: 3,
            user_id: "user".into(),
            app_id: 1,
            created_at: Some("not-a-date".into()),
        };

        // act
        let result = UserApp::try_from(row);

        // assert
        assert!(result.is_err());
    }
}
