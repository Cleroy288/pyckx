//! Generate Flashcards Use Case
//!
//! Orchestrates AI-powered flashcard generation.
//! Uses shared input/validation from `super::shared`.

use super::shared::{GenerateGameInput, GenerateGameOutput};
use crate::domain::intello::FlashcardSet;
use crate::error::{AppError, AppResult};
use crate::services::{validate_model, validate_token_count, GenerateContentInput, IntelloService, DEFAULT_MODEL};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateFlashcardsInput;

/// Output from flashcard generation
pub type GenerateFlashcardsOutput = GenerateGameOutput<FlashcardSet>;

/// Use case for generating flashcard sets via AI
pub struct GenerateFlashcardsUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateFlashcardsUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateFlashcardsInput) -> AppResult<GenerateFlashcardsOutput> {
        // Flashcards have custom validation: allows up to 100 cards vs standard 50
        // Using inline validation instead of shared to preserve this difference
        if let Some(ref model) = input.model {
            validate_model(model).map_err(|e| AppError::validation("model", e))?;
        }

        let total_token_count: u32 = input.documents.iter().map(|(_, _, t)| t).sum();
        let model_to_check = input.model.as_deref().unwrap_or(DEFAULT_MODEL);
        validate_token_count(model_to_check, total_token_count)
            .map_err(|e| AppError::validation("token_count", e))?;

        if input.subjects.is_empty() {
            return Err(AppError::validation("subjects", "At least one subject is required"));
        }

        // Flashcards-specific: allow up to 100 cards
        if input.num_questions == 0 || input.num_questions > 100 {
            return Err(AppError::validation("num_questions", "Must be between 1 and 100"));
        }

        info!(
            num_cards = input.num_questions,
            model = ?input.model,
            documents = input.documents.len(),
            "Generating AI flashcards via use case"
        );

        // Build service input
        let service_input = GenerateContentInput {
            name: input.name,
            description: input.description,
            instructions: input.instructions,
            language: input.language,
            level: input.level,
            subjects: input.subjects,
            num_questions: input.num_questions,
            documents: input.documents.clone(),
            model: input.model,
        };

        // Delegate to service
        let flashcard_set = self
            .intello_service
            .generate_ai_flashcards(&input.user_id, service_input)
            .await?;

        info!(set_id = %flashcard_set.id, cards = flashcard_set.cards.len(), "Flashcards generated");

        Ok(GenerateFlashcardsOutput {
            game_set: flashcard_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

