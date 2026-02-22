//! Order Phrase handlers - API endpoints for Order Phrase game

use super::helpers::parse_multipart;
use crate::app::App;
use crate::http_api::data_transfer_object::intello::{
    CreateOrderPhraseRequest, CreateOrderPhraseResponse,
    OrderPhraseQuestionResponse, OrderPhraseSetListResponse,
    OrderPhraseSetWithQuestionsResponse, OrderPhraseWordResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::shared::{AppError, AppResult};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
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
    let (metadata, documents) =
        parse_multipart::<CreateOrderPhraseRequest>(payload).await?;

    // Parse level (DTO validation)
    let level = metadata
        .parse_level()
        .map_err(|err| AppError::validation("level", err))?;

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
    let order_phrase_set = app
        .intello_service
        .generate_ai_order_phrases(&user_id, service_input)
        .await?;

    // Build response
    let question_responses: Vec<OrderPhraseQuestionResponse> = order_phrase_set
        .questions
        .iter()
        .map(|q| OrderPhraseQuestionResponse {
            id: q.id.to_string(),
            original_phrase: q.original_phrase.clone(),
            words: q.words.iter().map(OrderPhraseWordResponse::from).collect(),
            hint: q.hint.clone(),
        })
        .collect();

    Ok(HttpResponse::Created().json(CreateOrderPhraseResponse {
        success: true,
        message: format!(
            "Order phrase set created with {} questions",
            order_phrase_set.questions.len()
        ),
        id: order_phrase_set.id.to_string(),
        total_token_count: 0,
        documents_processed: order_phrase_set.questions.len(),
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

    let sets = app
        .intello_service
        .get_user_order_phrase_sets(&user_id)
        .await?;

    let set_responses: Vec<OrderPhraseSetWithQuestionsResponse> = sets
        .iter()
        .map(|set| OrderPhraseSetWithQuestionsResponse {
            id: set.id.to_string(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set
                .questions
                .iter()
                .map(|q| OrderPhraseQuestionResponse {
                    id: q.id.to_string(),
                    original_phrase: q.original_phrase.clone(),
                    words: q
                        .words
                        .iter()
                        .map(OrderPhraseWordResponse::from)
                        .collect(),
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
