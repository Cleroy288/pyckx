//! Flashcard operations

use tracing::{info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::ai_usage::ai_usage_domain::{
    feature_type, AiUsageInput,
};
use crate::services::error_domain::StudyError;
use crate::services::types_domain::GenerateContentInput;
use crate::services::StudyService;
use crate::services::SetId;

use super::domain::FlashcardSet;
use super::prompt::build_flashcard_prompt;
use crate::services::games::shared::prompt_helpers::GamePromptInput;
use crate::services::crud::crud_service;

impl StudyService {
    // ** get_user_flashcard_sets **
    // ==> Retrieves all flashcard sets for a user
    //
    // @ user_id : The user ID to query sets for
    // @ returns : Vector of all FlashcardSets owned by the user
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_flashcard_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<FlashcardSet>, StudyError> {
        // Step 1: Query repository for all user flashcard sets
        crud_service::get_user_sets(
            self.flashcard_repo.as_ref(),
            user_id,
            "flashcards",
        )
        .await
    }

    // ** generate_ai_flashcards **
    // ==> Generates flashcards using AI and stores them as a set
    //
    // @ user_id : The user requesting generation
    // @ input : Generation parameters (name, level, documents, etc.)
    // @ returns : The created FlashcardSet with AI-generated cards
    // @ errors : ValidationFailed if inputs invalid, ExternalServiceError if AI fails
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_flashcards(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<FlashcardSet, StudyError> {
        // Step 1: Validate user ID and generation input
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        info!(
            num_questions = input.num_questions,
            "Generating AI flashcards"
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
        let prompt = build_flashcard_prompt(&prompt_input);
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;

        // Step 4: Parse AI response into flashcards
        let cards =
            super::parser::parse_flashcard_response(&ai_result.content)?;

        // Step 5: Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
            self.try_log_ai_usage(AiUsageInput {
                user_id,
                model_id: DEFAULT_MODEL,
                feature_type: feature_type::FLASHCARD,
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
            })
            .await;
        }
        info!(generated = cards.len(), "AI flashcards generated");

        // Step 6: Build and store the flashcard set
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
