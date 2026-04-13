use crate::services::ai_usage::ai_usage_domain::AiUsageInput;
use crate::services::course::domain::{
    CoursePlan, ParsedSection, ParsedSynthesis,
};
use crate::services::course::parser::parse_generated_synthesis;
use crate::services::course::prompt::build_synthesis_prompt;
use crate::services::error_domain::StudyError;
use crate::services::StudyService;
use tracing::{info, instrument};

impl StudyService {
    // ** generate_course_synthesis **
    // ==> Stage 4: Generate final synthesis with comprehensive QCM
    //
    // @ course_plan : Original CoursePlan
    // @ parsed_sections : All generated sections
    // @ returns : ParsedSynthesis with summary and final QCM
    // @ errors : ExternalServiceError if AI fails
    #[instrument(skip(self, course_plan, parsed_sections))]
    pub async fn generate_course_synthesis(
        &self,
        user_id: &str,
        course_plan: &CoursePlan,
        parsed_sections: &[ParsedSection],
    ) -> Result<ParsedSynthesis, StudyError> {
        info!(
            sections_count = parsed_sections.len(),
            "Starting synthesis generation"
        );

        // Step 1: Build prompt for AI
        let prompt = build_synthesis_prompt(course_plan, parsed_sections);
        info!(prompt_len = prompt.len(), "Synthesis prompt built");

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
                feature_type: "course_synthesis_generation",
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
        let synthesis = parse_generated_synthesis(&ai_result.content)?;

        info!(
            takeaways = synthesis.key_takeaways.len(),
            qcm_questions = synthesis.final_qcm.questions.len(),
            "Synthesis generated"
        );

        // Step 4: Return parsed synthesis
        Ok(synthesis)
    }
}
