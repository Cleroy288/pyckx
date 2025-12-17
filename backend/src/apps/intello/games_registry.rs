//! Games Registry - Central list of all available games in Intello
//!
//! This is the source of truth for valid games. Similar to the app registry.

use serde::Serialize;

// == GAME INSTANCE // ==

/// Metadata for a game instance
#[derive(Debug, Clone, Serialize)]
pub struct GameInstance {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

impl GameInstance {
    pub const fn new(id: &'static str, name: &'static str, description: &'static str) -> Self {
        Self { id, name, description }
    }
}

// == AVAILABLE GAMES REGISTRY // ==

/// All available games in Intello
pub const AVAILABLE_GAMES: &[GameInstance] = &[
    GameInstance::new(
        "qcm",
        "QCM",
        "Multiple Choice Questions - Test your knowledge with quizzes",
    ),
    GameInstance::new(
        "true_false",
        "True or False",
        "Determine if statements are true or false based on your study materials",
    ),
];
