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
    pub const fn new(
        id: &'static str,
        name: &'static str,
        description: &'static str,
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_instance_new_stores_fields() {
        // arrange / act
        let game = GameInstance::new("qcm", "QCM", "desc");

        // assert
        assert_eq!(game.id, "qcm");
        assert_eq!(game.name, "QCM");
        assert_eq!(game.description, "desc");
    }

    #[test]
    fn test_available_games_not_empty() {
        // arrange / act / assert
        assert!(!AVAILABLE_GAMES.is_empty());
    }

    #[test]
    fn test_available_games_contains_qcm() {
        // arrange / act
        let found = AVAILABLE_GAMES.iter().any(|g| g.id == "qcm");

        // assert
        assert!(found);
    }

    #[test]
    fn test_available_games_contains_flashcard() {
        // arrange / act
        let found =
            AVAILABLE_GAMES.iter().any(|g| g.id == "flashcard");

        // assert
        assert!(found);
    }

    #[test]
    fn test_available_games_all_have_nonempty_ids() {
        for game in AVAILABLE_GAMES {
            assert!(
                !game.id.is_empty(),
                "Game has empty id: {:?}",
                game
            );
        }
    }

    #[test]
    fn test_available_games_all_have_nonempty_names() {
        for game in AVAILABLE_GAMES {
            assert!(
                !game.name.is_empty(),
                "Game has empty name: {:?}",
                game
            );
        }
    }

    #[test]
    fn test_available_games_unique_ids() {
        // arrange
        let ids: Vec<&str> =
            AVAILABLE_GAMES.iter().map(|g| g.id).collect();
        let unique_count = {
            let mut set = std::collections::HashSet::new();
            ids.iter().filter(|id| set.insert(**id)).count()
        };

        // assert
        assert_eq!(ids.len(), unique_count);
    }

    #[test]
    fn test_available_games_count() {
        // arrange / act / assert
        assert_eq!(AVAILABLE_GAMES.len(), 7);
    }
}
