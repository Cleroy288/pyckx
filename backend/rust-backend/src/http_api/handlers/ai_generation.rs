//! AI content generation handlers
//!
//! Handlers for generating QCM, open questions, and flashcards using AI.

use super::helpers::parse_multipart;
use crate::app::App;
use crate::http_api::data_transfer_object::{
    CreateCustomQuestionRequest, CreateFlashcardRequest,
    CreateFlashcardResponse, CreateOpenQuestionRequest,
    CreateOpenQuestionResponse, CustomQuestionResponse,
    FlashcardResponse, OpenQuestionResponse,
    QcmQuestionResponse, QuickQcmRequest,
    QuickQcmResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::shared::AppResult;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /api/study/qcm/generate - Generate AI QCM from documents
#[post("/qcm/generate")]
#[instrument(skip(app, req, payload))]
pub async fn create_custom_question_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    // Parse multipart form data
    let (metadata, documents) =
        parse_multipart::<CreateCustomQuestionRequest>(payload).await?;

    // Parse level from metadata (DTO validation)
    let level = metadata
        .parse_level()
        .map_err(|err| crate::shared::AppError::validation("level", err))?;

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

    // Call service directly (validation + generation inside)
    let qcm_set = app
        .study_service
        .generate_ai_qcm(&user_id, service_input)
        .await?;

    // Build response
    let question_responses: Vec<QcmQuestionResponse> = qcm_set
        .questions
        .iter()
        .map(QcmQuestionResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CustomQuestionResponse {
        success: true,
        message: format!(
            "Custom question created with {} AI-generated questions",
            qcm_set.questions.len()
        ),
        id: qcm_set.id.to_string(),
        total_token_count: 0, // TODO: Return from service if needed
        documents_processed: qcm_set.questions.len(),
        questions: question_responses,
    }))
}

/// POST /api/study/open-questions - Generate AI open questions
#[post("/open-questions")]
#[instrument(skip(app, req, payload))]
pub async fn create_open_question_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    // Parse multipart form data
    let (metadata, documents) =
        parse_multipart::<CreateOpenQuestionRequest>(payload).await?;

    // Parse level from metadata (DTO validation)
    let level = metadata
        .parse_level()
        .map_err(|err| crate::shared::AppError::validation("level", err))?;

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

    // Call service (source_content built internally)
    let question_set = app
        .study_service
        .generate_ai_open_questions(&user_id, service_input)
        .await?;

    // Build response
    let question_responses: Vec<OpenQuestionResponse> = question_set
        .questions
        .iter()
        .map(OpenQuestionResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateOpenQuestionResponse {
        success: true,
        message: format!(
            "Open question set created with {} questions",
            question_set.questions.len()
        ),
        id: question_set.id.to_string(),
        total_token_count: 0,
        documents_processed: question_set.questions.len(),
        questions: question_responses,
    }))
}

/// POST /api/study/flashcards - Generate AI flashcards
#[post("/flashcards")]
#[instrument(skip(app, req, payload))]
pub async fn create_flashcard_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    // Parse multipart form data
    let (metadata, documents) =
        parse_multipart::<CreateFlashcardRequest>(payload).await?;

    // Parse level from metadata (DTO validation)
    let level = metadata
        .parse_level()
        .map_err(|err| crate::shared::AppError::validation("level", err))?;

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
    let flashcard_set = app
        .study_service
        .generate_ai_flashcards(&user_id, service_input)
        .await?;

    // Build response
    let card_responses: Vec<FlashcardResponse> = flashcard_set
        .cards
        .iter()
        .map(FlashcardResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateFlashcardResponse {
        success: true,
        message: format!(
            "Flashcard set created with {} cards",
            flashcard_set.cards.len()
        ),
        id: flashcard_set.id.to_string(),
        total_token_count: 0,
        documents_processed: flashcard_set.cards.len(),
        cards: card_responses,
    }))
}

/// POST /api/study/qcm/generate-quick
/// Generate ephemeral QCM from uploaded documents
#[post("/qcm/generate-quick")]
#[instrument(skip(app, req, payload))]
pub async fn create_quick_qcm_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id =
        get_user_id_from_session(&app, &req).await?;

    let (metadata, documents) =
        parse_multipart::<QuickQcmRequest>(payload)
            .await?;

    let level =
        metadata.parse_level().map_err(|err| {
            crate::shared::AppError::validation(
                "level", err,
            )
        })?;

    let input = crate::services::QuickQcmInput {
        language: metadata.language,
        level,
        num_questions: metadata.num_questions,
        documents,
    };

    let qcm_set = app
        .study_service
        .generate_quick_qcm(&user_id, input)
        .await?;

    Ok(HttpResponse::Ok().json(
        QuickQcmResponse::from(&qcm_set),
    ))
}

