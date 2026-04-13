//! Games handler

use crate::app::App;
use crate::http_api::data_transfer_object::{
    AvailableGamesResponse, CheckCodingRequest,
    CheckCodingResponse, GameResponse,
    GenerateCodingGameRequest, GenerateCodingGameResponse,
};
use crate::services::games::coding::service::CodingGame;
use actix_web::{get, post, web, HttpResponse};
use tracing::instrument;

/// POST /api/study/games/coding/generate
#[post("/games/coding/generate")]
#[instrument(skip(app))]
pub async fn generate_coding_game_handler(
    app: web::Data<App>,
    body: web::Json<GenerateCodingGameRequest>,
) -> HttpResponse {
    let langue = body.langue.as_deref().unwrap_or("eng");
    let subject = body.subject.as_deref().unwrap_or("");
    let game = CodingGame::new((*app.openrouter_client).clone());
    match game.generate(&body.language, subject, &body.level, langue).await {
        Ok(exercise) => HttpResponse::Ok().json(GenerateCodingGameResponse {
            subject: exercise.subject,
            code_snippet: exercise.code_snippet,
        }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

/// POST /api/study/games/coding/check
#[post("/games/coding/check")]
#[instrument(skip(_app))]
pub async fn check_coding_game_handler(
    _app: web::Data<App>,
    body: web::Json<CheckCodingRequest>,
) -> HttpResponse {
    let is_correct =
        CodingGame::check(&body.expected, &body.user_code);
    HttpResponse::Ok().json(CheckCodingResponse { is_correct })
}

/// GET /api/study/games
#[get("/games")]
#[instrument(skip(app))]
pub async fn get_available_games_handler(app: web::Data<App>) -> HttpResponse {
    let games = app.study_service.get_available_games();
    let game_responses: Vec<GameResponse> =
        games.iter().map(GameResponse::from).collect();
    HttpResponse::Ok().json(AvailableGamesResponse {
        games: game_responses,
        count: games.len(),
    })
}
