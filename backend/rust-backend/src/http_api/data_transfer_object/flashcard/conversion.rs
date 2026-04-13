//! Flashcard DTO Conversions

use super::response::{
    FlashcardResponse, FlashcardSetListResponse, FlashcardSetResponse,
};
use crate::services::{Flashcard, FlashcardSet};

impl From<&Flashcard> for FlashcardResponse {
    fn from(card: &Flashcard) -> Self {
        Self {
            id: card.id.to_string(),
            front: card.front.clone(),
            back: card.back.clone(),
        }
    }
}

impl From<&FlashcardSet> for FlashcardSetResponse {
    fn from(set: &FlashcardSet) -> Self {
        Self {
            id: set.id.to_string(),
            user_id: set.user_id.to_string(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: set.level.to_string(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            cards: set.cards.iter().map(FlashcardResponse::from).collect(),
        }
    }
}

impl FlashcardSetListResponse {
    pub fn from_sets(sets: Vec<FlashcardSet>) -> Self {
        let count = sets.len();
        Self {
            sets: sets.iter().map(FlashcardSetResponse::from).collect(),
            count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{Level, QuestionId, SetId};
    use crate::infra::user::UserId;

    /// Build a test Flashcard
    fn test_card() -> Flashcard {
        Flashcard {
            id: QuestionId::from_string("card-1".into()),
            front: "Capital of France?".into(),
            back: "Paris".into(),
        }
    }

    /// Build a test FlashcardSet
    fn test_set() -> FlashcardSet {
        FlashcardSet {
            id: SetId::from_string("fset-1".into()),
            user_id: UserId::from("user-1"),
            name: "Geography".into(),
            description: "Countries".into(),
            level: Level::Medium,
            language: "en".into(),
            subjects: vec!["geo".into()],
            cards: vec![test_card()],
        }
    }

    #[test]
    fn test_flashcard_response_from_maps_fields() {
        // arrange
        let card = test_card();

        // act
        let resp = FlashcardResponse::from(&card);

        // assert
        assert_eq!(resp.id, "card-1");
        assert_eq!(resp.front, "Capital of France?");
        assert_eq!(resp.back, "Paris");
    }

    #[test]
    fn test_flashcard_set_response_from_maps_level() {
        // arrange
        let set = test_set();

        // act
        let resp = FlashcardSetResponse::from(&set);

        // assert
        assert_eq!(resp.level, "medium");
    }

    #[test]
    fn test_flashcard_set_response_from_maps_cards() {
        // arrange
        let set = test_set();

        // act
        let resp = FlashcardSetResponse::from(&set);

        // assert
        assert_eq!(resp.cards.len(), 1);
        assert_eq!(resp.name, "Geography");
    }

    #[test]
    fn test_flashcard_set_list_from_sets_count() {
        // arrange
        let sets = vec![test_set()];

        // act
        let resp = FlashcardSetListResponse::from_sets(sets);

        // assert
        assert_eq!(resp.count, 1);
        assert_eq!(resp.sets.len(), 1);
    }

    #[test]
    fn test_flashcard_set_list_empty() {
        // arrange
        let sets: Vec<FlashcardSet> = vec![];

        // act
        let resp = FlashcardSetListResponse::from_sets(sets);

        // assert
        assert_eq!(resp.count, 0);
    }
}
