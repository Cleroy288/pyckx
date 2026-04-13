use crate::services::ai_usage::ai_usage_domain::AiUsageInput;
use crate::services::course::domain::ExtractedIdeas;
use crate::services::course::parser::parse_extracted_ideas;
use crate::services::course::prompt::build_ideas_extraction_prompt;
use crate::services::error_domain::StudyError;
use crate::services::StudyService;
use tracing::{info, instrument};

impl StudyService {
    // ** extract_user_demand_ideas **
    // ==> Stage 1: Analyze user input to extract learning objectives
    //
    // @ user_input : Raw user topic/instructions
    // @ resources_content : Optional source documents
    // @ returns : ExtractedIdeas with analyzed demand
    // @ errors : ExternalServiceError if AI fails, ValidationFailed if parse fails
    #[instrument(skip(self, user_input, resources_content))]
    pub async fn extract_user_demand_ideas(
        &self,
        user_id: &str,
        user_input: &str,
        resources_content: Option<&str>,
    ) -> Result<ExtractedIdeas, StudyError> {
        info!("Starting ideas extraction");

        // Step 1: Build prompt for AI
        let prompt =
            build_ideas_extraction_prompt(user_input, resources_content);
        info!(prompt_len = prompt.len(), "Ideas prompt built");

        // Step 2: Send to AI
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;

        // Step 2b: Track AI usage
        if let Some(usage) = &ai_result.usage {
            self.try_log_ai_usage(AiUsageInput {
                user_id,
                model_id: &ai_result.model,
                feature_type: "course_ideas_extraction",
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
            })
            .await;
        }

        info!(
            response_len = ai_result.content.len(),
            "AI response received"
        );

        // Step 3: Parse AI response
        let ideas = parse_extracted_ideas(&ai_result.content)?;
        info!(
            mandatory_topics = ideas.mandatory_topics.len(),
            "Ideas extracted"
        );

        // Step 4: Return extracted ideas
        Ok(ideas)
    }
}
