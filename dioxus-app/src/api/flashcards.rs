//! Flashcards API — list and AI generation

use super::{endpoints, helpers};
use crate::domain::flashcard_types::*;

helpers::impl_sets_response!(
    FlashcardSetListResponse, FlashcardSet
);

/// Get all flashcard sets
pub async fn get_flashcard_sets(
) -> Result<Vec<FlashcardSet>, String> {
    helpers::get_sets::<FlashcardSetListResponse>(
        endpoints::FLASHCARDS,
    )
    .await
}

/// Create flashcards via AI (multipart upload)
pub async fn create_flashcards(
    form: &web_sys::FormData,
) -> Result<CreateFlashcardResponse, String> {
    helpers::post_multipart(
        endpoints::FLASHCARDS,
        form,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::helpers::SetsResponse;

    #[test]
    fn test_into_sets_returns_inner_vec() {
        // Arrange
        let resp = FlashcardSetListResponse {
            sets: vec![FlashcardSet {
                id: "f1".into(),
                user_id: "u1".into(),
                name: "Cards".into(),
                description: "D".into(),
                level: "easy".into(),
                language: "en".into(),
                subjects: vec![],
                cards: vec![],
            }],
            count: 1,
        };

        // Act
        let sets = resp.into_sets();

        // Assert
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0].id, "f1");
    }

    #[test]
    fn test_into_sets_empty_response() {
        // Arrange
        let resp = FlashcardSetListResponse {
            sets: vec![],
            count: 0,
        };

        // Act
        let sets = resp.into_sets();

        // Assert
        assert!(sets.is_empty());
    }
}
