//! Apps DTO Conversions

use super::response::{
    AppListResponse, AppResponse, AppSuccessResponse, UserAppResponse, UserAppSuccessResponse,
};
use crate::services::app_registry::registry_domain::{App as AppEntity, UserApp};

impl From<AppEntity> for AppResponse {
    fn from(app: AppEntity) -> Self {
        Self {
            id: app.id,
            name: app.name,
            description: app.description,
            created_at: app.created_at.to_rfc3339(),
            updated_at: app.updated_at.to_rfc3339(),
        }
    }
}

impl From<&AppEntity> for AppResponse {
    fn from(app: &AppEntity) -> Self {
        Self {
            id: app.id,
            name: app.name.clone(),
            description: app.description.clone(),
            created_at: app.created_at.to_rfc3339(),
            updated_at: app.updated_at.to_rfc3339(),
        }
    }
}

impl AppListResponse {
    pub fn from_apps(apps: Vec<AppEntity>) -> Self {
        let count = apps.len();
        Self {
            apps: apps.into_iter().map(AppResponse::from).collect(),
            count,
        }
    }
}

impl From<UserApp> for UserAppResponse {
    fn from(user_app: UserApp) -> Self {
        Self {
            id: user_app.id,
            app_id: user_app.app_id,
            added_at: user_app.created_at.to_rfc3339(),
        }
    }
}

impl AppSuccessResponse {
    pub fn created(app: AppEntity) -> Self {
        Self {
            message: "App created successfully".to_string(),
            app: AppResponse::from(app),
        }
    }

    pub fn updated(app: AppEntity) -> Self {
        Self {
            message: "App updated successfully".to_string(),
            app: AppResponse::from(app),
        }
    }
}

impl UserAppSuccessResponse {
    pub fn added(user_app: UserApp) -> Self {
        Self {
            message: "App added to user list".to_string(),
            user_app: UserAppResponse::from(user_app),
        }
    }
}
