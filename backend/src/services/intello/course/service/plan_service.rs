use crate::services::intello::course::domain::{CoursePlan, ExtractedIdeas};
use crate::services::intello::course::prompt::build_course_plan_prompt;
use crate::services::intello::course::parser::parse_course_plan;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;
use tracing::{info, instrument};

impl IntelloService {
    // ** generate_course_plan **
    // ==> Stage 2: Create structured course outline from extracted ideas
    //
    // @ ideas : ExtractedIdeas from Stage 1
    // @ section_count : Desired number of sections (default: 4)
    // @ returns : CoursePlan with title and section plans
    // @ errors : ExternalServiceError if AI fails, ValidationFailed if parse fails
    #[instrument(skip(self, ideas))]
    pub async fn generate_course_plan(
        &self,
        user_id: &str,
        ideas: &ExtractedIdeas,
        section_count: u8,
    ) -> Result<CoursePlan, IntelloError> {
        info!(section_count, "Starting course plan generation");

        // Step 1: Build prompt for AI
        let prompt = build_course_plan_prompt(ideas, section_count);
        info!(prompt_len = prompt.len(), "Plan prompt built");

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
                "course_plan_generation", 
                usage.prompt_tokens, 
                usage.completion_tokens
            ).await;
        }

        info!(response_len = ai_result.content.len(), "AI response received");

        // Step 3: Parse AI response
        let plan = parse_course_plan(&ai_result.content)?;
        info!(sections = plan.sections.len(), title = %plan.title, "Course plan generated");

        // Step 4: Return course plan
        Ok(plan)
    }
}
