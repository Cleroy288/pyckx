//! Flashcard domain types

use serde::Deserialize;

/// Single flashcard
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Flashcard {
    pub id: String,
    pub front: String,
    pub back: String,
}

/// Flashcard set with cards
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FlashcardSet {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub cards: Vec<Flashcard>,
}

/// List response for flashcard sets
#[derive(Debug, Clone, Deserialize)]
pub struct FlashcardSetListResponse {
    pub sets: Vec<FlashcardSet>,
    pub count: usize,
}

/// AI creation response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFlashcardResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub cards: Vec<Flashcard>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_flashcard_set() {
        let json = r#"{
            "id": "1", "user_id": "u",
            "name": "Vocab", "description": "D",
            "level": "medium", "language": "fr",
            "subjects": [],
            "cards": [
                {"id":"c1","front":"Q","back":"A"}
            ]
        }"#;
        let s: FlashcardSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.cards.len(), 1);
        assert_eq!(s.cards[0].front, "Q");
    }

    #[test]
    fn test_deserialize_create_response() {
        let json = r#"{
            "success": true, "message": "ok",
            "id": "x",
            "total_token_count": 500,
            "documents_processed": 2,
            "cards": []
        }"#;
        let r: CreateFlashcardResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.success);
        assert_eq!(r.total_token_count, 500);
    }

    #[test]
    fn test_deserialize_list_response() {
        let json = r#"{"sets": [], "count": 0}"#;
        let r: FlashcardSetListResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(r.count, 0);
    }
}
