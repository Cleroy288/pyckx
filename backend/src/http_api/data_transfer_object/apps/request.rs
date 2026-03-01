//! Apps Request DTOs

use serde::Deserialize;
use validator::Validate;

/// Request body for creating a new app
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAppRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be 1-100 characters"
    ))]
    pub name: String,
    pub description: Option<String>,
}

/// Request body for updating an app
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAppRequest {
    pub description: Option<String>,
}

/// Request body for adding an app to user's list
#[derive(Debug, Deserialize, Validate)]
pub struct AddUserAppRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "App name must be 1-100 characters"
    ))]
    pub app_name: String,
}
