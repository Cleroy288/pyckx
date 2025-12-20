//! Order Phrase handlers - API endpoints for Order Phrase game

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateOrderPhraseRequest, CreateOrderPhraseResponse, OrderPhraseQuestionResponse,
    OrderPhraseSetListResponse, OrderPhraseSetWithQuestionsResponse, OrderPhraseWordResponse,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::shared::session::get_user_id_from_session;
use crate::use_cases::intello::{GenerateOrderPhraseInput, GenerateOrderPhraseUseCase};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tracing::instrument;

/// POST /api/intello/order-phrases - Generate AI order phrase questions
#[post("/order-phrases")]
#[instrument(skip(app, req, payload))]
pub async fn create_order_phrase_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateOrderPhraseRequest>(payload).await?;

    // Parse level (DTO validation)
    let level = metadata.parse_level().map_err(|e| AppError::validation("level", e))?;

    // Build use case input
    let input = GenerateOrderPhraseInput {
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
    let use_case = GenerateOrderPhraseUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let question_responses: Vec<OrderPhraseQuestionResponse> = output.game_set.questions.iter()
        .map(|q| OrderPhraseQuestionResponse {
            id: q.id.clone(),
            original_phrase: q.original_phrase.clone(),
            words: q.words.iter().map(OrderPhraseWordResponse::from).collect(),
            hint: q.hint.clone(),
        })
        .collect();

    Ok(HttpResponse::Created().json(CreateOrderPhraseResponse {
        success: true,
        message: format!("Order phrase set created with {} questions", output.game_set.questions.len()),
        id: output.game_set.id,
        total_token_count: output.total_token_count,
        documents_processed: output.documents_processed,
        questions: question_responses,
    }))
}

/// GET /api/intello/order-phrases - List user's order phrase sets (with questions for playing)
#[get("/order-phrases")]
#[instrument(skip(app, req))]
pub async fn list_order_phrase_sets_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    let sets = app.intello_service.get_user_order_phrase_sets(&user_id).await?;

    let set_responses: Vec<OrderPhraseSetWithQuestionsResponse> = sets.iter()
        .map(|set| OrderPhraseSetWithQuestionsResponse {
            id: set.id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set.questions.iter()
                .map(|q| OrderPhraseQuestionResponse {
                    id: q.id.clone(),
                    original_phrase: q.original_phrase.clone(),
                    words: q.words.iter().map(OrderPhraseWordResponse::from).collect(),
                    hint: q.hint.clone(),
                })
                .collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(OrderPhraseSetListResponse {
        count: set_responses.len(),
        sets: set_responses,
    }))
}
