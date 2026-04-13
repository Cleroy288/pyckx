//! Open question handlers
//!
//! List and check/grade open questions.

use crate::app::App;
use crate::http_api::data_transfer_object::{
    CheckAnswersRequest, CheckAnswersResponse, GradedAnswerResponse,
    OpenQuestionSetListResponse,
};
use crate::infra::user::get_user_id_from_session;
use crate::services::{CheckAnswersInput, UserAnswer};
use crate::shared::AppResult;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tracing::{info, instrument};

/// GET /api/study/open-questions - Get user's open question sets
#[get("/open-questions")]
#[instrument(skip(app, req))]
pub async fn list_open_questions_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let sets = app
        .study_service
        .get_user_open_question_sets(&user_id)
        .await?;
    Ok(HttpResponse::Ok().json(OpenQuestionSetListResponse::from_sets(sets)))
}

/// POST /api/study/open-questions/check - Grade user answers
#[post("/open-questions/check")]
#[instrument(skip(app, req, body))]
pub async fn check_open_questions_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CheckAnswersRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;

    info!(set_id = %body.set_id, answers = body.answers.len(), "Checking open question answers");

    // Build service input
    let input = CheckAnswersInput {
        set_id: body.set_id.clone(),
        answers: body
            .answers
            .iter()
            .map(|a| UserAnswer {
                question_id: a.question_id.clone(),
                user_answer: a.user_answer.clone(),
            })
            .collect(),
    };

    // Call service directly
    let results = app
        .study_service
        .check_open_question_answers(&user_id, input)
        .await?;

    // Build response
    let grade_responses: Vec<GradedAnswerResponse> = results
        .iter()
        .map(|g| GradedAnswerResponse {
            question_id: g.question_id.clone(),
            grade: format!("{:?}", g.grade).to_lowercase(),
            feedback: g.feedback.clone(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(CheckAnswersResponse {
        success: true,
        message: format!("Graded {} answers", results.len()),
        set_id: body.set_id.clone(),
        grades: grade_responses,
    }))
}
