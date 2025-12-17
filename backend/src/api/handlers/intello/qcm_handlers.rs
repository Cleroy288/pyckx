//! QCM set handlers
//!
//! Create, get, update, delete QCM sets.

use crate::api::dto::intello::{
    CreateQcmSetRequest, QcmSetListResponse, QcmSetResponse, QcmSuccessResponse, UpdateQcmSetRequest,
};
use crate::app::App;
use crate::domain::intello::{QcmQuestion, QcmSet};
use crate::error::{AppError, AppResult};
use crate::shared::session::get_user_id_from_session;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// POST /app/intello/qcm - Create a new manual QCM set
#[post("/qcm")]
#[instrument(skip(app, req, body))]
pub async fn create_qcmset_handler(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Json<CreateQcmSetRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;

    let level = body.parse_level().map_err(|e| AppError::validation("level", e))?;
    body.validate_subjects().map_err(|e| AppError::validation("subjects", e))?;

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user_id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        level,
        language: body.language.clone(),
        subjects: body.subjects.clone(),
        questions: body.questions.iter().map(|q| QcmQuestion {
            id: uuid::Uuid::new_v4().to_string(),
            question: q.question.clone(),
            wrong_answers: q.wrong_answers.clone(),
            right_answer: q.right_answer.clone(),
            explanation: q.explanation.clone(),
        }).collect(),
    };

    let created = app.intello_service.create_qcm_set(qcmset).await?;
    Ok(HttpResponse::Created().json(QcmSuccessResponse::created(&created)))
}

/// GET /app/intello/qcm - Get all user's QCM sets
#[get("/qcm")]
#[instrument(skip(app, req))]
pub async fn get_user_qcmsets_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let sets = app.intello_service.get_user_qcm_sets(&user_id).await?;
    Ok(HttpResponse::Ok().json(QcmSetListResponse::from_sets(sets)))
}

/// GET /app/intello/qcm/{id} - Get a specific QCM set
#[get("/qcm/{set_id}")]
#[instrument(skip(app, req), fields(set_id = %path.as_str()))]
pub async fn get_qcmset_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let set_id = path.into_inner();

    match app.intello_service.get_qcm_set(&set_id, &user_id).await? {
        Some(s) => Ok(HttpResponse::Ok().json(QcmSetResponse::from(&s))),
        None => Ok(HttpResponse::NotFound().json(QcmSuccessResponse::not_found())),
    }
}

/// PUT /app/intello/qcm/{id} - Update a QCM set
#[put("/qcm/{set_id}")]
#[instrument(skip(app, req, body), fields(set_id = %path.as_str()))]
pub async fn update_qcmset_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateQcmSetRequest>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let set_id = path.into_inner();

    let existing = match app.intello_service.get_qcm_set(&set_id, &user_id).await? {
        Some(s) => s,
        None => return Ok(HttpResponse::NotFound().json(QcmSuccessResponse::not_found())),
    };

    let level = body.parse_level()
        .map_err(|e| AppError::validation("level", e))?
        .unwrap_or(existing.level.clone());
    body.validate_subjects().map_err(|e| AppError::validation("subjects", e))?;

    let updated_set = QcmSet {
        id: set_id.clone(),
        user_id: user_id.clone(),
        name: body.name.clone().unwrap_or(existing.name),
        description: body.description.clone().unwrap_or(existing.description),
        level,
        language: body.language.clone().unwrap_or(existing.language),
        subjects: body.subjects.clone().unwrap_or(existing.subjects),
        questions: match &body.questions {
            Some(qs) => qs.iter().map(|q| QcmQuestion {
                id: uuid::Uuid::new_v4().to_string(),
                question: q.question.clone(),
                wrong_answers: q.wrong_answers.clone(),
                right_answer: q.right_answer.clone(),
                explanation: q.explanation.clone(),
            }).collect(),
            None => existing.questions,
        },
    };

    if app.intello_service.update_qcm_set(updated_set.clone()).await? {
        Ok(HttpResponse::Ok().json(QcmSuccessResponse::updated(&updated_set)))
    } else {
        Ok(HttpResponse::NotFound().json(QcmSuccessResponse::not_found()))
    }
}

/// DELETE /app/intello/qcm/{id} - Delete a QCM set
#[delete("/qcm/{set_id}")]
#[instrument(skip(app, req), fields(set_id = %path.as_str()))]
pub async fn delete_qcmset_handler(
    app: web::Data<App>,
    req: HttpRequest,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let set_id = path.into_inner();

    if app.intello_service.delete_qcm_set(&set_id, &user_id).await? {
        Ok(HttpResponse::Ok().json(QcmSuccessResponse::deleted()))
    } else {
        Ok(HttpResponse::NotFound().json(QcmSuccessResponse::not_found()))
    }
}
