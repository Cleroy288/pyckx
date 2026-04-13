use crate::infra::openrouter::{
    extract_json_from_response, sanitize_ai_json,
};
use crate::services::course::domain::{
    CoursePlan, SectionPlan,
};
use crate::services::error_domain::StudyError;

/// Parses AI response JSON into a CoursePlan struct
pub fn parse_course_plan(
    ai_response: &str,
) -> Result<CoursePlan, StudyError> {
    let extracted =
        extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    let plan: CoursePlan =
        serde_json::from_str(&sanitized).map_err(
            |err| {
                StudyError::validation(
                    "plan_json",
                    format!(
                        "Failed to parse: {}",
                        err
                    ),
                )
            },
        )?;

    validate_plan_structure(&plan)?;
    Ok(plan)
}

/// Validate plan title, sections, and each section
fn validate_plan_structure(
    plan: &CoursePlan,
) -> Result<(), StudyError> {
    if plan.title.is_empty() {
        return Err(StudyError::validation(
            "title",
            "Course title cannot be empty",
        ));
    }
    if plan.sections.is_empty() {
        return Err(StudyError::validation(
            "sections",
            "Course must have at least one section",
        ));
    }
    for section in &plan.sections {
        validate_section_plan(section)?;
    }
    Ok(())
}

/// Validate a single section plan entry
fn validate_section_plan(
    section: &SectionPlan,
) -> Result<(), StudyError> {
    if section.title.is_empty() {
        return Err(StudyError::validation(
            "section_title",
            "Section title cannot be empty",
        ));
    }
    if section.key_concepts.is_empty() {
        return Err(StudyError::validation(
            "key_concepts",
            "Section must have at least one concept",
        ));
    }
    if section.qcm_count == 0 {
        return Err(StudyError::validation(
            "qcm_count",
            "Section must have at least 1 QCM",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build valid course plan JSON
    fn valid_plan_json() -> String {
        serde_json::json!({
            "title": "Rust Fundamentals",
            "subtitle": "Learn the basics of Rust",
            "sections": [{
                "order": 1,
                "title": "Ownership",
                "key_concepts": ["move", "borrow", "copy"],
                "qcm_count": 3
            }]
        })
        .to_string()
    }

    #[test]
    fn test_parse_plan_valid_returns_ok() {
        let json = valid_plan_json();
        let result = parse_course_plan(&json);
        assert_eq!(
            result.unwrap().title,
            "Rust Fundamentals"
        );
    }

    #[test]
    fn test_parse_plan_invalid_json_returns_error() {
        let result = parse_course_plan("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_empty_title_returns_error() {
        let json = serde_json::json!({
            "title": "",
            "subtitle": "desc",
            "sections": [{
                "order": 1,
                "title": "S1",
                "key_concepts": ["c1"],
                "qcm_count": 3
            }]
        })
        .to_string();
        let result = parse_course_plan(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_empty_sections_returns_error() {
        let json = serde_json::json!({
            "title": "Title",
            "subtitle": "desc",
            "sections": []
        })
        .to_string();
        let result = parse_course_plan(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_zero_qcm_returns_error() {
        let json = serde_json::json!({
            "title": "Title",
            "subtitle": "desc",
            "sections": [{
                "order": 1,
                "title": "S1",
                "key_concepts": ["c1"],
                "qcm_count": 0
            }]
        })
        .to_string();
        let result = parse_course_plan(&json);
        assert!(result.is_err());
    }
}
