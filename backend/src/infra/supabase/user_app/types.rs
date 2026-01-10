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
                .map_err(|e| {
                    AppError::Internal(crate::http_api::utils::InternalError::new(format!(
                        "Invalid created_at: {}",
                        e
                    )))
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
