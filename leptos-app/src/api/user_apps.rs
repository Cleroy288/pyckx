//! User apps API — app discovery and management

use super::{endpoints, helpers};
use crate::domain::app_types::*;

/// Get all available apps
pub async fn get_all_apps(
) -> Result<Vec<AppData>, String> {
    let resp: AppListResponse =
        helpers::get_json(endpoints::ALL_APPS).await?;
    Ok(resp.apps)
}

/// Get user's subscribed apps (returns full app data)
pub async fn get_user_apps(
) -> Result<Vec<AppData>, String> {
    let resp: AppListResponse =
        helpers::get_json(endpoints::USER_APPS).await?;
    Ok(resp.apps)
}

/// Add an app to user's list
pub async fn add_user_app(
    app_name: &str,
) -> Result<UserAppSuccessResponse, String> {
    let req = AddUserAppRequest {
        app_name: app_name.to_string(),
    };
    helpers::post_json(endpoints::USER_APPS, &req).await
}

/// Remove an app from user's list by name
pub async fn remove_user_app(
    app_name: &str,
) -> Result<bool, String> {
    let url =
        format!("{}/{}", endpoints::USER_APPS, app_name);
    helpers::delete_request(&url).await
}
