use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::App;
use crate::infra::user::get_user_id_from_session;
use crate::shared::AppError;

/// Body for creating a new user resource.
#[derive(Debug, Deserialize)]
pub struct CreateUserResource {
    pub filename: String,
    pub content: String,
    pub token_count: i32,
}

/// Body for linking a resource to a course.
#[derive(Debug, Deserialize)]
pub struct LinkResourceRequest {
    pub resource_id: String,
}

/// Query params for checking resource existence.
#[derive(Debug, Deserialize)]
pub struct CheckResourceQuery {
    pub filename: String,
}

/// Summary view of a user resource (no content).
#[derive(Debug, Serialize)]
pub struct UserResourceSummary {
    pub id: String,
    pub filename: String,
    pub token_count: i32,
    pub created_at: String,
}

/// Full view of a user resource (with content).
#[derive(Debug, Serialize)]
pub struct UserResourceFull {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub content: String,
    pub token_count: i32,
    pub created_at: String,
}

/// Response wrapper for a list of resources.
#[derive(Debug, Serialize)]
pub struct UserResourcesResponse {
    pub success: bool,
    pub resources: Vec<UserResourceSummary>,
}

/// Response wrapper for a single resource.
#[derive(Debug, Serialize)]
pub struct UserResourceResponse {
    pub success: bool,
    pub resource: UserResourceFull,
}

/// Response for resource existence check.
#[derive(Debug, Serialize)]
pub struct ResourceExistsResponse {
    pub success: bool,
    pub exists: bool,
}

/// Generic success response.
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

/// GET /api/study/resources
pub async fn list_user_resources(
    app: web::Data<App>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id =
        get_user_id_from_session(&app, &req).await?;

    info!(
        user_id = %user_id,
        "Listing user resources"
    );

    let resources = app
        .study_service
        .list_user_resources(&user_id)
        .await?;

    let resources: Vec<UserResourceSummary> =
        resources
            .into_iter()
            .map(|r| UserResourceSummary {
                id: r.id,
                filename: r.filename,
                token_count: r.token_count,
                created_at: r.created_at,
            })
            .collect();

    Ok(HttpResponse::Ok().json(
        UserResourcesResponse {
            success: true,
            resources,
        },
    ))
}

/// GET /api/study/resources/{id}
pub async fn get_resource_content(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id =
        get_user_id_from_session(&app, &req).await?;
    let resource_id = path.into_inner();

    info!(
        user_id = %user_id,
        resource_id = %resource_id,
        "Getting resource content"
    );

    let resource = app
        .study_service
        .get_resource_content(
            &user_id, &resource_id,
        )
        .await?;

    Ok(HttpResponse::Ok().json(
        UserResourceResponse {
            success: true,
            resource: UserResourceFull {
                id: resource.id,
                user_id: resource.user_id,
                filename: resource.filename,
                content: resource.content,
                token_count: resource.token_count,
                created_at: resource.created_at,
            },
        },
    ))
}

/// GET /api/study/resources/check?filename=X
pub async fn check_resource_exists(
    app: web::Data<App>,
    req: HttpRequest,
    query: web::Query<CheckResourceQuery>,
) -> Result<HttpResponse, AppError> {
    let user_id =
        get_user_id_from_session(&app, &req).await?;

    info!(
        user_id = %user_id,
        filename = %query.filename,
        "Checking resource existence"
    );

    let exists = app
        .study_service
        .resource_exists(&user_id, &query.filename)
        .await?;

    Ok(HttpResponse::Ok().json(
        ResourceExistsResponse {
            success: true,
            exists,
        },
    ))
}

/// POST /api/study/resources
pub async fn create_user_resource(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CreateUserResource>,
) -> Result<HttpResponse, AppError> {
    let user_id =
        get_user_id_from_session(&app, &req).await?;

    info!(
        user_id = %user_id,
        filename = %body.filename,
        "Creating user resource"
    );

    let resource = app
        .study_service
        .create_resource(
            &user_id,
            body.filename.clone(),
            body.content.clone(),
            body.token_count,
        )
        .await?;

    Ok(HttpResponse::Created().json(
        UserResourceResponse {
            success: true,
            resource: UserResourceFull {
                id: resource.id,
                user_id: resource.user_id,
                filename: resource.filename,
                content: resource.content,
                token_count: resource.token_count,
                created_at: resource.created_at,
            },
        },
    ))
}

/// POST /api/study/courses/{id}/resources/link
pub async fn link_resource(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<LinkResourceRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id =
        get_user_id_from_session(&app, &req).await?;
    let course_id = path.into_inner();

    info!(
        user_id = %user_id,
        course_id = %course_id,
        resource_id = %body.resource_id,
        "Linking resource to course"
    );

    app.study_service
        .link_resource_to_course(
            &user_id,
            &course_id,
            &body.resource_id,
        )
        .await?;

    Ok(HttpResponse::Ok().json(
        SuccessResponse { success: true },
    ))
}
