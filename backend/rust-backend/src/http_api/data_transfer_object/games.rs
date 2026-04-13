//! Games DTOs for Study

use crate::services::GameInstance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GenerateCodingGameRequest {
    pub language: String,
    pub subject: Option<String>,
    pub level: String,
    pub langue: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GenerateCodingGameResponse {
    pub subject: String,
    pub code_snippet: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckCodingRequest {
    pub expected: String,
    pub user_code: String,
}

#[derive(Debug, Serialize)]
pub struct CheckCodingResponse {
    pub is_correct: bool,
}

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

/// Response for available games in Study
#[derive(Debug, Serialize)]
pub struct AvailableGamesResponse {
    pub games: Vec<GameResponse>,
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A static test game for lifetime requirements
    static TEST_GAME: GameInstance = GameInstance::new(
        "test_game",
        "Test Game",
        "A game for testing",
    );

    #[test]
    fn test_game_response_from_maps_id() {
        // arrange / act
        let resp = GameResponse::from(&TEST_GAME);

        // assert
        assert_eq!(resp.id, "test_game");
    }

    #[test]
    fn test_game_response_from_maps_name() {
        // arrange / act
        let resp = GameResponse::from(&TEST_GAME);

        // assert
        assert_eq!(resp.name, "Test Game");
    }

    #[test]
    fn test_game_response_from_maps_description() {
        // arrange / act
        let resp = GameResponse::from(&TEST_GAME);

        // assert
        assert_eq!(resp.description, "A game for testing");
    }
}
