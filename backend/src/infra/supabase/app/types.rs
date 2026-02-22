//! App repository types - Row structs for database operations

use crate::infra::database::{CreateApp, UpdateApp};
use crate::services::app_registry::registry_domain::App as AppEntity;
use crate::shared::AppError;
use serde::{Deserialize, Serialize};

// ============================================================================
// INSERT/UPDATE TYPES
// ============================================================================

#[derive(Debug, Serialize)]
pub struct InsertAppRow {
    pub name: String,
    pub description: Option<String>,
}

impl From<&CreateApp> for InsertAppRow {
    fn from(app: &CreateApp) -> Self {
        Self {
            name: app.name.clone(),
            description: app.description.clone(),
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateAppRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<&UpdateApp> for UpdateAppRow {
    fn from(update: &UpdateApp) -> Self {
        Self {
            name: update.name.clone(),
            description: update.description.clone(),
        }
    }
}

// ============================================================================
// SELECT TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct AppRow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl TryFrom<AppRow> for AppEntity {
    type Error = AppError;

    fn try_from(row: AppRow) -> Result<Self, Self::Error> {
        use chrono::DateTime;

        let created_at = DateTime::parse_from_rfc3339(&row.created_at)
            .map_err(|err| {
                AppError::Internal(crate::http_api::utils::InternalError::new(
                    format!("Invalid created_at: {}", err),
                ))
            })?
            .with_timezone(&chrono::Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|err| {
                AppError::Internal(crate::http_api::utils::InternalError::new(
                    format!("Invalid updated_at: {}", err),
                ))
            })?
            .with_timezone(&chrono::Utc);

        Ok(AppEntity {
            id: row.id,
            name: row.name,
            description: row.description,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- InsertAppRow from CreateApp --

    #[test]
    fn test_insert_app_row_from_create_app() {
        // arrange
        let create = CreateApp {
            name: "TestApp".into(),
            description: Some("Desc".into()),
        };

        // act
        let row = InsertAppRow::from(&create);

        // assert
        assert_eq!(row.name, "TestApp");
        assert_eq!(row.description, Some("Desc".into()));
    }

    // -- UpdateAppRow from UpdateApp --

    #[test]
    fn test_update_app_row_from_update_app() {
        // arrange
        let update = UpdateApp {
            name: Some("New".into()),
            description: None,
        };

        // act
        let row = UpdateAppRow::from(&update);

        // assert
        assert_eq!(row.name, Some("New".into()));
        assert!(row.description.is_none());
    }

    // -- AppRow TryFrom to AppEntity --

    #[test]
    fn test_app_row_try_from_valid_dates() {
        // arrange
        let row = AppRow {
            id: 1,
            name: "MyApp".into(),
            description: None,
            created_at: "2024-01-01T00:00:00+00:00".into(),
            updated_at: "2024-06-15T12:30:00+00:00".into(),
        };

        // act
        let result = AppEntity::try_from(row);

        // assert
        assert!(result.is_ok());
        let app = result.unwrap();
        assert_eq!(app.id, 1);
        assert_eq!(app.name, "MyApp");
    }

    #[test]
    fn test_app_row_try_from_invalid_created_at() {
        // arrange
        let row = AppRow {
            id: 1,
            name: "Bad".into(),
            description: None,
            created_at: "not-a-date".into(),
            updated_at: "2024-01-01T00:00:00+00:00".into(),
        };

        // act
        let result = AppEntity::try_from(row);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_app_row_try_from_invalid_updated_at() {
        // arrange
        let row = AppRow {
            id: 1,
            name: "Bad".into(),
            description: None,
            created_at: "2024-01-01T00:00:00+00:00".into(),
            updated_at: "bad-date".into(),
        };

        // act
        let result = AppEntity::try_from(row);

        // assert
        assert!(result.is_err());
    }
}
