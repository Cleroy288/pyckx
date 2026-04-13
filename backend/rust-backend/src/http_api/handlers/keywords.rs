//! Keywords handlers - API endpoints for Keywords game

use super::helpers::parse_multipart;
use crate::app::App;
use crate::http_api::data_transfer_object::{
    CreateKeywordsRequest, CreateKeywordsResponse, KeywordQuestionResponse,
    KeywordResponse, KeywordSetListResponse, KeywordSetWithQuestionsResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::shared::{AppError, AppResult};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /api/study/keywords - Generate AI keyword questions
#[post("/keywords")]
#[instrument(skip(app, req, payload))]
pub async fn create_keywords_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    // Parse multipart form data
    let (metadata, documents) =
        parse_multipart::<CreateKeywordsRequest>(payload).await?;

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
    let keyword_set = app
        .study_service
        .generate_ai_keywords(&user_id, service_input)
        .await?;

    // Build response
    let question_responses: Vec<KeywordQuestionResponse> = keyword_set
        .questions
        .iter()
        .map(|q| KeywordQuestionResponse {
            id: q.id.to_string(),
            statement: q.statement.clone(),
            keywords: q.keywords.iter().map(KeywordResponse::from).collect(),
            explanation: q.explanation.clone(),
        })
        .collect();

    Ok(HttpResponse::Created().json(CreateKeywordsResponse {
        success: true,
        message: format!(
            "Keyword set created with {} questions",
            keyword_set.questions.len()
        ),
        id: keyword_set.id.to_string(),
        total_token_count: 0,
        documents_processed: keyword_set.questions.len(),
        questions: question_responses,
    }))
}

/// GET /api/study/keywords - List user's keyword sets (with questions for playing)
#[get("/keywords")]
#[instrument(skip(app, req))]
pub async fn list_keyword_sets_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    let sets = app.study_service.get_user_keyword_sets(&user_id).await?;

    let set_responses: Vec<KeywordSetWithQuestionsResponse> = sets
        .iter()
        .map(|set| KeywordSetWithQuestionsResponse {
            id: set.id.to_string(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: set.level.to_string(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set
                .questions
                .iter()
                .map(|q| KeywordQuestionResponse {
                    id: q.id.to_string(),
                    statement: q.statement.clone(),
                    keywords: q
                        .keywords
                        .iter()
                        .map(KeywordResponse::from)
                        .collect(),
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
