//! Keywords operations

use tracing::{info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::ai_usage::ai_usage_domain::{
    feature_type, AiUsageInput,
};
use crate::services::error_domain::StudyError;
use crate::services::types_domain::GenerateContentInput;
use crate::services::StudyService;
use crate::services::SetId;

use super::domain::KeywordSet;
use super::prompt::build_keywords_prompt;
use crate::services::games::shared::prompt_helpers::GamePromptInput;
use crate::services::crud::crud_service;

impl StudyService {
    // ** get_user_keyword_sets **
    // ==> Retrieves all keyword sets for a user
    //
    // @ user_id : The user ID to query sets for
    // @ returns : Vector of all KeywordSets owned by the user
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_keyword_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<KeywordSet>, StudyError> {
        // Step 1: Query repository for all user keyword sets
        crud_service::get_user_sets(
            self.keywords_repo.as_ref(),
            user_id,
            "keywords",
        )
        .await
    }

    // ** generate_ai_keywords **
    // ==> Generates keyword questions using AI and stores them as a set
    //
    // @ user_id : The user requesting generation
    // @ input : Generation parameters (name, level, documents, etc.)
    // @ returns : The created KeywordSet with AI-generated questions
    // @ errors : ValidationFailed if inputs invalid, ExternalServiceError if AI fails
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_keywords(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<KeywordSet, StudyError> {
        // Step 1: Validate user ID and generation input
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        info!(
            num_questions = input.num_questions,
            "Generating AI keyword questions"
        );

        // Step 2: Build prompt input from generation parameters
        let prompt_input = GamePromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_items: input.num_questions.into(),
            documents: input
                .documents
                .iter()
                .map(|(filename, content, _)| {
                    (filename.clone(), content.clone())
                })
                .collect(),
        };

        // Step 3: Build prompt and send request to AI service
        let prompt = build_keywords_prompt(&prompt_input);
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;

        // Step 4: Parse AI response into keyword questions
        let questions =
            super::parser::parse_keywords_response(&ai_result.content)?;

        // Step 5: Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
            self.try_log_ai_usage(AiUsageInput {
                user_id,
                model_id: DEFAULT_MODEL,
                feature_type: feature_type::KEYWORDS,
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
            })
            .await;
        }
        info!(
            generated = questions.len(),
            "AI keyword questions generated"
        );

        // Step 6: Build and store the keyword set
        let keyword_set = KeywordSet {
            id: SetId::new(),
            user_id: user_id.into(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.keywords_repo.insert(&keyword_set).await?;
        info!(set_id = %created.id, "AI keyword set stored");
        Ok(created)
    }
}
