//! Course CRUD Handlers
//!
//! Handlers for course and resource management using the service layer pattern.

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::App;
use crate::shared::AppError;

// =============================================================================
// DTOs
// =============================================================================

// -- Requests --

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadResourceRequest {
    pub filename: String,
    pub content: String,
    #[serde(default)]
    pub token_count: i32,
}

// -- Responses --

#[derive(Debug, Serialize)]
pub struct Course {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct Resource {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub content: String,
    pub token_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct CourseListResponse {
    pub success: bool,
    pub courses: Vec<Course>,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub success: bool,
    pub course: Course,
}

#[derive(Debug, Serialize)]
pub struct ResourceListResponse {
    pub success: bool,
    pub resources: Vec<Resource>,
}

#[derive(Debug, Serialize)]
pub struct ResourceResponse {
    pub success: bool,
    pub resource: Resource,
}

// -- Session DTOs --

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub topic: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Serialize)]
pub struct Session {
    pub id: String,
    pub course_id: String,
    pub topic: String,
    pub instructions: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_content: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub success: bool,
    pub session: Session,
}

#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub success: bool,
    pub sessions: Vec<Session>,
}

// -- Session Supabase row types --
use crate::infra::user::get_user_id_from_session;

// =============================================================================
// HANDLER 1: Create Course
// POST /api/intello/courses
// =============================================================================

// =============================================================================
// HANDLER 1: Create Course
// POST /api/intello/courses
// =============================================================================

pub async fn create_course(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CreateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(user_id = %user_id, name = %body.name, "Creating course via Service");

    let input = crate::services::intello::course::domain::CreateCourseInput {
        name: body.name.clone(),
        description: body.description.clone(),
    };

    let course = app.intello_service.create_course(&user_id, input).await?;

    Ok(HttpResponse::Created().json(CourseResponse {
        success: true,
        course: Course {
            id: course.id,
            user_id: course.user_id,
            name: course.name,
            description: course.description,
            created_at: course.created_at,
            updated_at: course.updated_at,
        },
    }))
}

// =============================================================================
// HANDLER 2: List User's Courses
// GET /api/intello/courses
// =============================================================================

pub async fn list_courses(app: web::Data<App>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(user_id = %user_id, "Listing courses via Service");

    let courses = app.intello_service.list_user_courses(&user_id).await?;

    let response_courses: Vec<Course> = courses
        .into_iter()
        .map(|c| Course {
            id: c.id,
            user_id: c.user_id,
            name: c.name,
            description: c.description,
            created_at: c.created_at,
            updated_at: c.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(CourseListResponse {
        success: true,
        courses: response_courses,
    }))
}

// =============================================================================
// HANDLER: Delete Course
// DELETE /api/intello/courses/{course_id}
// =============================================================================

pub async fn delete_course(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, "Deleting course via Service");

    app.intello_service
        .delete_course(&user_id, &course_id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Course deleted successfully"
    })))
}

// =============================================================================
// HANDLER 3: Upload Resource
// POST /api/intello/courses/{course_id}/resources
// =============================================================================

// =============================================================================
// HANDLER 3: Upload Resource (Multipart)
// POST /api/intello/courses/{course_id}/resources
// =============================================================================

pub async fn upload_resource(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    payload: actix_multipart::Multipart,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, "Uploading resources via Service (Multipart)");

    // 1. Parse and extract files
    let documents = super::helpers::parse_multipart_files_only(payload).await?;
    let mut created_resources = Vec::new();

    for (filename, content, token_count) in documents {
        // 2. Create user resource
        let resource = app
            .intello_service
            .create_resource(&user_id, filename, content, token_count as i32)
            .await?;

        // 3. Link resource to course
        app.intello_service
            .link_resource_to_course(&user_id, &course_id, &resource.id)
            .await?;

        created_resources.push(Resource {
            id: resource.id,
            user_id: resource.user_id,
            filename: resource.filename,
            content: resource.content,
            token_count: resource.token_count,
            created_at: resource.created_at,
        });
    }

    // Return list of created resources
    Ok(HttpResponse::Created().json(ResourceListResponse {
        success: true,
        resources: created_resources,
    }))
}

