//! Course generation handler
//!
//! NOTE: Currently disabled - course generation needs to be reimplemented
//! after the OpenRouter refactoring. The old course_generation_service.rs
//! was moved as part of the infra/services separation.
//!
//! TODO: Reimplement course generation:
//! 1. Move logic to intello/course_generation module
//! 2. Use OpenRouterClient.send_chat_request() for AI calls
//! 3. Parse responses using ai_parsing_service

use actix_web::{web, HttpResponse};
use crate::http_api::data_transfer_object::intello::course::GenerateCourseRequest;
use crate::app::App;
use tracing::error;

/// Generate a comprehensive course with modules and exercises
pub async fn generate_course(
    _app: web::Data<App>,
    _request: web::Json<GenerateCourseRequest>,
) -> HttpResponse {
    // TODO: Reimplement after refactoring is complete
    error!("Course generation temporarily disabled during refactoring");
    HttpResponse::ServiceUnavailable().json(serde_json::json!({
        "error": "Course generation is temporarily unavailable during system refactoring. Please try again later."
    }))
}
