//! Flashcard handlers
//!
//! List flashcard sets.

use crate::app::App;
use crate::http_api::data_transfer_object::FlashcardSetListResponse;
use crate::infra::user::get_user_id_from_session;
use crate::shared::AppResult;
use actix_web::{get, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// GET /api/study/flashcards - Get user's flashcard sets
#[get("/flashcards")]
#[instrument(skip(app, req))]
pub async fn list_flashcards_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req).await?;
    let sets = app
        .study_service
        .get_user_flashcard_sets(&user_id)
        .await?;
    Ok(HttpResponse::Ok().json(FlashcardSetListResponse::from_sets(sets)))
}
