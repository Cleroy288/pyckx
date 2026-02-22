//! Verification parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::VerificationAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::games::shared::types::{
    AnswerGrade, GradedAnswer,
};

/// Parse verification/grading response from AI
pub fn parse_verification_response(
    response: &str,
) -> Result<Vec<GradedAnswer>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: VerificationAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as verification: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let grades: Vec<GradedAnswer> = ai_response
        .grades
        .into_iter()
        .map(|g| GradedAnswer {
            question_id: g.question_id,
            grade: match g.grade.as_str() {
                "right" => AnswerGrade::Right,
                "medium" => AnswerGrade::Medium,
                _ => AnswerGrade::Error,
            },
            feedback: g.feedback,
        })
        .collect();

    Ok(grades)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_verification_valid_returns_grades() {
        // arrange
        let json = serde_json::json!({
            "grades": [{
                "question_id": "q1",
                "grade": "right",
                "feedback": "Correct!"
            }]
        })
        .to_string();

        // act
        let result = parse_verification_response(&json);

        // assert
        assert!(result.is_ok());
        let grades = result.unwrap();
        assert_eq!(grades.len(), 1);
        assert_eq!(grades[0].grade, AnswerGrade::Right);
    }

    #[test]
    fn test_parse_verification_invalid_json_returns_error() {
        // arrange / act
        let result = parse_verification_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_verification_grade_mapping_scenarios() {
        // arrange
        let cases = vec![
            ("right", AnswerGrade::Right),
            ("medium", AnswerGrade::Medium),
            ("error", AnswerGrade::Error),
            ("unknown", AnswerGrade::Error),
        ];

        for (grade_str, expected) in cases {
            let json = serde_json::json!({
                "grades": [{
                    "question_id": "q1",
                    "grade": grade_str,
                    "feedback": "F"
                }]
            })
            .to_string();

            // act
            let grades =
                parse_verification_response(&json).unwrap();

            // assert
            assert_eq!(
                grades[0].grade, expected,
                "Grade '{}' should map to {:?}",
                grade_str, expected
            );
        }
    }

    #[test]
    fn test_parse_verification_preserves_question_id() {
        // arrange
        let json = serde_json::json!({
            "grades": [{
                "question_id": "abc-123",
                "grade": "right",
                "feedback": "Good"
            }]
        })
        .to_string();

        // act
        let grades = parse_verification_response(&json).unwrap();

        // assert
        assert_eq!(grades[0].question_id, "abc-123");
    }
}
