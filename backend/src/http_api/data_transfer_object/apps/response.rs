//! Apps Response DTOs

use serde::Serialize;

/// Response for a single app
#[derive(Debug, Serialize)]
pub struct AppResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Response for app list
#[derive(Debug, Serialize)]
pub struct AppListResponse {
    pub apps: Vec<AppResponse>,
    pub count: usize,
}

/// Response for user app
#[derive(Debug, Serialize)]
pub struct UserAppResponse {
    pub id: i32,
    pub app_id: i32,
    pub added_at: String,
}

/// Response for successful app operations
#[derive(Debug, Serialize)]
pub struct AppSuccessResponse {
    pub message: String,
    pub app: AppResponse,
}

/// Response for successful user app operations
#[derive(Debug, Serialize)]
pub struct UserAppSuccessResponse {
    pub message: String,
    pub user_app: UserAppResponse,
}

/// Generic success response
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub message: String,
    pub success: bool,
}

impl SuccessResponse {
    pub fn deleted(entity: &str) -> Self {
        Self {
            message: format!("{} deleted successfully", entity),
            success: true,
        }
    }

    pub fn removed(entity: &str) -> Self {
        Self {
            message: format!("{} removed successfully", entity),
            success: true,
        }
    }
}
