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
        Self {
            id,
            name,
            description,
        }
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
    GameInstance::new(
        "fill_blank",
        "Fill in the Blank",
        "Complete phrases by selecting the correct answer from multiple options",
    ),
    GameInstance::new(
        "flashcard",
        "Flashcards",
        "Memorization cards with front/back content for study and review",
    ),
    GameInstance::new(
        "keywords",
        "Keywords",
        "Identify which keywords are related to given statements",
    ),
    GameInstance::new(
        "order_phrase",
        "Order Phrase",
        "Arrange shuffled words into the correct order to form coherent phrases",
    ),
    GameInstance::new(
        "open_question",
        "Open Questions",
        "Answer open-ended questions with your own written responses",
    ),
];
