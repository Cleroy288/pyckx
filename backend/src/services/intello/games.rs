//! Games listing

use super::types::IntelloService;
use crate::apps::intello::{GameInstance, AVAILABLE_GAMES};

impl IntelloService {
    /// Get available games
    pub fn get_available_games(&self) -> &'static [GameInstance] {
        AVAILABLE_GAMES
    }
}
