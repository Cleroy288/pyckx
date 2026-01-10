//! Get apps handlers

use crate::app::App;
use crate::http_api::data_transfer_object::{AppListResponse, AppResponse};
use crate::shared::AppResult;
use actix_web::{get, web, HttpResponse};
use tracing::instrument;

/// GET /api/apps
/// List all available apps
#[get("")]
#[instrument(skip(app))]
pub async fn get_all_apps_handler(app: web::Data<App>) -> AppResult<HttpResponse> {
    let apps = app.app_service.get_all_apps().await?;
    Ok(HttpResponse::Ok().json(AppListResponse::from_apps(apps)))
}

/// GET /api/apps/{name}
/// Get app by name
#[get("/{name}")]
#[instrument(skip(app), fields(name = %path.as_str()))]
pub async fn get_app_handler(
    app: web::Data<App>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let name = path.into_inner();
    let app_entity = app.app_service.get_app(&name).await?;
    Ok(HttpResponse::Ok().json(AppResponse::from(app_entity)))
}
