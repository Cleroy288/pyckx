//! Games DTOs for Intello

use crate::services::intello::GameInstance;
use serde::Serialize;

/// Response for a single game
#[derive(Debug, Serialize)]
pub struct GameResponse {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

impl From<&'static GameInstance> for GameResponse {
    fn from(game: &'static GameInstance) -> Self {
        Self {
            id: game.id,
            name: game.name,
            description: game.description,
        }
    }
}

/// Response for available games in Intello
#[derive(Debug, Serialize)]
pub struct AvailableGamesResponse {
    pub games: Vec<GameResponse>,
    pub count: usize,
}
