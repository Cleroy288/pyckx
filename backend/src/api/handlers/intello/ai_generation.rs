//! AI content generation handlers
//!
//! Handlers for generating QCM, open questions, and flashcards using AI.
//! All handlers delegate to Use Cases for validation and orchestration.

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateCustomQuestionRequest, CreateFlashcardRequest, CreateFlashcardResponse,
    CreateOpenQuestionRequest, CreateOpenQuestionResponse, CustomQuestionResponse,
    FlashcardResponse, OpenQuestionResponse, QcmQuestionResponse,
};
use crate::app::App;
use crate::error::AppResult;
use crate::shared::session::get_user_id_from_session;
use crate::use_cases::intello::{
    GenerateFlashcardsInput, GenerateFlashcardsUseCase, GenerateOpenQuestionsInput,
    GenerateOpenQuestionsUseCase, GenerateQcmInput, GenerateQcmUseCase,
};
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tracing::instrument;

/// POST /api/intello/qcm/generate - Generate AI QCM from documents
#[post("/qcm/generate")]
#[instrument(skip(app, req, payload))]
pub async fn create_custom_question_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateCustomQuestionRequest>(payload).await?;

    // Parse level from metadata (DTO validation)
    let level = metadata.parse_level().map_err(|e| crate::error::AppError::validation("level", e))?;

    // Build use case input
    let input = GenerateQcmInput {
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

    // Execute use case (validation + generation inside)
    let use_case = GenerateQcmUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let question_responses: Vec<QcmQuestionResponse> = output.game_set.questions.iter()
        .map(QcmQuestionResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CustomQuestionResponse {
        success: true,
        message: format!("Custom question created with {} AI-generated questions", output.game_set.questions.len()),
        id: output.game_set.id,
        total_token_count: output.total_token_count,
        documents_processed: output.documents_processed,
        questions: question_responses,
    }))
}

/// POST /api/intello/open-questions - Generate AI open questions
#[post("/open-questions")]
#[instrument(skip(app, req, payload))]
pub async fn create_open_question_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateOpenQuestionRequest>(payload).await?;

    // Parse level from metadata (DTO validation)
    let level = metadata.parse_level().map_err(|e| crate::error::AppError::validation("level", e))?;

    // Build use case input
    let input = GenerateOpenQuestionsInput {
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

    // Execute use case (validation + generation inside)
    let use_case = GenerateOpenQuestionsUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let question_responses: Vec<OpenQuestionResponse> = output.game_set.questions.iter()
        .map(OpenQuestionResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateOpenQuestionResponse {
        success: true,
        message: format!("Open question set created with {} questions", output.game_set.questions.len()),
        id: output.game_set.id,
        total_token_count: output.total_token_count,
        documents_processed: output.documents_processed,
        questions: question_responses,
    }))
}

/// POST /api/intello/flashcards - Generate AI flashcards
#[post("/flashcards")]
#[instrument(skip(app, req, payload))]
pub async fn create_flashcard_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateFlashcardRequest>(payload).await?;

    // Parse level from metadata (DTO validation)
    let level = metadata.parse_level().map_err(|e| crate::error::AppError::validation("level", e))?;

    // Build use case input
    let input = GenerateFlashcardsInput {
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

    // Execute use case (validation + generation inside)
    let use_case = GenerateFlashcardsUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let card_responses: Vec<FlashcardResponse> = output.game_set.cards.iter()
        .map(FlashcardResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateFlashcardResponse {
        success: true,
        message: format!("Flashcard set created with {} cards", output.game_set.cards.len()),
        id: output.game_set.id,
        total_token_count: output.total_token_count,
        documents_processed: output.documents_processed,
        cards: card_responses,
    }))
}
