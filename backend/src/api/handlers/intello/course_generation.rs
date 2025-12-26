//! Course Generation Handler
//!
//! Simple handler that calls OpenRouter directly for course generation.
//! Fetches resources directly from Supabase when resource_ids are provided.

use actix_web::{web, HttpResponse};
use tracing::info;
use std::sync::Arc;

use crate::app::App;
use crate::error::AppError;
use crate::api::dto::intello::course::GenerateCourseRequest;
use crate::use_cases::intello::GenerateCourseUseCase;

// =============================================================================
// HANDLER
// =============================================================================

/// Generate course content via AI
/// 
/// POST /api/intello/generate-course
/// 
/// This endpoint delegates to the GenerateCourseUseCase.
pub async fn generate_course_handler(
    app: web::Data<App>,
    body: web::Json<GenerateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    info!("Handling generate course request");
    
    // Initialize Use Case with IntelloService
    // In a real dependency injection container, this would be injected.
    // Here we construct it on the fly using the service from App state.
    let use_case = GenerateCourseUseCase::new(Arc::clone(&app.intello_service));

    // Execute Use Case
    let response = use_case.execute(body.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(response))
}
