//! DVD handlers
//!
//! Add, get, update, delete DVD operations.

use super::helpers::validate_request;
use crate::api::dto::{
    AddDvdRequest, DeleteResponse, DvdListResponse, DvdResponse, DvdSuccessResponse,
    UpdateDvdRequest,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::shared::session::get_user_id_from_session;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /app/collection/add-dvd
///
/// Add a new DVD to the user's collection.
#[post("/add-dvd")]
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

/// GET /app/collection/get-user-dvd
///
/// Get all DVDs for the current user.
#[get("/get-user-dvd")]
#[instrument(skip(app, req))]
pub async fn get_user_dvds_handler(app: web::Data<App>, req: HttpRequest) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let dvds = app.collection_service.get_user_dvds(&user_id).await?;

    Ok(HttpResponse::Ok().json(DvdListResponse::from_dvds(dvds)))
}

/// GET /app/collection/dvd/{dvd_id}
///
/// Get a specific DVD by ID.
#[get("/dvd/{dvd_id}")]
#[instrument(skip(app, req), fields(dvd_id = %path.as_str()))]
pub async fn get_dvd_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let dvd_id = path.into_inner();
    let dvd = app.collection_service.find_dvd(&user_id, &dvd_id).await?;

    Ok(HttpResponse::Ok().json(DvdResponse::from(dvd)))
}

/// PUT /app/collection/mod-dvd/{dvd_id}
///
/// Update a DVD's information.
#[put("/mod-dvd/{dvd_id}")]
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

/// DELETE /app/collection/del/{dvd_id}
///
/// Delete a DVD from the user's collection.
#[delete("/del/{dvd_id}")]
#[instrument(skip(app, req), fields(dvd_id = %path.as_str()))]
pub async fn delete_dvd_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let dvd_id = path.into_inner();
    app.collection_service.delete_dvd(&user_id, &dvd_id).await?;

    Ok(HttpResponse::Ok().json(DeleteResponse::dvd_success()))
}
