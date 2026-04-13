//! Course generation handler
//!
//! Generates comprehensive courses using AI.

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::info;

use crate::app::App;
use crate::http_api::data_transfer_object::course::{
    GenerateCourseRequest, GenerateCourseResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::services::course::domain::GenerateCourseInput;
use crate::shared::AppResult;

/// Generate a comprehensive course with modules and exercises
pub async fn generate_course(
    app: web::Data<App>,
    request: web::Json<GenerateCourseRequest>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    info!(
        user_id = %user_id,
        topic = %request.topic,
        "Multi-stage course generation requested"
    );

    let input = build_course_input(&request);

    let course = app
        .study_service
        .generate_course(&user_id, input, None)
        .await?;

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

/// Build GenerateCourseInput from the request DTO
fn build_course_input(
    request: &GenerateCourseRequest,
) -> GenerateCourseInput {
    GenerateCourseInput {
        topic: request.topic.clone(),
        keywords: request.keywords.clone(),
        instructions: request.instructions.clone(),
        resources: request.resources.clone(),
        session_id: request.session_id.clone(),
        text_length: request.text_length.clone(),
        exercise_depth: request.exercise_depth.clone(),
    }
}
