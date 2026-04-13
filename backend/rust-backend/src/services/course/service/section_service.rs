use crate::services::ai_usage::ai_usage_domain::AiUsageInput;
use crate::services::course::domain::{ParsedSection, SectionPlan};
use crate::services::course::parser::parse_generated_section;
use crate::services::course::prompt::build_section_prompt;
use crate::services::error_domain::StudyError;
use crate::services::StudyService;
use tracing::{info, instrument};

impl StudyService {
    // ** generate_course_section **
    // ==> Stage 3: Generate content for a single section
    //
    // @ section_plan : SectionPlan from CoursePlan
    // @ context : Previous sections summary (for continuity)
    // @ resources : Source documents for this section
    // @ returns : ParsedSection with content and QCM
    // @ errors : ExternalServiceError if AI fails, ValidationFailed if parse fails
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self, section_plan, context, resources))]
    pub async fn generate_course_section(
        &self,
        user_id: &str,
        section_plan: &SectionPlan,
        context: Option<&str>,
        resources: Option<&str>,
    ) -> Result<ParsedSection, StudyError> {
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
            self.try_log_ai_usage(AiUsageInput {
                user_id,
                model_id: &ai_result.model,
                feature_type: "course_section_generation",
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
        let mut section = parse_generated_section(&ai_result.content)?;

        // Step 4: Set order from plan
        section.order = section_plan.order;

        info!(
            blocks = section.content_blocks.len(),
            qcm_questions = section.qcm_set.questions.len(),
            "Section generated"
        );

        // Step 5: Return parsed section
        Ok(section)
    }
}
