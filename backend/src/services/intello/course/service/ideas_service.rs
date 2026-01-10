use crate::services::intello::course::domain::ExtractedIdeas;
use crate::services::intello::course::parser::parse_extracted_ideas;
use crate::services::intello::course::prompt::build_ideas_extraction_prompt;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;
use tracing::{info, instrument};

impl IntelloService {
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
    ) -> Result<ExtractedIdeas, IntelloError> {
        info!("Starting ideas extraction");

        // Step 1: Build prompt for AI
        let prompt = build_ideas_extraction_prompt(user_input, resources_content);
        info!(prompt_len = prompt.len(), "Ideas prompt built");

        // Step 2: Send to AI
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;

        // Step 2b: Track AI usage
        if let Some(usage) = &ai_result.usage {
            self.try_log_ai_usage(
                user_id,
                &ai_result.model,
                "course_ideas_extraction",
                usage.prompt_tokens,
                usage.completion_tokens,
            )
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
