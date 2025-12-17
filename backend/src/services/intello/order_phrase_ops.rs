//! Order Phrase operations

use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::OrderPhraseSet;
use crate::error::IntelloError;
use crate::shared::OrderPhrasePromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all order phrase sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_order_phrase_sets(&self, user_id: &str) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        let sets = self.order_phrase_repo.find_by_user(user_id).await?;
        info!(count = sets.len(), "Retrieved user order phrase sets");
        Ok(sets)
    }

    /// Get a specific order phrase set by ID
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_order_phrase_set(&self, set_id: &str, user_id: &str) -> Result<Option<OrderPhraseSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;
        self.order_phrase_repo.find_by_id(set_id, user_id).await
    }

    /// Delete an order phrase set with ownership verification
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_order_phrase_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Verify ownership
        let existing = self.order_phrase_repo.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            tracing::warn!(set_id = %set_id, "Order phrase set not found or user doesn't own it");
            return Ok(false);
        }

        let deleted = self.order_phrase_repo.delete(set_id, user_id).await?;
        if deleted {
            info!(set_id = %set_id, "Order phrase set deleted");
        }
        Ok(deleted)
    }

    /// Generate AI order phrase questions and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_order_phrases(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<OrderPhraseSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(num_questions = input.num_questions, model = ?input.model, "Generating AI order phrase questions");

        // Build prompt input
        let prompt_input = OrderPhrasePromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_questions: input.num_questions,
            documents: input.documents.iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate questions via AI
        let questions = self.openrouter_service.generate_order_phrases(&prompt_input, input.model.as_deref()).await?;

        info!(generated = questions.len(), "AI order phrase questions generated");

        // Build and store the set
        let order_phrase_set = OrderPhraseSet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.order_phrase_repo.insert(&order_phrase_set).await?;
        info!(set_id = %created.id, "AI order phrase set stored");

        Ok(created)
    }
}
