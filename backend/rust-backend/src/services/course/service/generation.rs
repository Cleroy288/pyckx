//! Course Generation Service
//!
//! Generates comprehensive courses with multiple stages (Ideas -> Plan -> Sections -> Synthesis).

use tracing::{info, instrument};

use crate::services::course::domain::{
    GenerateCourseInput, GeneratedCourse,
};
use crate::services::course::service::assemble_complete_course;
use crate::services::error_domain::StudyError;
use crate::services::StudyService;

impl StudyService {
    // ** generate_course **
    // ==> Complete 5-stage course generation pipeline for higher quality
    //
    // @ user_id : User requesting course
    // @ input : GenerateCourseInput with topic and resources
    // @ section_count : Number of sections to generate (default: 4)
    // @ returns : GeneratedCourse
    // @ errors : ExternalServiceError if AI fails, ValidationFailed if parse fails
    #[instrument(skip(self, input), fields(topic = %input.topic))]
    pub async fn generate_course(
        &self,
        user_id: &str,
        input: GenerateCourseInput,
        section_count: Option<u8>,
    ) -> Result<GeneratedCourse, StudyError> {
        info!(user_id = %user_id, "Starting multi-stage course generation");

        // Step 1: Extract user demand ideas
        let ideas = self
            .extract_user_demand_ideas(
                user_id,
                &input.topic,
                Some(&input.resources),
            )
            .await?;
        info!(
            mandatory_topics = ideas.mandatory_topics.len(),
            "Ideas extracted"
        );

        // Step 2: Generate course plan
        let section_count = section_count.unwrap_or(4);
        let plan = self
            .generate_course_plan(user_id, &ideas, section_count)
            .await?;
        info!(sections = plan.sections.len(), title = %plan.title, "Course plan generated");

        // Step 3: Generate each section (concurrent HTTP calls to AI)
        // Note: Sections are generated sequentially to avoid rate limiting
        let mut sections = Vec::new();
        for section_plan in &plan.sections {
            let section = self
                .generate_course_section(
                    user_id,
                    section_plan,
                    None,
                    Some(&input.resources),
                )
                .await?;
            info!(section_order = section.order, "Section generated");
            sections.push(section);
        }
        info!(
            sections_generated = sections.len(),
            "All sections generated"
        );

        // Step 4: Generate synthesis
        let synthesis = self
            .generate_course_synthesis(user_id, &plan, &sections)
            .await?;
        info!(
            qcm_questions = synthesis.final_qcm.questions.len(),
            "Synthesis generated"
        );

        // Step 5: Assemble final course
        let course = assemble_complete_course(&plan, sections, synthesis)?;
        info!(
            modules = course.modules.len(),
            "Course assembled successfully"
        );

        // Step 6: Save to session if session_id provided
        if let Some(ref session_id) = input.session_id {
            info!(session_id = %session_id, "Saving course to session");

            let course_json = serde_json::to_value(&course).map_err(|err| {
                StudyError::validation("json_serialize", err.to_string())
            })?;

            self.study_session_repo
                .save_session_content(session_id, &course_json)
                .await
                .map_err(|err| StudyError::storage(err.to_string()))?;
        }

        // Step 7: Return final course
        Ok(course)
    }
}
