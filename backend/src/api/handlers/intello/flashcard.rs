//! Flashcard handlers
//!
//! List flashcard sets.

use crate::api::dto::intello::FlashcardSetListResponse;
use crate::app::App;
use crate::error::AppResult;
use crate::shared::session::get_user_id_from_session;
use actix_web::{get, web, HttpRequest, HttpResponse};
use tracing::instrument;

/// GET /api/intello/flashcards - Get user's flashcard sets
#[get("/flashcards")]
#[instrument(skip(app, req))]
pub async fn list_flashcards_handler(
    app: web::Data<App>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let sets = app.intello_service.get_user_flashcard_sets(&user_id).await?;
    Ok(HttpResponse::Ok().json(FlashcardSetListResponse::from_sets(sets)))
}
