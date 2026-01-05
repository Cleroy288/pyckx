//! Generic games operations

use crate::services::intello::types_domain::IntelloService;
use crate::services::intello::{GameInstance, AVAILABLE_GAMES};

impl IntelloService {
    /// Get available games
    pub fn get_available_games(&self) -> &'static [GameInstance] {
        AVAILABLE_GAMES
    }
}
