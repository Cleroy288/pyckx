//! App management handlers (admin)

use crate::api::dto::{AppSuccessResponse, CreateAppRequest, SuccessResponse, UpdateAppRequest};
use crate::api::validation::validate_request;
use crate::app::App;
use crate::error::AppResult;
use actix_web::{delete, post, put, web, HttpResponse};
use tracing::instrument;

/// POST /api/apps
/// Create a new app (admin only - TODO: add admin check)
#[post("")]
#[instrument(skip(app, body))]
pub async fn create_app_handler(
    app: web::Data<App>,
    body: web::Json<CreateAppRequest>,
) -> AppResult<HttpResponse> {
    validate_request(&body.0)?;

    let app_entity = app
        .app_service
        .create_app(&body.name, body.description.clone())
        .await?;

    Ok(HttpResponse::Created().json(AppSuccessResponse::created(app_entity)))
}

/// PUT /api/apps/{name}
/// Update an app (admin only - TODO: add admin check)
#[put("/{name}")]
#[instrument(skip(app, body), fields(name = %path.as_str()))]
pub async fn update_app_handler(
    app: web::Data<App>,
    path: web::Path<String>,
    body: web::Json<UpdateAppRequest>,
) -> AppResult<HttpResponse> {
    validate_request(&body.0)?;

    let name = path.into_inner();
    let app_entity = app
        .app_service
        .update_app(&name, body.description.clone())
        .await?;

    Ok(HttpResponse::Ok().json(AppSuccessResponse::updated(app_entity)))
}

/// DELETE /api/apps/{name}
/// Delete an app (admin only - TODO: add admin check)
#[delete("/{name}")]
#[instrument(skip(app), fields(name = %path.as_str()))]
pub async fn delete_app_handler(
    app: web::Data<App>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let name = path.into_inner();
    app.app_service.delete_app(&name).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::deleted("App")))
}
