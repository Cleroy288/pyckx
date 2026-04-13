//! User apps handlers

use crate::app::App;
use crate::http_api::data_transfer_object::{
    AddUserAppRequest, AppListResponse, SuccessResponse, UserAppSuccessResponse,
};
use crate::http_api::utils::validation::validate_request;
use crate::infra::user::get_user_id_from_session;
use crate::shared::AppResult;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// GET /api/user/apps
/// Get user's enabled apps
#[get("")]
#[instrument(skip(app, req))]
pub async fn get_user_apps_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let apps = app.app_service.get_user_apps(&user_id).await?;
    Ok(HttpResponse::Ok().json(AppListResponse::from_apps(apps)))
}

/// POST /api/user/apps
/// Add an app to user's list
#[post("")]
#[instrument(skip(app, req, body))]
pub async fn add_user_app_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<AddUserAppRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    validate_request(&body.0)?;

    let user_app = app
        .app_service
        .add_user_app(&user_id, &body.app_name)
        .await?;

    Ok(HttpResponse::Created().json(UserAppSuccessResponse::added(user_app)))
}

/// DELETE /api/user/apps/{app_name}
/// Remove an app from user's list
#[delete("/{app_name}")]
#[instrument(skip(app, req), fields(app_name = %path.as_str()))]
pub async fn remove_user_app_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let app_name = path.into_inner();

    app.app_service.remove_user_app(&user_id, &app_name).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::removed("App")))
}
