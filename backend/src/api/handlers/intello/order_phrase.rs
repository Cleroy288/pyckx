//! Order Phrase handlers - API endpoints for Order Phrase game

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateOrderPhraseRequest, CreateOrderPhraseResponse, OrderPhraseQuestionResponse,
    OrderPhraseSetListResponse, OrderPhraseSetWithQuestionsResponse, OrderPhraseWordResponse,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::services::validate_model;
use crate::shared::session::get_user_id_from_session;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tracing::{info, instrument};

/// POST /app/intello/order-phrase/create - Generate AI order phrase questions
#[post("/order-phrase/create")]
#[instrument(skip(app, req, payload))]
pub async fn create_order_phrase_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateOrderPhraseRequest>(payload).await?;

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
        num_questions = metadata.num_questions,
        model = ?metadata.model,
        documents = documents.len(),
        total_tokens = total_token_count,
        "Generating AI order phrase questions"
    );

    // Build input for service
    let input = crate::services::GenerateContentInput {
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
    let order_phrase_set = app.intello_service.generate_ai_order_phrases(&user_id, input).await?;

    // Build response
    let question_responses: Vec<OrderPhraseQuestionResponse> = order_phrase_set.questions.iter()
        .map(|q| OrderPhraseQuestionResponse {
            id: q.id.clone(),
            original_phrase: q.original_phrase.clone(),
            words: q.words.iter().map(OrderPhraseWordResponse::from).collect(),
            hint: q.hint.clone(),
        })
        .collect();

    Ok(HttpResponse::Created().json(CreateOrderPhraseResponse {
        success: true,
        message: format!("Order phrase set created with {} questions", order_phrase_set.questions.len()),
        id: order_phrase_set.id,
        total_token_count,
        documents_processed: documents.len(),
        questions: question_responses,
    }))
}

/// GET /app/intello/order-phrase/list - List user's order phrase sets (with questions for playing)
#[get("/order-phrase/list")]
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
