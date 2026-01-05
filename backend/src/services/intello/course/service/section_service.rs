use crate::services::intello::course::domain::{ParsedSection, SectionPlan};
use crate::services::intello::course::prompt::build_section_prompt;
use crate::services::intello::course::parser::parse_generated_section;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;
use tracing::{info, instrument};

impl IntelloService {
    // ** generate_course_section **
    // ==> Stage 3: Generate content for a single section
    //
    // @ section_plan : SectionPlan from CoursePlan
    // @ context : Previous sections summary (for continuity)
    // @ resources : Source documents for this section
    // @ returns : ParsedSection with content and QCM
    // @ errors : ExternalServiceError if AI fails, ValidationFailed if parse fails
    #[instrument(skip(self, section_plan, context, resources))]
    pub async fn generate_course_section(
        &self,
        user_id: &str,
        section_plan: &SectionPlan,
        context: Option<&str>,
        resources: Option<&str>,
    ) -> Result<ParsedSection, IntelloError> {
        info!(section_order = section_plan.order, title = %section_plan.title, "Starting section generation");

        // Step 1: Build prompt for AI
        let prompt = build_section_prompt(section_plan, context, resources);
        info!(prompt_len = prompt.len(), "Section prompt built");

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
                "course_section_generation", 
                usage.prompt_tokens, 
                usage.completion_tokens
            ).await;
        }

        info!(response_len = ai_result.content.len(), "AI response received");

        // Step 3: Parse AI response
        let mut section = parse_generated_section(&ai_result.content)?;
        
        // Step 4: Set order from plan
        section.order = section_plan.order;
        
        info!(blocks = section.content_blocks.len(), qcm_questions = section.qcm_set.questions.len(), "Section generated");

        // Step 5: Return parsed section
        Ok(section)
    }
}
