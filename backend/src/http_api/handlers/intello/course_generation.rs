//! Course generation handler
//!
//! Generates comprehensive courses with modules and exercises using AI.

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{error, info};

use crate::app::App;
use crate::http_api::data_transfer_object::intello::course::{
    GenerateCourseRequest, GenerateCourseResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::services::intello::course::domain::GenerateCourseInput;
use crate::shared::AppResult;

/// Generate a comprehensive course with modules and exercises
pub async fn generate_course(
    app: web::Data<App>,
    request: web::Json<GenerateCourseRequest>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    // Get user from session
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(
        user_id = %user_id,
        topic = %request.topic,
        "Multi-stage course generation requested"
    );

    // Build service input
    let input = GenerateCourseInput {
        topic: request.topic.clone(),
        keywords: request.keywords.clone(),
        instructions: request.instructions.clone(),
        resources: request.resources.clone(),
        session_id: request.session_id.clone(),
        text_length: request.text_length.clone(),
        exercise_depth: request.exercise_depth.clone(),
    };

    // Use multi-stage generation (default: 4 sections)
    match app
        .intello_service
        .generate_course(&user_id, input, None)
        .await
    {
        Ok(course) => {
            info!(
                user_id = %user_id,
                title = %course.course_metadata.title,
                modules = course.modules.len(),
                "Course generated successfully"
            );
            Ok(HttpResponse::Ok().json(GenerateCourseResponse {
                success: true,
                course,
            }))
        }
        Err(err) => {
            error!(
                user_id = %user_id,
                error = %err,
                "Course generation failed"
            );
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to generate course: {}", err)
            })))
        }
    }
}
