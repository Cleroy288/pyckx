//! DVD handlers
//!
//! Add, get, update, delete DVD operations.

use super::helpers::validate_request;
use crate::app::App;
use crate::http_api::data_transfer_object::{
    AddDvdRequest, DeleteResponse, DvdListResponse, DvdResponse, DvdSuccessResponse,
    UpdateDvdRequest,
};
use crate::infra::user::get_user_id_from_session;
use crate::shared::{AppError, AppResult};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /api/collection/dvds
///
/// Add a new DVD to the user's collection.
#[post("/dvds")]
#[instrument(skip(app, req, body))]
pub async fn add_dvd_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<AddDvdRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    validate_request(&body.0)?;

    let year = body
        .parse_year()
        .map_err(|e| AppError::validation("year", e))?;

    // Call service directly
    let dvd = app
        .collection_service
        .add_dvd(
            &user_id,
            body.name.clone(),
            year,
            body.realisator.clone(),
            body.actors.clone(),
            body.genre.clone(),
        )
        .await?;

    Ok(HttpResponse::Created().json(DvdSuccessResponse::created(dvd)))
}

/// GET /api/collection/dvds
///
/// Get all DVDs for the current user.
#[get("/dvds")]
#[instrument(skip(app, req))]
pub async fn get_user_dvds_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Call service directly
    let dvds = app.collection_service.get_user_dvds(&user_id).await?;

    Ok(HttpResponse::Ok().json(DvdListResponse::from_dvds(dvds)))
}

/// GET /api/collection/dvds/{dvd_id}
///
/// Get a specific DVD by ID.
#[get("/dvds/{dvd_id}")]
#[instrument(skip(app, req), fields(dvd_id = %path.as_str()))]
pub async fn get_dvd_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let dvd_id = path.into_inner();

    // Call service directly
    let dvd = app.collection_service.find_dvd(&user_id, &dvd_id).await?;

    Ok(HttpResponse::Ok().json(DvdResponse::from(dvd)))
}

/// PUT /api/collection/dvds/{dvd_id}
///
/// Update a DVD's information.
#[put("/dvds/{dvd_id}")]
#[instrument(skip(app, req, body), fields(dvd_id = %path.as_str()))]
pub async fn update_dvd_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateDvdRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    validate_request(&body.0)?;

    let year = body
        .parse_year()
        .map_err(|e| AppError::validation("year", e))?;

    let dvd_id = path.into_inner();

    // Call service directly
    let dvd = app
        .collection_service
        .update_dvd(
            &user_id,
            &dvd_id,
            body.name.clone(),
            year,
            body.realisator.clone(),
            body.actors.clone(),
            body.genre.clone(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(DvdSuccessResponse::updated(dvd)))
}

/// DELETE /api/collection/dvds/{dvd_id}
///
/// Delete a DVD from the user's collection.
#[delete("/dvds/{dvd_id}")]
#[instrument(skip(app, req), fields(dvd_id = %path.as_str()))]
pub async fn delete_dvd_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let dvd_id = path.into_inner();

    // Call service directly
    app.collection_service.delete_dvd(&user_id, &dvd_id).await?;

    Ok(HttpResponse::Ok().json(DeleteResponse::dvd_success()))
}
