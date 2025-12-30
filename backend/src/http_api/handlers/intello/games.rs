//! Games handler

use crate::http_api::data_transfer_object::intello::{AvailableGamesResponse, GameResponse};
use crate::app::App;
use actix_web::{get, web, HttpResponse};
use tracing::instrument;

/// GET /api/intello/games
#[get("/games")]
#[instrument(skip(app))]
pub async fn get_available_games_handler(app: web::Data<App>) -> HttpResponse {
    let games = app.intello_service.get_available_games();
    let game_responses: Vec<GameResponse> = games.iter().map(GameResponse::from).collect();
    HttpResponse::Ok().json(AvailableGamesResponse {
        games: game_responses,
        count: games.len(),
    })
}
