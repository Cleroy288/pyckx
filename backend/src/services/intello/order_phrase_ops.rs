//! Order Phrase operations

use super::crud_ops;
use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::OrderPhraseSet;
use crate::error::IntelloError;
use crate::shared::OrderPhrasePromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all order phrase sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_order_phrase_sets(&self, user_id: &str) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        crud_ops::get_user_sets(self.order_phrase_repo.as_ref(), user_id, "order_phrase").await
    }

    /// Delete an order phrase set with ownership verification
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_order_phrase_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        crud_ops::delete_set(self.order_phrase_repo.as_ref(), set_id, user_id, "order_phrase").await
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
