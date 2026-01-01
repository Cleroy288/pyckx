//! Course Generation Handler
//!
//! Simple handler that calls OpenRouter directly for course generation.
//! Fetches resources directly from Supabase when resource_ids are provided.

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{info, error};

use crate::app::App;
use crate::shared::AppError;
use crate::http_api::data_transfer_object::intello::course::{GenerateCourseRequest, GenerateCourseResponse};
use crate::infra::user::get_user_id_from_session;

// =============================================================================
// HANDLER
// =============================================================================

/// Generate course content via AI
/// 
/// POST /api/intello/generate-course
/// 
pub async fn generate_course_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<GenerateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    info!("Handling generate course request for user {}", user_id);
    
    let request = body.into_inner();
    
    // Fetch resources if resource_ids provided
    let resources = if !request.resource_ids.is_empty() {
        let fetched = app.intello_service.course_repo
            .fetch_resources(&request.resource_ids)
            .await
            .map_err(|e| AppError::Internal(crate::http_api::utils::InternalError::new(e.to_string())))?;
        
        fetched.iter()
            .map(|r| format!("=== {} ===\n{}", r.filename, r.content))
            .collect::<Vec<_>>()
            .join("\n\n")
    } else {
        request.resources.clone()
    };

    // Call OpenRouter service directly
    let (generation_result, _) = app.openrouter_service
        .generate_course_unified(
            &request.topic,
            &request.keywords,
            &request.instructions,
            &resources,
            request.text_length.clone(),
            request.exercise_depth.clone(),
        )
        .await
        .map_err(|e| {
            error!("Course generation failed: {}", e);
            e
        })?;

    // Log AI usage for all stages
    // Note: Usage tracking is handled inside generate_course_unified
    
    // Persist content if session_id is provided
    if let Some(session_id) = request.session_id {
        info!("Saving generated course to session {}", session_id);
        let content_json = serde_json::to_value(&generation_result)
            .map_err(|e| AppError::Internal(crate::http_api::utils::InternalError::new(e.to_string())))?;
            
        app.intello_service.study_session_repo
            .save_session_content(&session_id, &content_json)
            .await?;
    }

    Ok(HttpResponse::Ok().json(GenerateCourseResponse {
        success: true,
        course: generation_result.course,
    }))
}
