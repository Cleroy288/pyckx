//! Fill Blank handlers - API endpoints for Fill Blank game

use super::helpers::parse_multipart;
use crate::app::App;
use crate::http_api::data_transfer_object::intello::{
    CreateFillBlankRequest, CreateFillBlankResponse, FillBlankOptionResponse,
    FillBlankQuestionResponse, FillBlankSetListResponse,
    FillBlankSetWithQuestionsResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::shared::{AppError, AppResult};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /api/intello/fill-blanks - Generate AI fill blank questions
#[post("/fill-blanks")]
#[instrument(skip(app, req, payload))]
pub async fn create_fill_blank_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    // Parse multipart form data
    let (metadata, documents) =
        parse_multipart::<CreateFillBlankRequest>(payload).await?;

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
    let fill_blank_set = app
        .intello_service
        .generate_ai_fill_blank(&user_id, service_input)
        .await?;

    // Build response
    let question_responses: Vec<FillBlankQuestionResponse> = fill_blank_set
        .questions
        .iter()
        .map(|q| FillBlankQuestionResponse {
            id: q.id.to_string(),
            phrase: q.phrase.clone(),
            options: q
                .options
                .iter()
                .map(FillBlankOptionResponse::from)
                .collect(),
            explanation: q.explanation.clone(),
        })
        .collect();

    Ok(HttpResponse::Created().json(CreateFillBlankResponse {
        success: true,
        message: format!(
            "Fill blank set created with {} questions",
            fill_blank_set.questions.len()
        ),
        id: fill_blank_set.id.to_string(),
        total_token_count: 0,
        documents_processed: fill_blank_set.questions.len(),
        questions: question_responses,
    }))
}

/// GET /api/intello/fill-blanks - List user's fill blank sets (with questions for playing)
#[get("/fill-blanks")]
#[instrument(skip(app, req))]
pub async fn list_fill_blank_sets_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    let sets = app
        .intello_service
        .get_user_fill_blank_sets(&user_id)
        .await?;

    let set_responses: Vec<FillBlankSetWithQuestionsResponse> = sets
        .iter()
        .map(|set| FillBlankSetWithQuestionsResponse {
            id: set.id.to_string(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set
                .questions
                .iter()
                .map(|q| FillBlankQuestionResponse {
                    id: q.id.to_string(),
                    phrase: q.phrase.clone(),
                    options: q
                        .options
                        .iter()
                        .map(FillBlankOptionResponse::from)
                        .collect(),
                    explanation: q.explanation.clone(),
                })
                .collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(FillBlankSetListResponse {
        count: set_responses.len(),
        sets: set_responses,
    }))
}
