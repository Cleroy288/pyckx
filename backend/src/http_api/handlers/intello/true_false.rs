//! True or False handlers - API endpoints for True/False game

use super::helpers::parse_multipart;
use crate::http_api::data_transfer_object::intello::{
    CreateTrueOrFalseRequest, CreateTrueOrFalseResponse, TrueOrFalseSetListResponse,
    TrueOrFalseSetWithStatementsResponse, TrueOrFalseStatementResponse,
};
use crate::app::App;
use crate::shared::{AppError, AppResult};
use crate::infra::user::get_user_id_from_session;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
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
    let level = metadata
        .parse_level()
        .map_err(|e| AppError::validation("level", e))?;

    // Build service input
    let service_input = crate::services::GenerateContentInput {
        name: metadata.name,
        description: metadata.description,
        instructions: metadata.instructions,
        language: metadata.language,
        level,
        subjects: metadata.subjects,
        num_questions: metadata.num_questions,
        documents,
    };

    // Call service directly
    let true_false_set = app.intello_service.generate_ai_true_false(&user_id, service_input).await?;

    // Build response
    let statement_responses: Vec<TrueOrFalseStatementResponse> = true_false_set
        .statements
        .iter()
        .map(TrueOrFalseStatementResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateTrueOrFalseResponse {
        success: true,
        message: format!(
            "True/false set created with {} statements",
            true_false_set.statements.len()
        ),
        id: true_false_set.id.to_string(),
        total_token_count: 0,
        documents_processed: true_false_set.statements.len(),
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

    let sets = app
        .intello_service
        .get_user_true_false_sets(&user_id)
        .await?;

    let set_responses: Vec<TrueOrFalseSetWithStatementsResponse> = sets
        .iter()
        .map(|set| TrueOrFalseSetWithStatementsResponse {
            id: set.id.to_string(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            statements: set
                .statements
                .iter()
                .map(TrueOrFalseStatementResponse::from)
                .collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(TrueOrFalseSetListResponse {
        count: set_responses.len(),
        sets: set_responses,
    }))
}
