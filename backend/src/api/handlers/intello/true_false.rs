//! True or False handlers - API endpoints for True/False game

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateTrueOrFalseRequest, CreateTrueOrFalseResponse, TrueOrFalseSetListResponse,
    TrueOrFalseSetWithStatementsResponse, TrueOrFalseStatementResponse,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::services::{validate_model, GenerateContentInput};
use crate::shared::session::get_user_id_from_session;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tracing::{info, instrument};

/// POST /app/intello/true-false/create - Generate AI true/false statements
#[post("/true-false/create")]
#[instrument(skip(app, req, payload))]
pub async fn create_true_false_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateTrueOrFalseRequest>(payload).await?;

    // Validate metadata
    let level = metadata.parse_level().map_err(|e| AppError::validation("level", e))?;
    metadata.validate_subjects().map_err(|e| AppError::validation("subjects", e))?;
    metadata.validate_num_questions().map_err(|e| AppError::validation("num_questions", e))?;
    
    // Validate model if provided
    if let Some(ref model) = metadata.model {
        validate_model(model).map_err(|e| AppError::validation("model", e))?;
    }

    let total_token_count: u32 = documents.iter().map(|(_, _, t)| t).sum();

    info!(
        name = %metadata.name,
        num_statements = metadata.num_questions,
        model = ?metadata.model,
        documents = documents.len(),
        total_tokens = total_token_count,
        "Generating AI true/false statements"
    );

    // Build input for service
    let input = GenerateContentInput {
        name: metadata.name,
        description: metadata.description,
        instructions: metadata.instructions,
        language: metadata.language,
        level,
        subjects: metadata.subjects,
        num_questions: metadata.num_questions,
        documents: documents.clone(),
        model: metadata.model,
    };

    // Delegate to service
    let true_false_set = app.intello_service.generate_ai_true_false(&user_id, input).await?;

    // Build response
    let statement_responses: Vec<TrueOrFalseStatementResponse> = true_false_set.statements.iter()
        .map(TrueOrFalseStatementResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateTrueOrFalseResponse {
        success: true,
        message: format!("True/false set created with {} statements", true_false_set.statements.len()),
        id: true_false_set.id,
        total_token_count,
        documents_processed: documents.len(),
        statements: statement_responses,
    }))
}

/// GET /app/intello/true-false/list - List user's true/false sets (with statements for playing)
#[get("/true-false/list")]
#[instrument(skip(app, req))]
pub async fn list_true_false_sets_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    let sets = app.intello_service.get_user_true_false_sets(&user_id).await?;

    let set_responses: Vec<TrueOrFalseSetWithStatementsResponse> = sets.iter()
        .map(|set| TrueOrFalseSetWithStatementsResponse {
            id: set.id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            statements: set.statements.iter()
                .map(TrueOrFalseStatementResponse::from)
                .collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(TrueOrFalseSetListResponse {
        count: set_responses.len(),
        sets: set_responses,
    }))
}

