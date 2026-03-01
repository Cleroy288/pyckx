// == NEW USER RESOURCE HANDLERS ==

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::App;
use crate::infra::user::get_user_id_from_session;
use crate::shared::AppError;

// -- Request DTOs --

#[derive(Debug, Deserialize)]
pub struct CreateUserResource {
    pub filename: String,
    pub content: String,
    pub token_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct LinkResourceRequest {
    pub resource_id: String,
}

// -- Response DTOs --

#[derive(Debug, Serialize)]
pub struct UserResourceSummary {
    pub id: String,
    pub filename: String,
    pub token_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserResourceFull {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub content: String,
    pub token_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserResourcesResponse {
    pub success: bool,
    pub resources: Vec<UserResourceSummary>,
}

#[derive(Debug, Serialize)]
pub struct UserResourceResponse {
    pub success: bool,
    pub resource: UserResourceFull,
}

#[derive(Debug, Serialize)]
pub struct ResourceExistsResponse {
    pub success: bool,
    pub exists: bool,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

// =============================================================================
// HANDLER: List User Resources
// GET /api/intello/resources
// =============================================================================

pub async fn list_user_resources(
    app: web::Data<App>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(user_id = %user_id, "Listing user resources");

    let resources = app.intello_service.list_user_resources(&user_id).await?;

    let response_resources: Vec<UserResourceSummary> = resources
        .into_iter()
        .map(|r| UserResourceSummary {
            id: r.id,
            filename: r.filename,
            token_count: r.token_count,
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(UserResourcesResponse {
        success: true,
        resources: response_resources,
    }))
}

// =============================================================================
// HANDLER: Get Resource Content
// GET /api/intello/resources/{id}
// =============================================================================

pub async fn get_resource_content(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let resource_id = path.into_inner();

    info!(user_id = %user_id, resource_id = %resource_id, "Getting resource content");

    let resource = app
        .intello_service
        .get_resource_content(&user_id, &resource_id)
        .await?;

    Ok(HttpResponse::Ok().json(UserResourceResponse {
        success: true,
        resource: UserResourceFull {
            id: resource.id,
            user_id: resource.user_id,
            filename: resource.filename,
            content: resource.content,
            token_count: resource.token_count,
            created_at: resource.created_at,
        },
    }))
}

// =============================================================================
// HANDLER: Check Resource Exists
// GET /api/intello/resources/check?filename=X
// =============================================================================

pub async fn check_resource_exists(
    app: web::Data<App>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let filename = query.get("filename").ok_or_else(|| {
        AppError::validation("filename", "Missing filename parameter")
    })?;

    info!(user_id = %user_id, filename = %filename, "Checking resource existence");

    let exists = app
        .intello_service
        .resource_exists(&user_id, filename)
        .await?;

    Ok(HttpResponse::Ok().json(ResourceExistsResponse {
        success: true,
        exists,
    }))
}

// =============================================================================
// HANDLER: Create User Resource
// POST /api/intello/resources
// =============================================================================

pub async fn create_user_resource(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CreateUserResource>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(user_id = %user_id, filename = %body.filename, "Creating user resource");

    let resource = app
        .intello_service
        .create_resource(
            &user_id,
            body.filename.clone(),
            body.content.clone(),
            body.token_count,
        )
        .await?;

    Ok(HttpResponse::Created().json(UserResourceResponse {
        success: true,
        resource: UserResourceFull {
            id: resource.id,
            user_id: resource.user_id,
            filename: resource.filename,
            content: resource.content,
            token_count: resource.token_count,
            created_at: resource.created_at,
        },
    }))
}

// =============================================================================
// HANDLER: Link Resource to Course
// POST /api/intello/courses/{course_id}/resources/link
// =============================================================================

pub async fn link_resource(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<LinkResourceRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, resource_id = %body.resource_id, "Linking resource to course");

    app.intello_service
        .link_resource_to_course(&user_id, &course_id, &body.resource_id)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse { success: true }))
}
