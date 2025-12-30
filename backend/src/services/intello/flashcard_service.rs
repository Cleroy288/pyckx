//! Flashcard operations

use super::crud_service;
use super::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::flashcard_domain::FlashcardSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::openrouter::DEFAULT_MODEL;
use super::prompt_builder_service::FlashcardPromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all flashcard sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_flashcard_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<FlashcardSet>, IntelloError> {
        crud_service::get_user_sets(self.flashcard_repo.as_ref(), user_id, "flashcards").await
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

        info!(num_cards = input.num_questions, "Generating AI flashcards");

        // Build prompt input
        let prompt_input = FlashcardPromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_cards: input.num_questions,
            documents: input
                .documents
                .iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate flashcards via AI
        let (cards, usage) = self
            .openrouter_service
            .generate_flashcards(&prompt_input, None)
            .await?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::FLASHCARD,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(generated = cards.len(), "AI flashcards generated");

        // Build and store the set
        let flashcard_set = FlashcardSet {
            id: SetId::new(),
            user_id: user_id.into(),
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
