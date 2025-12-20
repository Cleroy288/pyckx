//! Generate Order Phrase Use Case
//!
//! Orchestrates AI-powered order phrase game generation.
//! Uses shared input/validation from `super::shared`.

use super::shared::{validate_generation_input, GenerateGameOutput};
use crate::domain::intello::OrderPhraseSet;
use crate::error::AppResult;
use crate::services::{GenerateContentInput, IntelloService};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateOrderPhraseInput;

/// Output from order phrase generation
pub type GenerateOrderPhraseOutput = GenerateGameOutput<OrderPhraseSet>;

/// Use case for generating order phrase sets via AI
pub struct GenerateOrderPhraseUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateOrderPhraseUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateOrderPhraseInput) -> AppResult<GenerateOrderPhraseOutput> {
        // Shared validation
        let total_token_count = validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            model = ?input.model,
            "Generating AI order phrase via use case"
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
        let order_phrase_set = self
            .intello_service
            .generate_ai_order_phrases(&input.user_id, service_input)
            .await?;

        info!(set_id = %order_phrase_set.id, "Order phrase set generated");

        Ok(GenerateOrderPhraseOutput {
            game_set: order_phrase_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

