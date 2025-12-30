//! App repository types - Row structs for database operations

use crate::services::app_registry::registry_domain::App as AppEntity;
use crate::shared::AppError;
use crate::infra::database::{CreateApp, UpdateApp};
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
            .map_err(|e| AppError::Internal(crate::http_api::utils::InternalError::new(format!("Invalid created_at: {}", e))))?
            .with_timezone(&chrono::Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|e| AppError::Internal(crate::http_api::utils::InternalError::new(format!("Invalid updated_at: {}", e))))?
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
