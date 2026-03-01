//! Fill Blank operations

use tracing::{info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::intello::ai_usage::ai_usage_domain::{
    feature_type, AiUsageInput,
};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::types_domain::{
    GenerateContentInput, IntelloService,
};
use crate::services::intello::SetId;

use super::domain::FillBlankSet;
use super::prompt::{build_fill_blank_prompt, FillBlankPromptInput};
use crate::services::intello::crud::crud_service;

impl IntelloService {
    // ** get_user_fill_blank_sets **
    // ==> Retrieves all fill-in-the-blank sets for a user
    //
    // @ user_id : The user ID to query sets for
    // @ returns : Vector of all FillBlankSets owned by the user
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_fill_blank_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<FillBlankSet>, IntelloError> {
        // Step 1: Query repository for all user fill-in-the-blank sets
        crud_service::get_user_sets(
            self.fill_blank_repo.as_ref(),
            user_id,
            "fill_blank",
        )
        .await
    }

    // ** generate_ai_fill_blank **
    // ==> Generates fill-in-the-blank questions using AI and stores them
    //
    // @ user_id : The user requesting generation
    // @ input : Generation parameters (name, level, documents, etc.)
    // @ returns : The created FillBlankSet with AI-generated questions
    // @ errors : ValidationFailed if inputs invalid, ExternalServiceError if AI fails
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_fill_blank(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<FillBlankSet, IntelloError> {
        // Step 1: Validate user ID and generation input
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        info!(
            num_questions = input.num_questions,
            "Generating AI fill blank questions"
        );

        // Step 2: Build prompt input from generation parameters
        let prompt_input = FillBlankPromptInput {
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
                .map(|(filename, content, _)| {
                    (filename.clone(), content.clone())
                })
                .collect(),
        };

        // Step 3: Build prompt and send request to AI service
        let prompt = build_fill_blank_prompt(&prompt_input);
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;

        // Step 4: Parse AI response into fill-in-the-blank questions
        let questions =
            super::parser::parse_fill_blank_response(&ai_result.content)?;

        // Step 5: Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
            self.try_log_ai_usage(AiUsageInput {
                user_id,
                model_id: DEFAULT_MODEL,
                feature_type: feature_type::FILL_BLANK,
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
            })
            .await;
        }
        info!(
            generated = questions.len(),
            "AI fill blank questions generated"
        );

        // Step 6: Build and store the fill-in-the-blank set
        let fill_blank_set = FillBlankSet {
            id: SetId::new(),
            user_id: user_id.into(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.fill_blank_repo.insert(&fill_blank_set).await?;
        info!(set_id = %created.id, "AI fill blank set stored");
        Ok(created)
    }
}
