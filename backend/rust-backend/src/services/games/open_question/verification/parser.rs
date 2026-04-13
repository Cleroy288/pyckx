//! Verification parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::VerificationAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::games::shared::types::{
    AnswerGrade, GradedAnswer,
};

/// Map a grade string to AnswerGrade (case-insensitive)
fn parse_grade(raw: &str) -> AnswerGrade {
    match raw.to_lowercase().as_str() {
        "right" => AnswerGrade::Right,
        "medium" => AnswerGrade::Medium,
        _ => AnswerGrade::Error,
    }
}

/// Parse verification/grading response from AI
pub fn parse_verification_response(
    response: &str,
) -> Result<Vec<GradedAnswer>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: VerificationAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
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
            grade: parse_grade(&g.grade),
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
        let grades =
            parse_verification_response(&json).unwrap();

        // assert
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
    fn test_parse_grade_case_insensitive() {
        // arrange
        let cases = vec![
            ("Right", AnswerGrade::Right),
            ("RIGHT", AnswerGrade::Right),
            ("Medium", AnswerGrade::Medium),
            ("MEDIUM", AnswerGrade::Medium),
            ("Error", AnswerGrade::Error),
            ("ERROR", AnswerGrade::Error),
        ];

        for (input, expected) in cases {
            // act
            let result = parse_grade(input);

            // assert
            assert_eq!(
                result, expected,
                "parse_grade('{}') mismatch",
                input
            );
        }
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

    #[test]
    fn test_parse_verification_empty_grades_returns_empty() {
        // arrange
        let json =
            serde_json::json!({"grades": []}).to_string();

        // act
        let result =
            parse_verification_response(&json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_verification_multiple_grades_returns_all()
    {
        // arrange
        let json = serde_json::json!({
            "grades": [
                {"question_id":"q1","grade":"right","feedback":"A"},
                {"question_id":"q2","grade":"medium","feedback":"B"},
                {"question_id":"q3","grade":"error","feedback":"C"}
            ]
        })
        .to_string();

        // act
        let grades =
            parse_verification_response(&json).unwrap();

        // assert
        assert_eq!(grades.len(), 3);
    }

    #[test]
    fn test_parse_verification_extra_text_before_json_returns_ok()
    {
        // arrange
        let json = serde_json::json!({
            "grades": [{
                "question_id": "q1",
                "grade": "right",
                "feedback": "Ok"
            }]
        })
        .to_string();
        let input = format!("Grading:\n{}", json);

        // act / assert
        parse_verification_response(&input).unwrap();
    }

    #[test]
    fn test_parse_verification_preserves_feedback() {
        // arrange
        let json = serde_json::json!({
            "grades": [{
                "question_id": "q1",
                "grade": "right",
                "feedback": "Well done!"
            }]
        })
        .to_string();

        // act
        let grades =
            parse_verification_response(&json).unwrap();

        // assert
        assert_eq!(grades[0].feedback, "Well done!");
    }
}
