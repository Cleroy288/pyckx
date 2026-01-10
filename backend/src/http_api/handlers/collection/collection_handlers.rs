//! Collection handlers
//!
//! Create, get, delete collection operations.

use crate::app::App;
use crate::http_api::data_transfer_object::{
    CollectionItemsResponse, CollectionListResponse, CollectionSuccessResponse,
    CreateCollectionRequest, DeleteResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::services::collection::collection_domain::CollectionItemType;
use crate::shared::{AppError, AppResult};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /app/collection
///
/// Create a new collection for a specific type.
#[post("")]
#[instrument(skip(app, req, body))]
pub async fn create_collection_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CreateCollectionRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let item_type = body
        .parse_type()
        .map_err(|e| AppError::validation("item_type", e))?;

    let collection = app
        .collection_service
        .add_collection(&user_id, item_type)
        .await?;

    Ok(HttpResponse::Created().json(CollectionSuccessResponse::created(collection)))
}

/// GET /app/collection
///
/// Get all collections for the current user.
#[get("")]
#[instrument(skip(app, req))]
pub async fn get_collections_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let collections = app
        .collection_service
        .get_user_collections(&user_id)
        .await?;

    Ok(HttpResponse::Ok().json(CollectionListResponse::from_collections(collections)))
}

/// GET /app/collection/{type}
///
/// Get all items of a specific type (currently only DVDs supported).
#[get("/{item_type}")]
#[instrument(skip(app, req), fields(item_type = %path.as_str()))]
pub async fn get_collection_items_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let item_type_str = path.into_inner();
    let _item_type = CollectionItemType::from_str(&item_type_str).ok_or_else(|| {
        AppError::validation("item_type", format!("Invalid type: {}", item_type_str))
    })?;

    // Currently only DVDs are supported
    let dvds = app.collection_service.get_collection_dvds(&user_id).await?;

    Ok(HttpResponse::Ok().json(CollectionItemsResponse::from_dvds(dvds)))
}

/// DELETE /app/collection/{type}
///
/// Delete a collection and all its items.
#[delete("/{item_type}")]
#[instrument(skip(app, req), fields(item_type = %path.as_str()))]
pub async fn delete_collection_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let item_type_str = path.into_inner();
    let item_type = CollectionItemType::from_str(&item_type_str).ok_or_else(|| {
        AppError::validation("item_type", format!("Invalid type: {}", item_type_str))
    })?;

    app.collection_service
        .delete_collection(&user_id, item_type)
        .await?;

    Ok(HttpResponse::Ok().json(DeleteResponse::collection_success()))
}
