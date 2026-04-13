//! Generic games operations

use crate::services::StudyService;
use crate::services::{GameInstance, AVAILABLE_GAMES};

impl StudyService {
    /// Get available games
    pub fn get_available_games(&self) -> &'static [GameInstance] {
        AVAILABLE_GAMES
    }
}
