//! Order Phrase operations

use tracing::{info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::intello::ai_usage::ai_usage_domain::feature_type;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::SetId;

use super::domain::OrderPhraseSet;
use super::prompt::{build_order_phrase_prompt, OrderPhrasePromptInput};
use crate::services::intello::crud::crud_service;

impl IntelloService {
    // ** get_user_order_phrase_sets **
    // ==> Retrieves all order phrase sets for a user
    //
    // @ user_id : The user ID to query sets for
    // @ returns : Vector of all OrderPhraseSets owned by the user
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_order_phrase_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        // Step 1: Query repository for all user order phrase sets
        crud_service::get_user_sets(self.order_phrase_repo.as_ref(), user_id, "order_phrase").await
    }

    // ** generate_ai_order_phrases **
    // ==> Generates order phrase questions using AI and stores them
    //
    // @ user_id : The user requesting generation
    // @ input : Generation parameters (name, level, documents, etc.)
    // @ returns : The created OrderPhraseSet with AI-generated questions
    // @ errors : ValidationFailed if inputs invalid, ExternalServiceError if AI fails
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_order_phrases(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<OrderPhraseSet, IntelloError> {
        // Step 1: Validate user ID and generation input
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        info!(
            num_questions = input.num_questions,
            "Generating AI order phrase questions"
        );

        // Step 2: Build prompt input from generation parameters
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

        // Step 3: Build prompt and send request to AI service
        let prompt = build_order_phrase_prompt(&prompt_input);
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;

        // Step 4: Parse AI response into order phrase questions
        let questions = super::parser::parse_order_phrase_response(&ai_result.content)?;

        // Step 5: Log AI usage (fire-and-forget)
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

        // Step 6: Build and store the order phrase set
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
