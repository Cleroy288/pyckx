//! Open question handlers
//!
//! List and check/grade open questions.

use crate::api::dto::intello::{
    CheckAnswersRequest, CheckAnswersResponse, GradedAnswerResponse, OpenQuestionSetListResponse,
};
use crate::app::App;
use crate::error::AppResult;
use crate::services::{CheckAnswersInput, IntelloAnswerGrade, UserAnswer};
use crate::shared::session::get_user_id_from_session;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tracing::{info, instrument};

/// GET /app/intello/open-question/list - Get user's open question sets
#[get("/open-question/list")]
#[instrument(skip(app, req))]
pub async fn list_open_questions_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let sets = app.intello_service.get_user_open_question_sets(&user_id).await?;
    Ok(HttpResponse::Ok().json(OpenQuestionSetListResponse::from_sets(sets)))
}

/// POST /app/intello/open-question/check - Grade user answers
#[post("/open-question/check")]
#[instrument(skip(app, req, body))]
pub async fn check_open_questions_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CheckAnswersRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    info!(set_id = %body.set_id, answers = body.answers.len(), "Checking open question answers");

    // Build input for service
    let input = CheckAnswersInput {
        set_id: body.set_id.clone(),
        answers: body.answers.iter().map(|a| UserAnswer {
            question_id: a.question_id.clone(),
            user_answer: a.user_answer.clone(),
        }).collect(),
    };

    // Delegate to service
    let grades = app.intello_service.check_open_question_answers(&user_id, input).await?;

    // Build response
    let grade_responses: Vec<GradedAnswerResponse> = grades.iter().map(|g| {
        GradedAnswerResponse {
            question_id: g.question_id.clone(),
            grade: match g.grade {
                IntelloAnswerGrade::Right => "right".to_string(),
                IntelloAnswerGrade::Medium => "medium".to_string(),
                IntelloAnswerGrade::Error => "error".to_string(),
            },
            feedback: g.feedback.clone(),
        }
    }).collect();

    Ok(HttpResponse::Ok().json(CheckAnswersResponse {
        success: true,
        message: format!("Graded {} answers", grades.len()),
        set_id: body.set_id.clone(),
        grades: grade_responses,
    }))
}
