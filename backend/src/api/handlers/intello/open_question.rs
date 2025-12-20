//! Open question handlers
//!
//! List and check/grade open questions.

use crate::api::dto::intello::{
    CheckAnswersRequest, CheckAnswersResponse, GradedAnswerResponse, OpenQuestionSetListResponse,
};
use crate::app::App;
use crate::error::AppResult;
use crate::shared::session::get_user_id_from_session;
use crate::use_cases::intello::{
    CheckAnswersUseCaseInput, CheckOpenQuestionAnswersUseCase, UserAnswerInput,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tracing::{info, instrument};

/// GET /api/intello/open-questions - Get user's open question sets
#[get("/open-questions")]
#[instrument(skip(app, req))]
pub async fn list_open_questions_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let sets = app.intello_service.get_user_open_question_sets(&user_id).await?;
    Ok(HttpResponse::Ok().json(OpenQuestionSetListResponse::from_sets(sets)))
}

/// POST /api/intello/open-questions/check - Grade user answers
#[post("/open-questions/check")]
#[instrument(skip(app, req, body))]
pub async fn check_open_questions_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CheckAnswersRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(set_id = %body.set_id, answers = body.answers.len(), "Checking open question answers");

    // Build use case input
    let input = CheckAnswersUseCaseInput {
        user_id,
        set_id: body.set_id.clone(),
        answers: body.answers.iter().map(|a| UserAnswerInput {
            question_id: a.question_id.clone(),
            user_answer: a.user_answer.clone(),
        }).collect(),
    };

    // Execute use case
    let use_case = CheckOpenQuestionAnswersUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;

    // Build response
    let grade_responses: Vec<GradedAnswerResponse> = output.results.iter().map(|g| {
        GradedAnswerResponse {
            question_id: g.question_id.clone(),
            grade: g.grade.clone(),
            feedback: g.feedback.clone(),
        }
    }).collect();

    Ok(HttpResponse::Ok().json(CheckAnswersResponse {
        success: true,
        message: format!("Graded {} answers", output.results.len()),
        set_id: body.set_id.clone(),
        grades: grade_responses,
    }))
}
