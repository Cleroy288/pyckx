use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::CoursePlan;
use crate::services::intello::error_domain::IntelloError;

// ** parse_course_plan **
// ==> Parses AI response JSON into CoursePlan struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : CoursePlan if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or sections are empty
pub fn parse_course_plan(
    ai_response: &str,
) -> Result<CoursePlan, IntelloError> {
    // Step 1: Extract JSON from AI response
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let plan: CoursePlan = serde_json::from_str(&sanitized).map_err(|err| {
        IntelloError::validation(
            "plan_json",
            format!("Failed to parse plan JSON: {}", err),
        )
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
        // arrange
        let json = valid_plan_json();

        // act
        let result = parse_course_plan(&json);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Rust Fundamentals");
    }

    #[test]
    fn test_parse_plan_invalid_json_returns_error() {
        // arrange / act
        let result = parse_course_plan("not json");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_empty_title_returns_error() {
        // arrange
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

        // act
        let result = parse_course_plan(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_empty_sections_returns_error() {
        // arrange
        let json = serde_json::json!({
            "title": "Title",
            "subtitle": "desc",
            "sections": []
        })
        .to_string();

        // act
        let result = parse_course_plan(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_section_zero_qcm_returns_error() {
        // arrange
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

        // act
        let result = parse_course_plan(&json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_markdown_wrapped_returns_ok() {
        // arrange
        let json = format!(
            "```json\n{}\n```",
            valid_plan_json()
        );

        // act
        let result = parse_course_plan(&json);

        // assert
        assert!(result.is_ok());
    }
}
