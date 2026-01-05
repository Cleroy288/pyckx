use crate::services::intello::course::domain::{CoursePlan, ParsedSection, ParsedSynthesis};
use crate::services::intello::course::prompt::build_synthesis_prompt;
use crate::services::intello::course::parser::parse_generated_synthesis;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;
use tracing::{info, instrument};

impl IntelloService {
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
    ) -> Result<ParsedSynthesis, IntelloError> {
        info!(sections_count = parsed_sections.len(), "Starting synthesis generation");

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
            self.try_log_ai_usage(
                user_id, 
                &ai_result.model, 
                "course_synthesis_generation", 
                usage.prompt_tokens, 
                usage.completion_tokens
            ).await;
        }

        info!(response_len = ai_result.content.len(), "AI response received");

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
