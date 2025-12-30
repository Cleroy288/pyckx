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

    let input = crate::services::intello::course_domain::CreateCourseInput {
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
// HANDLER 3: Upload Resource
// POST /api/intello/courses/{course_id}/resources
// =============================================================================

pub async fn upload_resource(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UploadResourceRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let course_id = path.into_inner();

    info!(user_id = %user_id, course_id = %course_id, filename = %body.filename, "Uploading resource via Service");

    // 1. Create user resource
    let resource = app
        .intello_service
        .create_resource(
            &user_id,
            body.filename.clone(),
            body.content.clone(),
            body.token_count,
        )
        .await?;

    // 2. Link resource to course
    app.intello_service
        .link_resource_to_course(&user_id, &course_id, &resource.id)
        .await?;

    Ok(HttpResponse::Created().json(ResourceResponse {
        success: true,
        resource: Resource {
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

    let input = crate::services::intello::study_session_domain::CreateStudySessionInput {
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
            created_at: s.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(SessionListResponse {
        success: true,
        sessions: response_sessions,
    }))
}
