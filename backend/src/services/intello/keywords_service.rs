//! Keywords operations

use super::crud_service;
use super::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::keywords_domain::KeywordSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::infra::openrouter::DEFAULT_MODEL;
use super::prompt_builder_service::KeywordsPromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all keyword sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_keyword_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<KeywordSet>, IntelloError> {
        crud_service::get_user_sets(self.keywords_repo.as_ref(), user_id, "keywords").await
    }

    /// Generate AI keyword questions and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_keywords(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<KeywordSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            "Generating AI keyword questions"
        );

        // Build prompt input
        let prompt_input = KeywordsPromptInput {
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
        let prompt = super::prompt_builder_service::build_keywords_prompt(&prompt_input);
        let ai_result = self.openrouter_client.send_chat_request(&prompt, None).await?;

        // Parse the response
        let questions = crate::services::intello::ai_parsing_service::parse_keywords_response(&ai_result.content)?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::KEYWORDS,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(
            generated = questions.len(),
            "AI keyword questions generated"
        );

        // Build and store the set
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
