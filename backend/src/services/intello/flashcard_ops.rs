//! Flashcard operations

use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::FlashcardSet;
use crate::error::IntelloError;
use crate::shared::FlashcardPromptInput;
use tracing::{info, instrument, warn};

impl IntelloService {
    /// Get all flashcard sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_flashcard_sets(&self, user_id: &str) -> Result<Vec<FlashcardSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        let sets = self.flashcard_repo.find_by_user(user_id).await?;
        info!(count = sets.len(), "Retrieved user flashcard sets");
        Ok(sets)
    }

    /// Get a specific flashcard set by ID
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_flashcard_set(&self, set_id: &str, user_id: &str) -> Result<Option<FlashcardSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;
        self.flashcard_repo.find_by_id(set_id, user_id).await
    }

    /// Delete a flashcard set with ownership verification
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_flashcard_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Verify ownership
        let existing = self.flashcard_repo.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            warn!(set_id = %set_id, "Flashcard set not found or user doesn't own it");
            return Ok(false);
        }

        let deleted = self.flashcard_repo.delete(set_id, user_id).await?;
        if deleted {
            info!(set_id = %set_id, "Flashcard set deleted");
        }
        Ok(deleted)
    }

    /// Generate AI flashcards and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_flashcards(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<FlashcardSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(num_cards = input.num_questions, model = ?input.model, "Generating AI flashcards");

        // Build prompt input
        let prompt_input = FlashcardPromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_cards: input.num_questions,
            documents: input.documents.iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate flashcards via AI
        let cards = self.openrouter_service.generate_flashcards(&prompt_input, input.model.as_deref()).await?;

        info!(generated = cards.len(), "AI flashcards generated");

        // Build and store the set
        let flashcard_set = FlashcardSet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            cards,
        };

        let created = self.flashcard_repo.insert(&flashcard_set).await?;
        info!(set_id = %created.id, "AI flashcard set stored");

        Ok(created)
    }
}
