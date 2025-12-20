//! Keywords handlers - API endpoints for Keywords game

use super::helpers::parse_multipart;
use crate::api::dto::intello::{
    CreateKeywordsRequest, CreateKeywordsResponse, KeywordQuestionResponse,
    KeywordResponse, KeywordSetListResponse, KeywordSetWithQuestionsResponse,
};
use crate::app::App;
use crate::error::{AppError, AppResult};
use crate::shared::session::get_user_id_from_session;
use crate::use_cases::intello::{GenerateKeywordsInput, GenerateKeywordsUseCase};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tracing::instrument;

/// POST /api/intello/keywords - Generate AI keyword questions
#[post("/keywords")]
#[instrument(skip(app, req, payload))]
pub async fn create_keywords_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) = parse_multipart::<CreateKeywordsRequest>(payload).await?;

    // Parse level (DTO validation)
    let level = metadata.parse_level().map_err(|e| AppError::validation("level", e))?;

    // Build use case input
    let input = GenerateKeywordsInput {
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
    let use_case = GenerateKeywordsUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let question_responses: Vec<KeywordQuestionResponse> = output.game_set.questions.iter()
        .map(|q| KeywordQuestionResponse {
            id: q.id.clone(),
            statement: q.statement.clone(),
            keywords: q.keywords.iter().map(KeywordResponse::from).collect(),
            explanation: q.explanation.clone(),
        })
        .collect();

    Ok(HttpResponse::Created().json(CreateKeywordsResponse {
        success: true,
        message: format!("Keyword set created with {} questions", output.game_set.questions.len()),
        id: output.game_set.id,
        total_token_count: output.total_token_count,
        documents_processed: output.documents_processed,
        questions: question_responses,
    }))
}

/// GET /api/intello/keywords - List user's keyword sets (with questions for playing)
#[get("/keywords")]
#[instrument(skip(app, req))]
pub async fn list_keyword_sets_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    let sets = app.intello_service.get_user_keyword_sets(&user_id).await?;

    let set_responses: Vec<KeywordSetWithQuestionsResponse> = sets.iter()
        .map(|set| KeywordSetWithQuestionsResponse {
            id: set.id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set.questions.iter()
                .map(|q| KeywordQuestionResponse {
                    id: q.id.clone(),
                    statement: q.statement.clone(),
                    keywords: q.keywords.iter().map(KeywordResponse::from).collect(),
                    explanation: q.explanation.clone(),
                })
                .collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(KeywordSetListResponse {
        count: set_responses.len(),
        sets: set_responses,
    }))
}