// =============================================================================
// HANDLER 4: Get Course Resources
// GET /api/intello/courses/{course_id}/resources
// =============================================================================

pub async fn get_resources(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, "Getting resources via Service");

    let resources = app
        .intello_service
        .list_course_resources(&user_id, &course_id)
        .await?;

    let response_resources: Vec<Resource> = resources
        .into_iter()
        .map(|r| Resource {
            id: r.id,
            user_id: r.user_id,
            filename: r.filename,
            content: r.content,
            token_count: r.token_count,
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ResourceListResponse {
        success: true,
        resources: response_resources,
    }))
}

// =============================================================================
// HANDLER 5: Create Session
// POST /api/intello/courses/{course_id}/sessions
// =============================================================================

pub async fn create_session(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, topic = %body.topic, "Creating session via Service");

    let input =
        crate::services::intello::study_session::study_session_domain::CreateStudySessionInput {
            topic: body.topic.clone(),
            instructions: body.instructions.clone(),
            keywords: body.keywords.clone(),
            language: body.language.clone(),
        };

    let session = app
        .intello_service
        .create_study_session(&user_id, &course_id, input)
        .await?;

    Ok(HttpResponse::Created().json(SessionResponse {
        success: true,
        session: Session {
            id: session.id,
            course_id: session.course_id,
            topic: session.topic,
            instructions: session.instructions,
            keywords: session.keywords,
            language: session.language,
            status: session.status,
            generated_content: None,
            created_at: session.created_at,
        },
    }))
}

// =============================================================================
// HANDLER 6: List Course Sessions
// GET /api/intello/courses/{course_id}/sessions
// =============================================================================

pub async fn list_sessions(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, "Listing sessions via Service");

    let sessions = app
        .intello_service
        .list_course_sessions(&user_id, &course_id)
        .await?;

    let response_sessions: Vec<Session> = sessions
        .into_iter()
        .map(|s| Session {
            id: s.id,
            course_id: s.course_id,
            topic: s.topic,
            instructions: s.instructions,
            keywords: s.keywords,
            language: s.language,
            status: s.status,
            generated_content: None, // Keep list lightweight
            created_at: s.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(SessionListResponse {
        success: true,
        sessions: response_sessions,
    }))
}

// =============================================================================
// HANDLER 7: Get Session Details
// GET /api/intello/courses/{course_id}/sessions/{session_id}
// =============================================================================

pub async fn get_session(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let (_course_id, session_id) = path.into_inner();

    info!(user_id = %user_id, session_id = %session_id, "Getting session details via Service");

    let session = app
        .intello_service
        .get_study_session(&user_id, &session_id)
        .await?;

    Ok(HttpResponse::Ok().json(SessionResponse {
        success: true,
        session: Session {
            id: session.id,
            course_id: session.course_id,
            topic: session.topic,
            instructions: session.instructions,
            keywords: session.keywords,
            language: session.language,
            status: session.status,
            generated_content: session.generated_content,
            created_at: session.created_at,
        },
    }))
}

// =============================================================================
// HANDLER 8: Delete Session
// DELETE /api/intello/courses/{course_id}/sessions/{session_id}
// =============================================================================

pub async fn delete_session(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let (course_id, session_id) = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, session_id = %session_id, "Deleting session via Service");

    // Verify course ownership
    let courses = app.intello_service.list_user_courses(&user_id).await?;
    if !courses.iter().any(|c| c.id == course_id) {
        return Err(AppError::Intello(
            crate::services::intello::error_domain::IntelloError::Forbidden,
        ));
    }

    app.intello_service
        .study_session_repo
        .delete(&session_id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Session deleted successfully"
    })))
}
