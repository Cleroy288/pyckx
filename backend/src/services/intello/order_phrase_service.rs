//! Order Phrase operations

use super::crud_service;
use super::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::order_phrase_domain::OrderPhraseSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::infra::openrouter::DEFAULT_MODEL;
use super::prompt_builder_service::OrderPhrasePromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all order phrase sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_order_phrase_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        crud_service::get_user_sets(self.order_phrase_repo.as_ref(), user_id, "order_phrase").await
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

        info!(
            num_questions = input.num_questions,
            "Generating AI order phrase questions"
        );

        // Build prompt input
        let prompt_input = OrderPhrasePromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_questions: input.num_questions,
            documents: input
                .documents
                .iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Build prompt and send request
        let prompt = super::prompt_builder_service::build_order_phrase_prompt(&prompt_input);
        let ai_result = self.openrouter_client.send_chat_request(&prompt, None).await?;

        // Parse the response
        let questions = crate::services::intello::ai_parsing_service::parse_order_phrase_response(&ai_result.content)?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::ORDER_PHRASE,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(
            generated = questions.len(),
            "AI order phrase questions generated"
        );

        // Build and store the set
        let order_phrase_set = OrderPhraseSet {
            id: SetId::new(),
            user_id: user_id.into(),
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
