//! AI Models handler

use crate::services::{AVAILABLE_MODELS, DEFAULT_MODEL};
use actix_web::{get, HttpResponse};
use serde::Serialize;
use tracing::instrument;

/// Response containing available AI models
#[derive(Debug, Serialize)]
pub struct AvailableModelsResponse {
    pub models: Vec<String>,
    pub default_model: String,
}

/// GET /app/intello/models
#[get("/models")]
#[instrument]
pub async fn get_available_models_handler() -> HttpResponse {
    let models: Vec<String> = AVAILABLE_MODELS.iter().map(|s| s.to_string()).collect();
    HttpResponse::Ok().json(AvailableModelsResponse {
        models,
        default_model: DEFAULT_MODEL.to_string(),
    })
}
