use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::CoursePlan;
use crate::services::intello::error_domain::IntelloError;

// ** parse_course_plan **
// ==> Parses AI response JSON into CoursePlan struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : CoursePlan if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or sections are empty
pub fn parse_course_plan(ai_response: &str) -> Result<CoursePlan, IntelloError> {
    // Step 1: Extract JSON from AI response
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let plan: CoursePlan = serde_json::from_str(&sanitized).map_err(|e| {
        IntelloError::validation("plan_json", format!("Failed to parse plan JSON: {}", e))
    })?;

    // Step 2: Validate course plan structure
    if plan.title.is_empty() {
        return Err(IntelloError::validation(
            "title",
            "Course title cannot be empty",
        ));
    }

    if plan.sections.is_empty() {
        return Err(IntelloError::validation(
            "sections",
            "Course must have at least one section",
        ));
    }

    // Step 3: Validate each section
    for section in &plan.sections {
        if section.title.is_empty() {
            return Err(IntelloError::validation(
                "section_title",
                "Section title cannot be empty",
            ));
        }

        if section.key_concepts.is_empty() {
            return Err(IntelloError::validation(
                "key_concepts",
                "Section must have at least one key concept",
            ));
        }

        if section.qcm_count == 0 {
            return Err(IntelloError::validation(
                "qcm_count",
                "Section must have at least 1 QCM question",
            ));
        }
    }

    // Step 4: Return validated plan
    Ok(plan)
}
