//! Apps DTO Conversions

use super::response::{
    AppListResponse, AppResponse, AppSuccessResponse, UserAppResponse,
    UserAppSuccessResponse,
};
use crate::services::app_registry::registry_domain::{
    App as AppEntity, UserApp,
};

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a test AppEntity with known values
    fn test_app() -> AppEntity {
        AppEntity::new(1, "TestApp", Some("A description".into()))
    }

    /// Build a test UserApp with known values
    fn test_user_app() -> UserApp {
        UserApp::new(10, "user-abc", 1)
    }

    #[test]
    fn test_app_response_from_app_maps_id() {
        // arrange
        let app = test_app();

        // act
        let resp = AppResponse::from(app);

        // assert
        assert_eq!(resp.id, 1);
    }

    #[test]
    fn test_app_response_from_app_maps_name() {
        // arrange
        let app = test_app();

        // act
        let resp = AppResponse::from(app);

        // assert
        assert_eq!(resp.name, "TestApp");
    }

    #[test]
    fn test_app_response_from_app_maps_description() {
        // arrange
        let app = test_app();

        // act
        let resp = AppResponse::from(app);

        // assert
        assert_eq!(resp.description, Some("A description".into()));
    }

    #[test]
    fn test_app_response_from_ref_maps_fields() {
        // arrange
        let app = test_app();

        // act
        let resp = AppResponse::from(&app);

        // assert
        assert_eq!(resp.id, 1);
        assert_eq!(resp.name, "TestApp");
    }

    #[test]
    fn test_app_list_response_from_apps_count() {
        // arrange
        let apps = vec![test_app(), test_app()];

        // act
        let resp = AppListResponse::from_apps(apps);

        // assert
        assert_eq!(resp.count, 2);
    }

    #[test]
    fn test_app_list_response_from_empty_apps() {
        // arrange
        let apps: Vec<AppEntity> = vec![];

        // act
        let resp = AppListResponse::from_apps(apps);

        // assert
        assert_eq!(resp.count, 0);
        assert!(resp.apps.is_empty());
    }

    #[test]
    fn test_user_app_response_from_maps_fields() {
        // arrange
        let ua = test_user_app();
        let created_at = ua.created_at.to_rfc3339();

        // act
        let resp = UserAppResponse::from(ua);

        // assert
        assert_eq!(resp.id, 10);
        assert_eq!(resp.app_id, 1);
        assert_eq!(resp.added_at, created_at);
    }

    #[test]
    fn test_app_success_created_message() {
        // arrange
        let app = test_app();

        // act
        let resp = AppSuccessResponse::created(app);

        // assert
        assert_eq!(resp.message, "App created successfully");
    }

    #[test]
    fn test_app_success_updated_message() {
        // arrange
        let app = test_app();

        // act
        let resp = AppSuccessResponse::updated(app);

        // assert
        assert_eq!(resp.message, "App updated successfully");
    }

    #[test]
    fn test_user_app_success_added_message() {
        // arrange
        let ua = test_user_app();

        // act
        let resp = UserAppSuccessResponse::added(ua);

        // assert
        assert_eq!(resp.message, "App added to user list");
    }

    #[test]
    fn test_app_response_timestamps_are_rfc3339() {
        // arrange
        let app = test_app();
        let created = app.created_at.to_rfc3339();
        let updated = app.updated_at.to_rfc3339();

        // act
        let resp = AppResponse::from(app);

        // assert
        assert_eq!(resp.created_at, created);
        assert_eq!(resp.updated_at, updated);
    }
}
