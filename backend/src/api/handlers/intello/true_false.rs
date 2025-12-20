//! True or False handlers - API endpoints for True/False game

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateTrueOrFalseRequest, CreateTrueOrFalseResponse, TrueOrFalseSetListResponse,
    TrueOrFalseSetWithStatementsResponse, TrueOrFalseStatementResponse,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::shared::session::get_user_id_from_session;
use crate::use_cases::intello::{GenerateTrueFalseInput, GenerateTrueFalseUseCase};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tracing::instrument;

/// POST /api/intello/true-false - Generate AI true/false statements
#[post("/true-false")]
#[instrument(skip(app, req, payload))]
pub async fn create_true_false_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateTrueOrFalseRequest>(payload).await?;

    // Parse level (DTO validation)
    let level = metadata.parse_level().map_err(|e| AppError::validation("level", e))?;

    // Build use case input
    let input = GenerateTrueFalseInput {
        user_id,
        name: metadata.name,
        description: metadata.description,
        instructions: metadata.instructions,
        language: metadata.language,
        level,
        subjects: metadata.subjects,
        num_questions: metadata.num_questions,
        documents,
        model: metadata.model,
    };

    // Execute use case
    let use_case = GenerateTrueFalseUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let statement_responses: Vec<TrueOrFalseStatementResponse> = output.game_set.statements.iter()
        .map(TrueOrFalseStatementResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateTrueOrFalseResponse {
        success: true,
        message: format!("True/false set created with {} statements", output.game_set.statements.len()),
        id: output.game_set.id,
        total_token_count: output.total_token_count,
        documents_processed: output.documents_processed,
        statements: statement_responses,
    }))
}

/// GET /api/intello/true-false - List user's true/false sets (with statements for playing)
#[get("/true-false")]
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
