//! AI content generation handlers
//!
//! Handlers for generating QCM, open questions, and flashcards using AI.

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateCustomQuestionRequest, CreateFlashcardRequest, CreateFlashcardResponse,
    CreateOpenQuestionRequest, CreateOpenQuestionResponse, CustomQuestionResponse,
    FlashcardResponse, OpenQuestionResponse, QcmQuestionResponse,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::services::{validate_model, GenerateContentInput};
use crate::shared::session::get_user_id_from_session;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse};
use tracing::{info, instrument};

/// POST /app/intello/custom-question - Generate AI QCM from documents
#[post("/custom-question")]
#[instrument(skip(app, req, payload))]
pub async fn create_custom_question_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateCustomQuestionRequest>(payload).await?;

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
        "Generating AI QCM"
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
    let qcm_set = app.intello_service.generate_ai_qcm(&user_id, input).await?;

    // Build response
    let question_responses: Vec<QcmQuestionResponse> = qcm_set.questions.iter()
        .map(QcmQuestionResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CustomQuestionResponse {
        success: true,
        message: format!("Custom question created with {} AI-generated questions", qcm_set.questions.len()),
        id: qcm_set.id,
        total_token_count,
        documents_processed: documents.len(),
        questions: question_responses,
    }))
}

/// POST /app/intello/open-question/create - Generate AI open questions
#[post("/open-question/create")]
#[instrument(skip(app, req, payload))]
pub async fn create_open_question_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateOpenQuestionRequest>(payload).await?;

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
        "Generating AI open questions"
    );

    // Build source content for caching (used during grading)
    let source_content = documents.iter()
        .map(|(filename, content, _)| format!("=== {} ===\n{}", filename, content))
        .collect::<Vec<_>>()
        .join("\n\n");

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
    let open_question_set = app.intello_service
        .generate_ai_open_questions(&user_id, input, source_content)
        .await?;

    // Build response
    let question_responses: Vec<OpenQuestionResponse> = open_question_set.questions.iter()
        .map(OpenQuestionResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateOpenQuestionResponse {
        success: true,
        message: format!("Open question set created with {} questions", open_question_set.questions.len()),
        id: open_question_set.id,
        total_token_count,
        documents_processed: documents.len(),
        questions: question_responses,
    }))
}

/// POST /app/intello/flashcard/create - Generate AI flashcards
#[post("/flashcard/create")]
#[instrument(skip(app, req, payload))]
pub async fn create_flashcard_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateFlashcardRequest>(payload).await?;

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
        num_cards = metadata.num_questions,
        model = ?metadata.model,
        documents = documents.len(),
        total_tokens = total_token_count,
        "Generating AI flashcards"
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
    let flashcard_set = app.intello_service.generate_ai_flashcards(&user_id, input).await?;

    // Build response
    let card_responses: Vec<FlashcardResponse> = flashcard_set.cards.iter()
        .map(FlashcardResponse::from)
        .collect();

    Ok(HttpResponse::Created().json(CreateFlashcardResponse {
        success: true,
        message: format!("Flashcard set created with {} cards", flashcard_set.cards.len()),
        id: flashcard_set.id,
        total_token_count,
        documents_processed: documents.len(),
        cards: card_responses,
    }))
}
