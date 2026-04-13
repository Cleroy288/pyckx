//! Course CRUD Handlers
//!
//! HTTP handlers for course and resource management.

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::info;

use crate::app::App;
use crate::infra::user::get_user_id_from_session;
use crate::shared::AppError;

use super::course_dto::{
    Course, CourseListResponse, CourseResponse,
    Resource, ResourceListResponse,
};

/// POST /api/study/courses
pub async fn create_course(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<super::course_dto::CreateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    info!(user_id = %user_id, name = %body.name, "Creating course");

    let input =
        crate::services::course::domain::CreateCourseInput {
            name: body.name.clone(),
            description: body.description.clone(),
        };

    let c = app
        .study_service
        .create_course(&user_id, input)
        .await?;

    Ok(HttpResponse::Created().json(CourseResponse {
        success: true,
        course: map_course(c),
    }))
}

/// GET /api/study/courses
pub async fn list_courses(
    app: web::Data<App>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    info!(user_id = %user_id, "Listing courses");

    let courses = app
        .study_service
        .list_user_courses(&user_id)
        .await?;

    let items: Vec<Course> =
        courses.into_iter().map(map_course).collect();

    Ok(HttpResponse::Ok().json(CourseListResponse {
        success: true,
        courses: items,
    }))
}

/// DELETE /api/study/courses/{id}
pub async fn delete_course(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let course_id = path.into_inner();
    info!(user_id = %user_id, course_id = %course_id, "Deleting course");

    app.study_service
        .delete_course(&user_id, &course_id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Course deleted successfully"
    })))
}

/// POST /api/study/courses/{id}/resources
pub async fn upload_resource(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    payload: actix_multipart::Multipart,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let course_id = path.into_inner();
    info!(user_id = %user_id, course_id = %course_id, "Uploading resources");

    let documents =
        super::helpers::parse_multipart_files_only(payload).await?;
    let mut created = Vec::new();

    for (filename, content, token_count) in documents {
        let r = app
            .study_service
            .create_resource(
                &user_id, filename, content, token_count as i32,
            )
            .await?;
        app.study_service
            .link_resource_to_course(
                &user_id, &course_id, &r.id,
            )
            .await?;
        created.push(map_resource(r));
    }

    Ok(HttpResponse::Created().json(ResourceListResponse {
        success: true,
        resources: created,
    }))
}

/// GET /api/study/courses/{id}/resources
pub async fn get_resources(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let course_id = path.into_inner();
    info!(user_id = %user_id, course_id = %course_id, "Getting resources");

    let resources = app
        .study_service
        .list_course_resources(&user_id, &course_id)
        .await?;

    let items: Vec<Resource> =
        resources.into_iter().map(map_resource).collect();

    Ok(HttpResponse::Ok().json(ResourceListResponse {
        success: true,
        resources: items,
    }))
}

/// Maps a service-level course to the DTO.
fn map_course(
    c: crate::services::course::domain::Course,
) -> Course {
    Course {
        id: c.id,
        user_id: c.user_id,
        name: c.name,
        description: c.description,
        created_at: c.created_at,
        updated_at: c.updated_at,
    }
}

/// Maps a service-level resource to the DTO.
fn map_resource(
    r: crate::services::course::domain::UserResource,
) -> Resource {
    Resource {
        id: r.id,
        user_id: r.user_id,
        filename: r.filename,
        content: r.content,
        token_count: r.token_count,
        created_at: r.created_at,
    }
}
