//! Session CRUD Handlers
//!
//! HTTP handlers for study session management within
//! courses.

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::info;

use crate::app::App;
use crate::infra::user::get_user_id_from_session;
use crate::services::error_domain::StudyError;
use crate::shared::AppError;

use super::course_dto::{
    CreateSessionRequest, Session,
    SessionListResponse, SessionResponse,
};

/// POST /api/study/courses/{id}/sessions
pub async fn create_session(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let course_id = path.into_inner();
    info!(user_id = %user_id, course_id = %course_id, "Creating session");

    let input = crate::services::study_session
        ::study_session_domain::CreateStudySessionInput {
            topic: body.topic.clone(),
            instructions: body.instructions.clone(),
            keywords: body.keywords.clone(),
            language: body.language.clone(),
        };

    let s = app
        .study_service
        .create_study_session(&user_id, &course_id, input)
        .await?;

    Ok(HttpResponse::Created().json(SessionResponse {
        success: true,
        session: map_session(s, None),
    }))
}

/// GET /api/study/courses/{id}/sessions
pub async fn list_sessions(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let course_id = path.into_inner();
    info!(user_id = %user_id, course_id = %course_id, "Listing sessions");

    let sessions = app
        .study_service
        .list_course_sessions(&user_id, &course_id)
        .await?;

    let items: Vec<Session> = sessions
        .into_iter()
        .map(|s| map_session(s, None))
        .collect();

    Ok(HttpResponse::Ok().json(SessionListResponse {
        success: true,
        sessions: items,
    }))
}

/// GET /api/study/courses/{course_id}/sessions/{session_id}
pub async fn get_session(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let (_course_id, session_id) = path.into_inner();
    info!(user_id = %user_id, session_id = %session_id, "Getting session");

    let s = app
        .study_service
        .get_study_session(&user_id, &session_id)
        .await?;

    let content = s.generated_content.clone();

    Ok(HttpResponse::Ok().json(SessionResponse {
        success: true,
        session: map_session(s, content),
    }))
}

/// DELETE /api/study/courses/{course_id}/sessions/{session_id}
pub async fn delete_session(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let (course_id, session_id) = path.into_inner();
    info!(user_id = %user_id, session_id = %session_id, "Deleting session");

    verify_course_ownership(&app, &user_id, &course_id)
        .await?;

    app.study_service
        .delete_study_session(&user_id, &session_id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Session deleted successfully"
    })))
}

/// Verifies the user owns the specified course.
async fn verify_course_ownership(
    app: &web::Data<App>,
    user_id: &str,
    course_id: &str,
) -> Result<(), AppError> {
    let courses = app
        .study_service
        .list_user_courses(user_id)
        .await?;
    if !courses.iter().any(|c| c.id == course_id) {
        return Err(AppError::Study(StudyError::Forbidden));
    }
    Ok(())
}

/// Maps a domain StudySession to the DTO Session.
fn map_session(
    s: crate::services::study_session
        ::study_session_domain::StudySession,
    content: Option<serde_json::Value>,
) -> Session {
    Session {
        id: s.id,
        course_id: s.course_id,
        topic: s.topic,
        instructions: s.instructions,
        keywords: s.keywords,
        language: s.language,
        status: s.status,
        generated_content: content,
        created_at: s.created_at,
    }
}
